import sys, deepl

TO_TRANSLATE = (
    {
        "src_lang": "DE",
        "dst_lang": "EN-GB",
        "file_names": (
            "ch00-cover-pages.tex",
            "ch01-general.tex",
            "ch02-installation.tex",
            "ch03-operation.tex",
            "ch04-configuration.tex",
        )
    },
)

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

        self._dictionary = dictionary


    def check(self):
        for lang in self._languages:
            for file_name in lang.file_names:
                in_file = f"{lang.src_lang[:2].lower()}/{file_name}"               
                print(f"...checking {in_file}")
                with open(in_file) as f:
                    right = f.read()
                
                while True:
                    left, found, right = self._find_tr(right)
                    if found is None:
                        break
                    if found.find("\\") >= 0:
                        print(f"Command inside translation string found '{found}'")
                        sys.exit(1)


    def go(self):
        for lang in self._languages:
            for file_name in lang.file_names:
                in_file = f"{lang.src_lang[:2].lower()}/{file_name}"               
                out_file = f"{lang.dst_lang[:2].lower()}/{file_name}"               
                print(f"...translating {in_file} -> {out_file}")
                with open(in_file) as f:
                    right = f.read()
                
                out_str = ""
                while True:
                    left, found, right = self._find_tr(right)
                    if found is None:
                        out_str += left
                        break
                    else:
                        tr = self._translate(lang, found)
                        out_str = out_str + left + tr

                with open(out_file, "w") as f:
                    f.write(out_str)

    def _find_tr(self, right):
        pos = right.find("\\tr{")
        if pos<0:
            return (right, None, "")

        found = ""
        level = 0
        pos += 4
        pos_begin = pos
        while True:
            c = right[pos]
            if c == '}':
                if level <= 0:
                    return (right[:pos_begin], found, right[pos:])
                else:
                    level -= 1
            elif c == '{':
                level += 1

            found += c
            pos += 1

    def _translate(self, lang, in_str):
        result = self._deepl.translate_text(
            in_str, 
            source_lang=lang.src_lang,
            target_lang=lang.dst_lang,
            split_sentences="nonewlines",
            preserve_formatting=True,
            model_type="prefer_quality_optimized",
        )
        return result.text

if __name__ == "__main__":
    translate = Translate(TO_TRANSLATE)
    translate.check()
    translate.go()