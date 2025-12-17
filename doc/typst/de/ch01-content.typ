#import "../manual.typ": *

#outline(
    target: heading.where(supplement: [#hr[Abschnitt]]), 
    title: [#hr[Inhaltsverzeichnis]],
    depth: 3,
)

#outline(
    target: heading.where(supplement: [#hr[Anhang]]), 
    title: [#hr[Anhang]],
)

#outline(
    title: [#hr[Tabellenverzeichnis]],
    target: figure.where(kind: table),    
)

#outline(
    title: [#hr[Abbildungsverzeichnis]],
    target: figure.where(kind: image),    
)
#pagebreak()
