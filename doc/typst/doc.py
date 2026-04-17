import re, time
# This class is used for fragments that are not translated.
class NoFunction():
    def __init__(self, raw):
        self._raw = raw

    @property
    def raw(self):
        return self._raw

    def add_to_str(self, in_str, raw=False):
        return in_str + self._raw

    def check_ok(self):
        return True

    def translate(self, deepl, lang, glossary):
        pass

# This is for fragments for translation
class Function(NoFunction):
    def __init__(self, raw):
        super().__init__(raw)

        pos = raw.find('[')
        pos2 = raw.rfind(']')
        if pos > 0 and pos2 > pos:
            self._content = raw[pos+1:pos2]
            self._name = raw[1:pos]
        else:
            raise Exception("Invalid input")

        self._gaps = {}
        counter = 1

        for key_word in ("#keep", "#link"):
            while True:
                pos = self._content.find(key_word)
                if pos < 0:
                    break

                pos2 = self._content.find(']', pos) + 1
                value = self._content[pos:pos2]
                key = f"@@{counter:03d}"
                self._gaps[key] = value
                self._content = self._content[:pos] + key + self._content[pos2:]
                counter += 1

    def add_to_str(self, in_str, raw=False):
        if raw:
            return f"{in_str}#{self._name}[{self._content}]"
        else:
            content = self._content
            for key in self._gaps.keys():
                content = content.replace(key, self._gaps[key])
            return f"{in_str}#{self._name}[{content}]"

    def check_ok(self):
        hash_ok = True
        for pos in [m.start() for m in re.finditer('#', self._content)]:
            if pos > 0 and self._content[pos-1] == '\\':
                continue
            hash_ok = False
            print(f"Command in translation string found")
            print(self._content)

        text_ok = self._raw == self.add_to_str("")
        if not text_ok:
            print(f"Rebuild text is not identical")
            print(self._raw)
            print(self.add_to_str(""))

        return hash_ok and text_ok

    def translate(self, deepl, lang, glossary):
        while True:
            try:
                # Translate with deepl
                result = deepl.translate_text(
                    self._content, 
                    source_lang=lang.src_lang,
                    target_lang=lang.dst_lang,
                    split_sentences="nonewlines",
                    preserve_formatting=True,
                    model_type="prefer_quality_optimized",
                    tag_handling="xml",
                    ignore_tags="x",
                    glossary=glossary
                )
                self._content = result.text
                break
            except:
                sleep_time = 20
                print(f"*** Error while translating {self._content[:20]}...")
                print(f"sleep for {sleep_time} seconds...")
                time.sleep(sleep_time)
                print("trying again...")

        if lang.headers_keep_small is None:
            return found

        # If header, adjust upper and lowercase letters
        if self._name == "hr":
            for split_char in [" ", "/", "-"]:
                words = []
                for word in self._content.split(split_char):
                    if word not in lang.headers_keep_small:
                        if len(word) > 0:
                            word = word[0].upper() + word[1:]
                    words.append(word)
                self._content = split_char.join(words)


    @property
    def name(self):
        return self._name

    @property
    def args(self):
        return self._args

    @property
    def content(self):
        return self._content


    def __repr__(self):
        return f"<{self._name}, '{self._content}'>"


# Helper class to get an iterator for the progress bar
class DocIter():
    def __init__(self, children, deepl, lang, glossary):
        self._children = children
        self._deepl = deepl
        self._lang = lang
        self._glossary = glossary
        self._idx = 0

    def __iter__(self):
        self._idx = 0
        return self

    def __next__(self):
        if self._idx >= len(self._children):
            raise StopIteration
        else:
            self._children[self._idx].translate(self._deepl, self._lang, self._glossary)
            self._idx += 1
            return self._idx / len(self._children)

