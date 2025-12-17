#import "manual.typ": *
#set text(lang: "en",)

#show: basic_format
#include "en/ch00-cover-pages.typ"

// Apply content_format to all following chapters including appendix
#show: content_format([Section])[
  #include "en/ch01-content.typ"
  #include "en/ch1-general.typ"
  #include "en/ch2-installation.typ"
  #include "en/ch3-operation.typ"
  #include "en/ch4-configuration.typ"

  // Apply appendix_format to appendix chapters
  #show: appendix_format([Appendix])[
    #include "en/ch5-ap1-troubleshooting.typ"
    #include "en/ch5-ap2-technical-data.typ"
    #include "menus.typ"
  ]
]