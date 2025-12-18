import sys, deepl, time, os
from deepl.api_data import MultilingualGlossaryDictionaryEntries
from doc import Doc

TO_TRANSLATE = (
    {
        "src_lang": "DE",
        "dst_lang": "EN-GB",
        "file_names": (
            "ch00-cover-pages.typ",
            "ch01-content.typ",
            "ch1-general.typ",
            "ch2-installation.typ",
            "ch3-operation.typ",
            "ch4-configuration.typ",
            "ch5-ap1-troubleshooting.typ",
            "ch5-ap2-technical-data.typ",
            "ch5-ap3-menus.typ",
            "ch5-ap4-literature.typ",
        ),
        "headers_keep_small": (
            "a", "an", "and", "of", "in", "is", "the", "on", "from", "to", "for", "with",
        ),
        "glossary": {
            "Bremsklappen": "airbrakes",
            "Fahrwerk": "landing gear",
            "Fahrwerkes": "landing gear",
            "Geradeausflugmodus": "straight flight mode",
            "Haubenblitzer": "canopy flasher",
            "Kreisflugmodus": "circling mode",
            "Libelle": "inclinometer",
            "Polare": "polar",
            "Sollfahrt": "speed to fly",
            "Steigen": "climb rate",
            "Wölbklappen": "flaps",
        },
    },
)

class Language():
    def __init__(self, lang):
        self.__dict__.update(lang)

class Docs():
    def __init__(self, to_translate, dictionary=None):
        # convert dict to objects for nicer handling
        self._languages = []
        for lang in to_translate:
            self._languages.append(Language(lang))

        # read api key from file and instantiate deepl access lib
        with open("api.key") as f:
            api_key = f.read()
        self._deepl = deepl.DeepLClient(api_key)

        # Find and delete glossaries
        glossaries = self._deepl.list_multilingual_glossaries()
        for glossary in glossaries:
            self._deepl.delete_multilingual_glossary(glossary)

        # Add glossary
        dicts = []
        for lang in self._languages:
            dicts.append(MultilingualGlossaryDictionaryEntries(
                lang.src_lang, 
                lang.dst_lang, 
                lang.glossary,
            ))

        self._glossary = self._deepl.create_multilingual_glossary("glossary", dicts)
        print(self._glossary.glossary_id)

    def check(self):
        # Verify that the text fragments only contain supported commands
        for lang in self._languages:
            lang_key = f"{lang.src_lang}->{lang.dst_lang}"
            for file_name in lang.file_names:
                in_file = f"{lang.src_lang[:2].lower()}/{file_name}"   

                print(f"...checking {in_file}")
                with open(in_file) as f:
                    to_check = f.read()
                doc = Doc(to_check)

                if not doc.check_ok():
                    raise Exception("Check not ok")

    def translate(self):
        print("...translating")
        for lang in self._languages:
            #ToDo: add glossary

            # clean and create output folder
            in_dir = lang.src_lang[:2].lower()
            out_dir = lang.dst_lang[:2].lower()
            os.popen(f"rm -rf {out_dir}")
            os.popen(f"mkdir {out_dir}")

            # translate the documents and store them
            for file_name in lang.file_names:
                in_file = f"{in_dir}/{file_name}"               
                out_file = f"{out_dir}/{file_name}"

                with open(in_file) as f:
                    to_translate = f.read()
                doc = Doc(to_translate)

                for step in doc.translate(self._deepl, lang, self._glossary):
                    ready = int(step*20)
                    to_do = 20 - ready
                    print(f"\r[{'-'*ready}{' '*to_do}] {in_file} -> {out_file}", end='')

                print()
                out_str = doc.result()
                with open(out_file, "w") as f:
                    f.write(out_str)

if __name__ == "__main__":
    # Translation with deepl
    docs = Docs(TO_TRANSLATE)
    docs.check()
    docs.translate()
