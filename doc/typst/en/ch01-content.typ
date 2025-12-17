#import "../manual.typ": *

#outline(
    target: heading.where(supplement: [#hr[Section]]), 
    title: [#hr[Table of Contents]],
    depth: 3,
)

#outline(
    target: heading.where(supplement: [#hr[Appendix]]), 
    title: [#hr[Appendix]],
)

#outline(
    title: [#hr[List of Tables]],
    target: figure.where(kind: table),    
)

#outline(
    title: [#hr[List of Figures]],
    target: figure.where(kind: image),    
)
#pagebreak()
