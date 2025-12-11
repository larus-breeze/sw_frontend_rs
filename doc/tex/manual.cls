%----------------------------------------------------------------------------------------
%	CLASS CONFIGURATION
%----------------------------------------------------------------------------------------

\NeedsTeXFormat{LaTeX2e}
\ProvidesClass{manual}[2025/12/01]

\usepackage{etoolbox} % Required for conditional logic and easily changing commands

\newtoggle{unnumberedsections} % Create toggle for a class option
\settoggle{unnumberedsections}{false} % Default value for the class option
\DeclareOption{unnumberedsections}{\settoggle{unnumberedsections}{true}} % Set the class option toggle if the class option was used in the template

\DeclareOption*{\PassOptionsToClass{\CurrentOption}{article}} % Pass through any extra options specified to the base class
\ProcessOptions\relax % Process class options

\LoadClass{article} % Load the base class

%----------------------------------------------------------------------------------------
%	REQUIRED PACKAGES AND MISC CONFIGURATIONS
%----------------------------------------------------------------------------------------

\usepackage{graphicx} % Required for including images
\graphicspath{{images/}{./}} % Specifies where to look for included images (trailing slash required)

\setlength{\parindent}{0pt} % Paragraph indentation
\setlength{\parskip}{10pt} % Vertical space between paragraphs

\usepackage[bottom, hang]{footmisc} % Force footnotes to the bottom of the page and to the left margin
\setlength{\footnotemargin}{6pt} % Horizontal space between the footnote marker and text

\usepackage[document]{ragged2e} % Left align all text in the document (i.e. have a ragged right margin)

\usepackage{lastpage} % Required to determine the total number of pages

\AtBeginEnvironment{quote}{\itshape\rmfamily} % Style quotations

\usepackage{appendix} % Required for the appendices section

\usepackage{float} % Allows more precisely positioning floats e.g. \begin{figure}[H]

\usepackage{siunitx}
\usepackage{setspace} % Für die Steuerung des Zeilenabstands innerhalb von Einträgen


%----------------------------------------------------------------------------------------
%	MARGINS
%----------------------------------------------------------------------------------------

\usepackage[
	top=3cm, % Top margin
	bottom=3cm, % Bottom margin
	left=2cm, % Left margin
	right=2cm, % Right margin
	footskip=1.5cm, % Space from the bottom margin to the baseline of the footer
	headsep=0.8cm, % Space from the top margin to the baseline of the header
	headheight=1cm, % Height of the header
	%showframe % Uncomment to show frames around the margins for debugging purposes
]{geometry}

%----------------------------------------------------------------------------------------
%	FONTS
%----------------------------------------------------------------------------------------

\usepackage{mathpazo} % Use Palatino for math

\usepackage[sfdefault]{plex-sans} % Use IBM Plex Sans as the sans font and make it the default
\usepackage{plex-serif} % Use IBM Plex Serif as the serif font
\usepackage{plex-mono} % Use IBM Plex Mono as the mono font

