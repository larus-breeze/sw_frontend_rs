// Import the generated firmware version
#import "version.typ": version

// Definition of some basic parameters
#let basic_format(body) = [
    #set page(
        paper: "a4",
        margin: (top: 2.5cm, bottom: 2.5cm),
    )

    #set text(
        font: "dejavu sans",
        size: 9.5pt,
    )

    #set table.hline(stroke: .6pt)
    #set table(
        stroke: none,
        inset: 1.5mm,
    )

    #set list(
        indent: 4mm,
    )

    #show heading.where(level: 1): it => {
        set text(
            size: 12pt,
            weight: 600,
        )
        v(2pt)
        it
        v(7pt)
    }

    #show heading.where(level: 2): it => {
        set text(
            size: 11pt,
            weight: 600,
        )
        v(2pt)
        it
        v(4pt)
    }

    #show heading.where(level: 3): it => {
        set text(
            size: 10pt,
            weight: 600,
        )
        v(2pt)
        it
        v(4pt)
    }

    #show heading.where(level: 4): it => {
        set text(
            weight: 600,
        )
        v(1pt)
        it
        v(2pt)
    }

    #show heading.where(level: 5): it => {
        set text(
            weight: 600,
        )
        v(1pt)
        it
        v(2pt)
    }

    #show link: set text(rgb("0029F5"))
    #show ref: set text(rgb("0029F5"))

    #body
]

// The content is displayed with a matching header/footer.
#let content_format(chpater_name, body) = [
    #set heading(
        numbering: "1.1",
        supplement: [#chpater_name],
    )

    #set page(
        header: context {
            set text(8pt)
            [LARUS vario display#h(1fr)#box(image("/img/Larus_Logo.jpg", width: 0.8cm))]
            line(length: 100%, stroke: 0.1mm)
        },
        footer: context {
            set text(8pt)
            line(length: 100%, stroke: 0.1mm)
            [
                #version
                #h(1fr)
                #counter(page).display(
                    "1 of 1",
                    both: true,
                )
                #h(1fr)
            ]
        },

    )
    #body
]


// This is a pseudo-function for automated translation. The function does nothing within typ. An external Python script evaluates this function and translates the content with Deepl. A few functions are allowed within such a section:
// - *bold*
// - #keep[do not translate this]
// - #link()[]
#let tr(body) = {
    [#body]
}

// Performs the same function as #tr[], except that after translation by the Python script, the upper/lower case is adjusted to the conventions for headings.
#let hr(body) = {
    [#body]
}

// Do not translate the body of this function
#let keep(body) = {
    [#body]
}

// This is a special function for displaying important notes together with a pictograph.
#let picnote(image_path, body) = {
    table( 
        columns: (2cm, 14cm),
        stroke: none,
        table.hline(),
        [
            #align(center)[
                #image(image_path, width: 1cm)
            ]
        ], 
        [#body],
        table.hline(),
    )
}

#let appendix_format(chapter_name, body) = [
    #set heading(
        numbering: "A.1",
        supplement: [#chapter_name],
    )
    #counter(heading).update(0)
    #body
]
