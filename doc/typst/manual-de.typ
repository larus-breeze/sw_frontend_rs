#import "manual.typ": *
#set text(lang: "de",)

#show: basic_format
#include "de/ch00-cover-pages.typ"

// Apply content_format to all following chapters including appendix
#show: content_format([Abschnitt])[
  #include "de/ch01-content.typ"
  #include "de/ch1-general.typ"
  #include "de/ch2-installation.typ"
  #include "de/ch3-operation.typ"
  #include "de/ch4-configuration.typ"

  // Apply appendix_format to appendix chapters
  #show: appendix_format([Anhang])[
    #include "de/ch5-ap1-troubleshooting.typ"
    #include "de/ch5-ap2-technical-data.typ"
    #include "de/ch5-ap3-menus.typ"
    #include "de/ch5-ap4-literature.typ"
  ]
]