import sys, deepl, time
from to_translate import TO_TRANSLATE, TO_TEST

class Language():
    def __init__(self, lang):
        self.__dict__.update(lang)

class Translate():
    def __init__(self, to_translate, dictionary=None):
        self._languages = []
        for lang in to_translate:
            self._languages.append(Language(lang))

        with open("api.key") as f:
            api_key = f.read()
        self._deepl = deepl.DeepLClient(api_key)

    def check(self):
        for lang in self._languages:
            lang_key = f"{lang.src_lang}->{lang.dst_lang}"
            for file_name in lang.file_names:
                in_file = f"{lang.src_lang[:2].lower()}/{file_name}"               
                print(f"...checking {in_file}")
                with open(in_file) as f:
                    right = f.read()
                
                while True:
                    hr, left, found, right = self._find_tr(right)
                    if found is None:
                        break
                    if found.find("\\") >= 0:
                        print(f"Command inside translation string found '{found}'")
                        sys.exit(1)


    def go(self, test=False):
        for lang in self._languages:
            glossary = {}
            # add ignore tags to glossary
            for key in lang.glossary.keys():
                glossary[key] = f"<x>{lang.glossary[key]}</x>"

            for file_name in lang.file_names:
                in_file = f"{lang.src_lang[:2].lower()}/{file_name}"               
                out_file = f"{lang.dst_lang[:2].lower()}/{file_name}"               
                print(f"...translating {in_file} -> {out_file}")
                with open(in_file) as f:
                    right = f.read()
                
                out_str = ""
                while True:
                    hr, left, found, right = self._find_tr(right)
                    if found is None:
                        # Nothing found, translation of this chunk is finished
                        out_str += left
                        break
                    else:
                        # translate glossary terms
                        for replace_key in glossary.keys():
                            pos = found.find(replace_key)
                            found = found.replace(replace_key, glossary[replace_key])
                            if pos == 0:
                                found = found[0].upper() + found[1:]
                        
                        # do not translate during testing
                        if test:
                            tr = found
                        else:
                            # print('translate:', found)
                            tr = self._translate(lang, found)

                        # after translaten remove xml tags
                        tr = tr.replace('<x>', '').replace('</x>', ' ')
                        
                        # If header, then respect case sensitivity
                        if hr:
                            tr = self._header(lang, tr)

                        out_str = out_str + left + tr

                with open(out_file, "w") as f:
                    f.write(out_str)

    def _header(self, lang, found):
        if lang.headers_keep_small is None:
            return found

        for split_char in [" ", "/", "-"]:
            words = []
            for word in found.split(split_char):
                if word not in lang.headers_keep_small:
                    if len(word) > 0:
                        word = word[0].upper() + word[1:]
                words.append(word)
            found = split_char.join(words)

        return found

    def _find_tr(self, right):
        hr = False
        pos = right.find("\\tr{")
        pos2 = right.find("\\hr{")
        if pos <= 0 and pos2 <=0:
            return (False, right, None, "")

        if pos < 0:
            pos = pos2
            hr = True
        else:
            if pos2 >= 0 and pos2 < pos:
                pos = pos2
                hr = True

        found = ""
        level = 0
        pos += 4
        pos_begin = pos
        while True:
            c = right[pos]
            if c == '}':
                if level <= 0:
                    return (hr, right[:pos_begin], found, right[pos:])
                else:
                    level -= 1
            elif c == '{':
                level += 1

            found += c
            pos += 1

    def _translate(self, lang, in_str):
        while True:
            try:
                result = self._deepl.translate_text(
                    in_str, 
                    source_lang=lang.src_lang,
                    target_lang=lang.dst_lang,
                    split_sentences="nonewlines",
                    preserve_formatting=True,
                    model_type="prefer_quality_optimized",
                    tag_handling="xml",
                    ignore_tags="x"
                )
                break
            except:
                sleep_time = 20
                print(f"*** Error while translating {in_str[:20]}...")
                print(f"sleep for {sleep_time} seconds...")
                time.sleep(sleep_time)
                print("trying again...")
        return result.text

if __name__ == "__main__":
    if False:
        # Test
        translate = Translate(TO_TEST)
        translate.check()
        translate.go(test=True)
    else:
        # Translation with deepl
        translate = Translate(TO_TRANSLATE)
        translate.check()
        translate.go()