\newcommand{\textb}[1]{{\fontseries{b}\selectfont #1}} % Define a simple command for using the SemiBold weight

\usepackage{fontspec}
% ---------------------------------------------
% 2. DejaVu Sans als spezielle Serifenlos-Schrift
% ---------------------------------------------
% Wir definieren DejaVu Sans als \setsansfont.
% Dies ist der Standardbefehl für die serifenlose Schrift im Dokument.
% Damit wird \textsf{...} in DejaVu Sans gesetzt.
 \newfontfamily\menufont{DejaVu Sans}
%----------------------------------------------------------------------------------------
%	HEADERS AND FOOTERS
%----------------------------------------------------------------------------------------
\usepackage{fancyhdr} % Required for customizing headers and footers
\pagestyle{fancy} % Enable custom headers and footers

\renewcommand{\headrulewidth}{0.5pt} % Top horizontal rule thickness
\newcommand*\myheaderfooterfont{\normalfont}

\fancyhf{} % Clear default headers/footers

\fancyhead[L]{\reporttitle} % Left header
\fancyhead[C]{} % Center header
\fancyhead[R]{\rightheadercontent} % Right header

\fancyfoot[L]{} % Left footer
\fancyfoot[C]{\thepage~/ \pageref{LastPage}} % Center footer
\fancyfoot[R]{\version} % Right footer

%----------------------------------------------------------------------------------------
%	SECTIONS
%----------------------------------------------------------------------------------------

\usepackage{titlesec} % Required for modifying sections

\iftoggle{unnumberedsections}{ % Conditional logic for the unnumbered sections class options
	\setcounter{secnumdepth}{0} % Don't number sections at any level
}{
	\setcounter{secnumdepth}{3} % Number sections down to subsubsections
}

\titleformat
	{\section} % Section type being modified
	[block] % Section layout type, can be: hang, block, display, runin, leftmargin, rightmargin, drop, wrap, frame
	{\raggedright\large\bfseries} % Text formatting of the whole section, i.e. label and title
	{\thesection} % Section label (e.g. number) and its formatting
	{5pt} % Horizontal space between the section label and title
	{} % Code before the section title
	[] % Code after the section title

\titlespacing*{\section}{0pt}{\baselineskip}{4pt} % Spacing around section titles, the order is: left, before and after

%------------------------------------------------

\titleformat
	{\subsection} % Section type being modified
	[block] % Section layout type, can be: hang, block, display, runin, leftmargin, rightmargin, drop, wrap, frame
	{\raggedright\bfseries\normalsize} % Text formatting of the whole section, i.e. label and title
	{\thesubsection} % Section label (e.g. number) and its formatting
	{5pt} % Horizontal space between the section label and title
	{} % Code before the section title
	[] % Code after the section title

\titlespacing*{\subsection}{0pt}{0.75\baselineskip}{4pt} % Spacing around section titles, the order is: left, before and after

%------------------------------------------------

\titleformat
	{\subsubsection} % Section type being modified
	[block] % Section layout type, can be: hang, block, display, runin, leftmargin, rightmargin, drop, wrap, frame
	{\bfseries} % Text formatting of the whole section, i.e. label and title
	{\thesubsubsection} % Section label (e.g. number) and its formatting
	{5pt} % Horizontal space between the section label and title
	{} % Code before the section title
	[] % Code after the section title

\titlespacing*{\subsubsection}{0pt}{0.5\baselineskip}{0pt} % Spacing around section titles, the order is: left, before and after

%------------------------------------------------

\titleformat
	{\paragraph} % Section type being modified
	[block] % Section layout type, can be: hang, block, display, runin, leftmargin, rightmargin, drop, wrap, frame
	{\bfseries} % Text formatting of the whole section, i.e. label and title
	{} % Section label (e.g. number) and its formatting
	{0pt} % Horizontal space between the section label and title
	{} % Code before the section title
	[] % Code after the section title

\titlespacing*{\paragraph}{0pt}{0pt}{0pt} % Spacing around section titles, the order is: left, before and after

\titleformat
	{\subparagraph} % Section type being modified
	[block] % Section layout type, can be: hang, block, display, runin, leftmargin, rightmargin, drop, wrap, frame
	{\bfseries} % Text formatting of the whole section, i.e. label and title
	{} % Section label (e.g. number) and its formatting
	{0pt} % Horizontal space between the section label and title
	{} % Code before the section title
	[~~~~] % Code after the section title

\titlespacing*{\subparagraph}{0pt}{0pt}{0pt} % Spacing around section titles, the order is: left, before and after

%----------------------------------------------------------------------------------------
%	TABLE OF CONTENTS
%----------------------------------------------------------------------------------------

\usepackage{titletoc} % Required for manipulating the table of contents

\setcounter{tocdepth}{3} % Show entries in the table of contents down to subsections

\renewcommand{\contentsname}{Table of Contents} % Rename the table of contents section heading

\titlecontents{section} % Section type being modified
	[0pt] % Left indentation
	{\vspace{-2pt}} % Before code
	{\textb{\thecontentslabel}~~\textb} % Formatting of numbered sections of this type
	{\textb} % Formatting of numberless sections of this type
	{\titlerule*[6pt]{.}\textb{\contentspage}} % Formatting of the filler to the right of the heading and the page number
	[\vspace{2pt}] % After code

%------------------------------------------------

\titlecontents{subsection} % Section type being modified
	[0.5cm] % Left indentation
	{\vspace{-8pt}} % Before code
	{\textb{\thecontentslabel}~~} % Formatting of numbered sections of this type
	{} % Formatting of numberless sections of this type
	{\titlerule*[6pt]{.}\contentspage} % Formatting of the filler to the right of the heading and the page number
	[] % After code
	
%------------------------------------------------

\titlecontents{subsubsection} % Section type being modified
	[1.35cm] % Left indentation
	{\vspace{-4pt}} % Before code
	{\textb{\thecontentslabel}~~} % Formatting of numbered sections of this type
	{} % Formatting of numberless sections of this type
	{\titlerule*[6pt]{.}\contentspage} % Formatting of the filler to the right of the heading and the page number
	[] % After code

%----------------------------------------------------------------------------------------
%	COLORS
%----------------------------------------------------------------------------------------

\usepackage[svgnames]{xcolor} % Required for defining and using custom colors

%\definecolor{cyan}{RGB}{0, 255, 255} % Define custom colors in this format

%----------------------------------------------------------------------------------------
%	BIBLIOGRAPHY
%----------------------------------------------------------------------------------------

\usepackage[
	backend=biber, % Use the biber backend for compiling the bibliography
	citestyle=numeric, % In-text citation style
	bibstyle=numeric, % Bibliography style
]{biblatex}
\usepackage{csquotes}

%----------------------------------------------------------------------------------------
%	TABLES
%----------------------------------------------------------------------------------------

\usepackage{booktabs} % Required for better horizontal rules in tables

\usepackage{array} % Required for manipulating table columns
\renewcommand{\arraystretch}{1.25} % Increase the height of table rows

\newcolumntype{R}[1]{>{\raggedleft\arraybackslash}p{#1}} % Define a new right-aligned paragraph column type
\newcolumntype{L}[1]{>{\raggedright\arraybackslash}p{#1}} % Define a new left-aligned (no justification) paragraph column type
\newcolumntype{C}[1]{>{\centering\arraybackslash}p{#1}} % Define a new centered paragraph column type

%----------------------------------------------------------------------------------------
%	CAPTIONS
%----------------------------------------------------------------------------------------

\usepackage{caption} % Required for customizing captions

\captionsetup{skip=6pt} % Vertical whitespace between figures/tables and the caption (default is 10pt)
\captionsetup{font={bf,small}} % Define caption font style
\captionsetup{justification=raggedright, singlelinecheck=false} % Left-align captions

%----------------------------------------------------------------------------------------
%	LISTS
%----------------------------------------------------------------------------------------

\usepackage{enumitem} % Required for list customization

\setlist{itemsep=0pt, topsep=0pt} % Customize spacing around and inside lists

\renewcommand{\labelenumi}{\arabic{enumi}.} % Customize the first level numbered list characters
\renewcommand{\labelenumii}{\alph{enumii}.} % Customize the second level numbered list characters
\renewcommand{\labelenumiii}{\roman{enumiii}.} % Customize the third level numbered list characters

%----------------------------------------------------------------------------------------
%	LINKS
%----------------------------------------------------------------------------------------

\usepackage[
    colorlinks=true, % Sets all options to color, not boxes
    linkcolor=blue,  % Color for internal links
    citecolor=blue,  % Color for citations
    urlcolor=blue,   % Color for URLs
	linktoc=page,    % Only page numbers clickable
]{hyperref}

%----------------------------------------------------------------------------------------
%	CUSTOM COMMANDS
%----------------------------------------------------------------------------------------

\newcommand{\reporttitle}[1]{\renewcommand{\reporttitle}{#1}}
\newcommand{\reportsubtitle}[1]{\renewcommand{\reportsubtitle}{#1}}
\newcommand{\reportauthors}[1]{\renewcommand{\reportauthors}{#1}}
\newcommand{\reportdate}[1]{\renewcommand{\reportdate}{#1}}
\newcommand{\rightheadercontent}[1]{\renewcommand{\rightheadercontent}{#1}}

\newcommand{\picnote}[2]{
	\begin{table*}[ht!]
		\begin{tabular}{C{0.1\linewidth} L{0.775\linewidth}}
			\toprule
			\vspace{-0.4cm} \includegraphics[width=1cm]{{#1}} & 
			{#2} \\
			\bottomrule
		\end{tabular}
	\end{table*}
}
\newcommand{\tr}[1]{#1} % macro for translation for python script
\newcommand{\hr}[1]{#1} % macro for header translation for python script