# The doc class holds the complete document
class Doc():
    def __init__(self, in_str):
        self._children = []
        left = in_str        

        while True:
            left, found, right = self.filter(left)
            if left is not None:
                self._children.append(NoFunction(left))
            if found is not None:
                self._children.append(Function(found))
            if right is None:
                break
            else:
                left = right

    def result(self, raw=False):
        result = ""
        for child in self._children:
            result = child.add_to_str(result, raw)
        return result

    def check_ok(self):
        r = True
        for child in self._children:
            if not child.check_ok():
                r = False
        return r

    def translate(self, deepl, lang, glossary):
        return iter(DocIter(self._children, deepl, lang, glossary))

    @staticmethod
    def filter(in_str):
        left = None
        found = None
        right = None

        pos = 0
        while True:
            p = 1e10
            for f_name in ("#tr[", "#hr["):
                x = in_str.find(f_name, pos)
                if x >= 0 and x < p:
                    p = x

            if p != 1e10:
                pos = p
            else:
                # nothing found
                left = in_str
                break

            pos2 = in_str.find("[", pos)
            level = 1
            while level > 0:
                pos2 += 1
                if pos2 >= len(in_str):
                    # this is not allowed
                    raise Exception("No ] found")

                if in_str[pos2] == '[':
                    level += 1

                if in_str[pos2] == ']':
                    level -= 1
                    pos2 += 1
                    if level == 0:
                        left = in_str[0:pos]
                        found = in_str[pos:pos2]
                        right = in_str[pos2:]
            
            if found is not None:
                break

        return (left, found, right)

if __name__ == "__main__":
    def assert_exception(in_str, e_str):
        try:
            Doc.filter(in_str)
        except Exception as e:
            assert str(e) == e_str

    assert Doc.filter("dies ist ein test")                              == ('dies ist ein test', None, None)
    assert Doc.filter("\\#[dies] ist ein test")                         == ('\\#[dies] ist ein test', None, None)
    assert Doc.filter("#tr[dies ist ein test]")                         == ('', '#tr[dies ist ein test]', '')
    assert Doc.filter("dies \\#[ist] ein test")                         == ('dies \\#[ist] ein test', None, None)
    assert Doc.filter("#tr[dies #keep[ist] ein test]")                  == ('', '#tr[dies #keep[ist] ein test]', '')
    assert Doc.filter("#tr[dies ist ein test], der zeigen soll...")     == ('', '#tr[dies ist ein test]', ', der zeigen soll...')
    assert Doc.filter("dies ist #tr[ein test], der zeigen soll...")     == ('dies ist ', '#tr[ein test]', ', der zeigen soll...')
    assert_exception("#trdies #keep[ist] ein test",                 "Function name contains invalid chars")
    assert_exception("#tr[dies #keep[ist] ein test",                "No ] found")
    
    assert repr(Function("#tr[test]")) == "<tr, 'test'>"

    assert Doc("dies ist ein Test").result()                            == "dies ist ein Test"
    assert Doc("#tr[dies ist ein Test]").result(True)                   == "#tr[dies ist ein Test]"
    assert Doc("#tr[dies #keep[ist ein] Test]").result(True)            == "#tr[dies @@1 Test]"
    assert Doc("#tr[dies #keep[ist ein] Test]").result(False)           == "#tr[dies #keep[ist ein] Test]"
    assert Doc("#tr[dies #keep[ist ein] Test, der zeigen #keep[soll], dass es auch #links(von hier)[nach da] gibt]").result(True)   == "#tr[dies @@1 Test, der zeigen @@2, dass es auch @@3 gibt]"
    assert Doc("#tr[dies #keep[ist ein] Test, der zeigen #keep[soll], dass es auch #links(von hier)[nach da] gibt]").result()       == "#tr[dies #keep[ist ein] Test, der zeigen #keep[soll], dass es auch #links(von hier)[nach da] gibt]"
    assert Doc("#tr[dies ist ein] Test, #hr[der zeigen] soll, #nixda[dass] es auch #tr[links] gibt]").result()                      == "#tr[dies ist ein] Test, #hr[der zeigen] soll, #nixda[dass] es auch #tr[links] gibt]"
    assert Doc("#tr[dies \\#ist ein Test]").check_ok()            == True
    assert Doc("#tr[dies #ist \\ein Test]").check_ok()            == False
