# ANSI X3.53-1976 (R1998) - Full Programming Language PL/I

> **Role:** Boundary of full standard PL/I (NOT the implementation target). Use to mark out-of-scope vs Subset G.
> **Source file:** `!Жопа с книжками\ANSI-X3_53-1976_R1998.PDF`
> **Pages:** 421
> **Extraction:** pypdf text layer (not re-OCR)

---


## Page 1

ANSI INCITS 53-1976 (R1998)
(formerly ANSI X3.53-1976 (R1998))
Programming Language
PL/I
adopted by the
DEPARTMENT
OF
DEFENSE
see acceptance notice on
inside of standard
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 2

DOD ANSI X3.53-1976
14 October 1976
ACCEPTANCE NOTICE
The above Industry Standardization Document was adopted on 14 October 1976 and is approved for voluntary use by the
Departments of the Army, Navy, and Air Force The indicated industry groups have forwarded the clearances required by
existing regulations. Copies of the document are stocked by DOD Single Stock Point, U S Naval Publications and Forms
Center, Philadelphia, Pennsylvania, for issue to DOD activities only.
Title of Document: Programming Language PLll
Date of Specific Issue Adaopted: 9 August 1976
Releasing Industry Group: American National Standards Institute
Custodians: Military Coordinating Activity:
Army -AD Air Force - 02
Navy - OM
Air Force - 02 Project Number IPSC6075
IS0
This standard has been adopted as IS0 International Standard IS0 6160-1979. IS0 (the International Organi-
zation for Standardization) is a worldwide federation of national standards institutes (IS0 member bodies).
The work of developing International Standards is carried out through IS0 technical committees. Every mem-
ber body interested in a subjectforwhich a technical committee has been set up has the right to be represented
on that committee. International organizations, governmental and nongovernmental, in liaison with ISO, also
take part in the work.
Draft International Standards adopted by the technical committees are circulated to the member bodies for
approval before their acceptance as International Standards by the IS0 Council.
International Standard IS0 6160 was developed by Technical Committee lSO/TC 97, Computers and informa-
tion processing, and was circulated to the member bodies in May 1978.
It has been approved by the member bodies of the following countries:
Belgium Hungary New Zealand Sweden
Canada Italy Poland Switzerland
Czechoslovakia Japan Romania United Kingdom
France Mexico South Africa, Rep. of USA
Germany, F. R. Netherlands Spain USSR
No member body expressed disapproval of the document.
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 3

INTERNATIONAL STANDARD
(NTERNATIONAL ORGAN,ZATtON FOR STANDARDIZATIOUUE*nlHA~IIHAR O“TA”H,A”HR “0 CTA~(nAPTH3AUH~RGANlSATlON INTERNATIONALE DE NORYALISATION
Foreword
IS0 (the International Organization for Standardization) is a worldwide fed-
eration of national standards institutes (IS0 member bodies). The work of
developing International Standards is carried out through IS0 technical
committees. Every member body interested in a subject for which a technical
committee has been set up has the right to be represented on that committee.
International organizations, governmental and non-governmental, in liaison
with ISO, also take part in the work.
Draft International Standards adopted by the technical committees are circu-
lated to the member bodies for approval before their acceptance as Interna-
tional Standards by the IS0 Council.
International Standard IS0 6160 was developed by Technical Committee ISO/
TC 97, Computers and information processing, and was circulated to the
member bodies in May 1978.
It has been approved by the member bodies of the following countries:
Belgium Japan
Canada Mexico
Czechoslovakia Netherlands
France New Zealand
Germany, F.R. Poland
Hungary Romania
Italy South Africa, Rep. of
Spain
Sweden
Switzerland
United Kingdom
USA
USSR
No member body expressed disapproval of the document.
Programming languages - PLII
The following national standard is adopted as IS0 International Standard 6160-l 979:
- ANSI standard X3.53-l 976.
NOTE - Copies of the ANSI standard may be obtained through the American National Standards Institute, 1430
Broadway, New York, N Y. 10018, U.S.A. The corresponding French text will be established by the Assocation
francaise de normalisation, Tour Europe, Cedex 7, 92080 Paris La Defense, France
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 4

ANSI @
X3.53- 1976
American National Standard
Programming Language
PWI
Secretariat
Computer and Business Equipment Manufacturers Association
Approved August 9, 1976
American National Standards Institute, Inc
Abstract
This document defines American National Standard Programming Language PL/I and specifies
both the form and interpretation of computer programs written in PL/I. The standard is intended
to provide a high degree of machine independence and thereby facilitate program exchange among
a variety of computing systems. The document serves as an authoritative reference rather than as
a tutorial exposition. The language is defined by specifying a conceptual PL/I machine which
translates and interprets intended PL/I programs. The relationship between an actual implementa-
tion of PLA and the conceptual machine presented in this document is also given. This reference
document was developed jointly under the auspices of the American National Standards Institute
and the European Computer Manufacturers Association.
Indexing terms: PL/I, computer programming language, computer programming language def-
inition, formal languages.
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 5

American
National
Standard
Approval of an American National Standard requires review by ANSI that the
requirements for due process, consensus, and other criteria for approval have
been met by the standards developer.
Consensus is established when, in the judgment of the ANSI Board of Standards
Review, substantial agreement has been reached by directly and materially
affected interests.  Substantial agreement means much more than a simple
majority, but not necessarily unanimity.  Consensus requires that all views and
objections be considered, and that a concerted effort be made toward their
resolution.
The use of American National Standards is completely voluntary; their existence
does not in any respect preclude anyone, whether he has approved the standards
or not, from manufacturing, marketing, purchasing, or using products, processes,
or procedures not conforming to the standards.
The American National Standards Institute does not develop standards and will in
no circumstances give an interpretation of any American National Standard.
Moreover, no person shall have the right or authority to issue an interpretation of
an American National Standard in the name of the American National Standards
Institute.  Requests for interpretations should be addressed to the secretariat or
sponsor whose name appears on the title page of this standard.
CAUTION NOTICE: This American National Standard may be revised or
withdrawn at any time. The procedures of the American National Standards
Institute require that action be taken periodically to reaffirm, revise, or withdraw
this standard.  Purchasers of American National Standards may receive current
information on all standards by calling or writing the American National Standards
Institute.
Published by
American National Standards Institute
11 West 42nd Street, New York, New York 10036
Copyright 1976 by Information Technology Industry Council (ITI)
All rights reserved.
No part of this publication may be reproduced in any
form, in an electronic retrieval system or otherwise,
without prior written permission of ITI, 1250 Eye Street NW,
Washington, DC  20005.
Printed in the United States of America
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 6

Foreword (This Foreword is not a part of American National Standard Programming Language PL/I, X3.53-1976.)
In October 1963, a committee was formed by the SHARE FORTRAN Project and the Intema-
tional Business Machines Corporation to extend FORTRAN, then the primary high-level language
used by scientific programmers. The extended language was to be suitable for commercial and
systems programs as well and was to take into account the architecture of modern computers.
The committee soon determined that the language would have to differ from FORTRAN, and
their report, presented in April 1964, was entitled “Specifications for the New Programming
Language.”
The New Programming Language was widely discussed. Revised descriptions were issued, and in
1965 it was renamed PL/I (for programming language one). Versions were implemented on the
computers of at least two manufacturers by late 1966.
By April 1966, Technical Committee 10 (TC 10) had been set up by the European Computer
Manufacturers Association (ECMA) and an ad hoc group by American National Standards Com-
mittee X3 to investigate the development of PL/I standards. TC 10 embarked immediately on
standardization but the Standards Institute group considered first whether PL/I was a suitable
candidate. Early in 1969 Technical Committee X351 was formed by American National Stan-
dards Committee X3 to carry out the work in conjunction with TC 10.
The two technical committees, X3Jl and TC 10, formed the joint PL/I standardization project.
Building on the work begun by TC 10, nearly 3500 proposals for language or textual change
were processed in developing a joint draft standard for PL/I. The joint working document under-
went thirteen complete revisions. During this time, the definition evolved from an imprecise
English description to a precise specification in a semiformal metalanguage using a stylized
English.
The Joint Project has enjoyed excellent liaison with potential users and with other standards
organizations such as International Organization for Standardization Technical Committee 97,
Subcommittee 5 (ISO/TC 97/SC 5) and PL/I working groups in Austria, France, Germany, the
Netherlands, Japan, and the United Kingdom.
Suggestions for improvement of this standard will be welcome. They should be sent to the
American National Standards Institute, 1430 Broadway, New York, N.Y. 10018.
This standard was processed and approved for submittal to ANSI by American National Stan-
dards Committee on Computers and Information Processing, X3. Committee approval of the
standard does not necessarily imply that all committee members voted for its approval. At the
time it approved this standard, the X3 Committee had the following members:
J. F. Auwaerter, Chairman
R. M. Brown, Vice-Chairman
W. F. Hanrahan, Secretary
Organization Represented Name of Representative
Addressograph Multigraph Corporation . . . . . . . . . . . . . . . . . . . . . . . . . . A. C. Brown
D. Anderson (Ah)
Air Transport Association . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . F. C. White
C. Hart (Alt)
American Library Association . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . J. R. Rizzolo
J. C. Kountz (Ah)
M. S. Malinconico (Alt)
American Nuclear Society . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . D. R. Vondy
M. K. Butler (Alt)
American Society for Information Science. . , . . , . . . . . . . . . . . . . . . . . . . M. C. Kepplinger
Association for Computing Machinery . . . . . . . . . . . . . . . . . . . . . . . . . . . P. Skelly
J. A. N. Lee (Ah)
H. E. Thiess (Ah)
Association for Educational Data Systems. . . . . . . . . . . . . . . . . . . . . . . . . A. K. Swanson
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 7

Organization Represented Name of Representative
Association of American Railroads . . . . . . . . . . . . . . . . . . . . . . . . . . . . . R. A. Petrash
Association of Computer Programmers and Analysts . . . . . . . . . . . . . . . . . . . L. A. Ruh
M. A. Morris, Jr (Alt)
Association of Data Processing Service Organizations. . . . . . . . . . . . . . . . . . . J. B. Christiansen
Burroughs Corporation . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . E. Lohse
J. S. Foley (Alt)
J. F. Kalbach (Alt)
California Computer Products, Inc . . . . . . . . . . . . . . . . . . . . . . . . . . . . . R. C. Derby
Computer Industry Association . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . N. J. Ream
A. G. W. Biddle (Alt)
Control Data Corporation . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . C. E. Cooper
G. I. Williams (Alt)
Datapoint Corporation . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . V. D. Poor
H. W. Swanson (Alt)
Data Processing Management Association . . . . . . . . . . . . . . . . . . . . . . . . . A. E. Dubnow
D. W. Sanford (Alt) .
Digital Equipment Corporation . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . P. White
A. R. Kent (Alt)
Edison Electric Institute . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . S. P. Shrivastava
J. i. Weiser (Alt)
General Services Administration. . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . D. L. Shoemaker
M. L. Burris (Alt)
GUIDE International . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . T. E. Wiese
B. R. Nelson (Alt)
D. Stanford (Alt)
Honevwell Information Svstems. Inc . . . . . . . . . . . . . . . . . . . . . _ . . . . . . T. J. McNamara
E. H. Clamons (Alt)
Institute of Electrical and Electronics Engineers, Communications Society . . . . . . (Representation Vacant)
Institute of Electrical and Electronics Engineers, Computer Society . . . . . . . . . . R. L. Curtis
T. Feng (Ah)
International Business Machines Corporation . . . . .................. L. Robinson
W. F. McClelland (Alt)
ITEL Corporation. . . . . . . . . . . . . . . . . . . . ................. R. W. Whitcomb
J. D. Bock (Alt)
Joint Users Group. . . . . . . . . . . . . . . . . . . . .................. T. E. Wiese
L. Rodgers (Alt)
Life Office Management Association . . . . . . . . . .................. R. Ricketts
J. F. Foley (Alt)
Litton Industries . . . . . . . . . . . . . . . . . . . . .................. I. Danowitz
National Association of State Information Systems. .................. G. H. Roehm
J. L. Lewis (Alt)
G. I. Theis (Alt)
National Bureau of Standards .............. H. S. White, Jr
R. E. Rountree (Alt)
National Communications System ............................. M. L. Cain
G. W. White (Alt)
National Machine Tool Builders’ Association ....................... 0. A. Rodriques
NCR Corporation ....................................... R. J. Mindlin
A. R. Daniels (Alt)
T. W. Kern (Alt)
Olivetti Corporation of America. .............................. E. J. Almquist
Printing Industries of America ................................ N. Scharpf
E. Masten (Alt)
Recognition Equipment, Inc ................................ H. F. Schantz
W. E. Viering (Alt)
Sanders Associates, Inc .................................. A. L. Goldstein
T. H. Buchert (Alt)
Scientific Apparatus Makers Association .......................... A. Savitsky
J. E. French (Alt)
SHAREInc .......................................... T.B.Steel,Jr
E. Brubaker (Alt)
Society of Certified Data Processors ............................ T. M. Kurihara
A. E. Dubnow (Alt)
. UNIVAC, Division of Sperry Rand Corporation. . . . . . . . . . . . . . . . . . . . . . M. W. Bass
C. D. Card (Alt)
Telephone C.mun . _ _ _ . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . ._ -__-= . . . . . V. N. Vaughan, Jr
S. M. Garland (Alt)
J. C. Nelson (Al0
3MCompany . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . R. C. Smith
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 8

Organization Represented Name of Representative
U.S. Department of Defense . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . W. L. McGreer
W. B. Rinehuls (Alt)
W. B. Robertson (Ak)
. . . . . . . . . . W. R. McPherson, Jr
M. A. Evans (Alt)
. . . . . . . . . . S. W. White
M. R. Speers (Alt)
. . . . . . . . . . J. L. Wheeler
A. R. Machell (Ah)
U.S. Department of Health, Education and Welfare. ......
VIM (CDC 6000 Users Group). ..................
Xerox Corporation .........................
Technical Committees X3J1 and TC 10 had the following members at the time they forwarded
this standard to their parent committees.
L. C. Frampton, Chairwoman, X3 Jl P. W. Abrahams
R. L. Wexelblat, Vice-Chairman, X3J 1 M. E. Adams
J. P. Kuoni, Vice-Chairman, X3Jl A. J. Arthur
M. E. Barber
J. Bourgain, Chairman, TC 10
B. Kahan, Vice-Chairman, TC 10
D. Hekimi, Secretary, TC 10
D. Beech
P. Belmont
E. Boerger
D. R. Doucette
B. Folsom
A. C. Franklin
R. A. Freiburghouse
G. R. Greenacre
J. C. Klensin
D. J. Andrews, Editor
R. R. Knight
H. Krainer
L. Y. Lee
M. D. MacLaren
M. Marcotty
(Former Chairman, X3Jl)
J. D. Mills
E. Hogan Northam
W. Schoniger
W. F. Sims
K. Tweedy
S. K. Wilson
Others who contributed to the development of this standard are the following:
A. Astierks
M. A. Batey
(Former Editor)
J. Besse
(Former Secretary, TC 10)
J. L. Callahan
R. C. Dennis
E. Fontanel
A. G. Grace
W. A. Haas
C. A. R. Hoare
(Former Chairman, TC 10)
C. G. Keane
H. Keilhau
L. Korneva
A. H. Kvilekval
P. Lindes
G. A. Logemann
K. J. LUM
(Former Editor)
J. Marshall
J. M. Mazola
L. D. Mulli6
G. Oakes
J. C. Pao
D. F. Purcell
J. Robb
A. H. Rogers
A. J. Roskell
R. Rowe
F. Schneider
A. Schwald
R. A. Sharman
J. A. Taylor
F. Welsh
J. Witt
(Former Chairman, TC 10)
L. Zmievskaja
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 9

Contents
CHAPTER 1: SCOPE AND OVERVIEW:, . . . .
1.0 Scope . . . . . . . . .
1.1 An Informal Guide to the PL/I Definition
1.1.1 A Summary of PL/I . . . . .
1.1.2 The Form of the Definition . . .
1.1.3 Summary of Chapter Structure . .
1.1.4 Introduction to the Metalanguage .
1.1.4.1 Tree Concepts . . . . .
1.1.4.2 Syntaxes . . . . . .
1.1.4.3 Algorittim Concepts . . . .
1.2 Relationships between an implementation and
. . . . .
. . . . .
. . . . .
. . . . .
. . . . .
. . . . .
. . . . .
. . . . .
. . . . .
. . . . .
This Definition .
1.2.1 Flexibilities of Interpretation ....
1.2.1-l Rejection of Prograns ......
1.2.1.2 Guantitative Restrictions .....
1.2.1.3 Operating Environment ......
1.2.1.4 Expression Evaluation ......
1.2.1.5 Interrupts and Assignment .....
1.2.1.6 Input/Output ........
1.2.1.7 On-units .........
1.2.2 Implementation-defined keatures ....
1.3 The Metalanguage .........
1.3.1 Trees ...........
1.3.1.1 Tree Definitions .......
1.3.1.2 Node Objects ........
1.3.1.2.1 Unique-names .......
1.3.1.2.2 Types .........
1.3.1.3 Node Notation ........
1.3.1.4 Tree Notation ........
1.3.1.4.1 Enumerated Trees ......
1.3.1.4.2 Forms .........
1.3.1.5 Tree Copies ........
1.3.2 Production Rules ........
1.3.2.1 Production Rules and Syntaxes ....
1.3.2.2 Complete and Partial Trees .....
1.3.2.3 Svntactic-expressions and Syntactic-units .
1.3.2.4 Application of the Production Rules .
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
1.3.3 Operations . . . . . . .
1.3.3.1 Nature of an Operation . . .
1.3.3.2 Nondeterministic Operations .
1.3.3.3 Format of an Operation . . .
1.3.3.4 Instructions . . . . .
1.3.3.5 Convert . . . . . .
1.3.3.6 Additional Notational Conventions
1.3.3.7 Arithmetic . . . . . .
1.3.4 The Processor . . . . . .
1.3.5 Mechanization of the Metalanguage .
1.4 Initialization of the Machine-state . .
1.4-l The Machine-state . . . . .
1.4.2 Initialization . . . . . .
1.4.3 The Top-level Operations . . .
1.4.3.1 Define-program . . . . .
1.4.3.2 Translation-phase . . . .
1.4.3.3 Interpretation-phase . . .
CHAPTER 2: CGNCRETE SYNTAX . . . . .
2.0 Introduction . . . . . . .
2.1 The Intent of this Definition . . .
2.1.1 Concrete and Abstract Syntaxes . .
2.2 Organization of the Concrete Syntax . .
2.3 The High-level Syntax of PL/I . . .
2.3.1 Procedure . . . . . . .
2.3.2 Unit . . . . . . . .
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
Contents
1
1
1
1
2
3
7
7
8
3
11
11
11
12
12
12
12
13
14
14
16
16
17
18
18
18
13
20
20
20
21
21
21
22
23
23
24
25
25
25
27
28
28
28
29
25
31
31
31
31
31
32
32
33
33
33
33
33
33
33
33
1
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 10

2.3.3 Executable Units
2.4 The Middle-level Syntax if PVI ...........
2.4.1 Sentence .........
2.4.2 Statement .........
2.4.3 Prefixes .........
2.4.3.1 Condition Prefixes ......
2.4.3.2 Statement Name Prefixes ....
2.4.4 Data Declaration .......
2.4.4.1 Dimension Attribute and Dimension Suffix
2.4.4.2 Attributes . . . . . .
2.4.4.3 Data Attributes . . . .
2.4.4.4 Environment and Options . .
2.4.4.5 Generic . . . . . .
2.4.4.6 Initial . . . . . .
2.4.4.7 The Default Statement . . .
2.4.5 The Procedure Statement . . .
2.4.6 The Entry Statement . . . .
2.4.7 The Begin Statement . . . .
2.4.8 The Do Statement . . . . .
2.4.9 The End Statement . . . . .
2.4.10 Flow of Control Statements . . .
2.4.10.1 The Call and Return Statements .
2.4.10.2 The Go To Statement . . .
2.4.10.3 The Null Statement . . . .
2.4.10.4 The Revert and Signal Statements
2.4.10.5 The Stop Statement . . . .
2.4.11 Storage Control Statements . . .
2.4.12 Input/Output Statements . . .
2.4.12.1 The Open and Close Statements .
2.4.12.2 Record I/O . . . . . .
2.4.12.3 Stream I/O . . . . . .
2.4.12.3.1 Stream Input Specitication .
2.4.12.3.2 Stream Output Specification
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
2.4.12.3.3 Format Snecification Lists and the Format Statement
2.4.13 Expressions . . . . . ........
2.5 The Low-level Syntax of PL/I . ........
2.5.1 PL/I Text . . . . . ........
2.5.2 Comment . . . . . ........
2.5.3 Identifier . . . . . ........
2.5.4 Arithmetic Constant . . ........
2.5.5 String Constants and Pictures ........
2.5.6 Isub . . . . . . ........
2.5.7 Include . . . . . ........
2.6 Character Sets . . . . . ........
2.6.1 Language Character Set . . ........
2.6.1.1 Letters and Digits . . ........
2.6.1.2 Special Characters . . ........
2.6.2 Data Character Set . . . ........
2.7 Abbreviations . . . . . ........
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
34
35
35
36
36
36
36
36
37
37
37
38
38
38
39
39
39
39
40
40
40
40
40
40
40
40
41
41
41
41
42
42
43
43
44
47
47
47
47
47
48
48
48
48
48
49
49
49
50
CBAPTER 3: ABSTRACTSYNTAX ............. 51
3.0 Introduction ............... 51
3.1 Abstract Syntax Rules ............. 51
3.1.1 Program ............... 51
3.1.2 Procedure ............... 51
3.1.3 Declaration .............. 51
3.1.4 Variable ............... 52
3.1.5 Data-description ............. 52
3.1.6 Data-type ............... 53
3.1.7 Named-constant .............. 53
3.1.8 Entry-or-executable-unit ........... 54
3.1.9 Begin-block .............. 54
3.1.10 Groups ................ 54
3.1.11 On Statement .............. 54
3.1.12 If Statement .............. 55
3.1.13 Flow of Control Statements ........... 55
3.1.14 Storage Statements ............. 55
3.1.15 I/O Statements .............. 56
3.1.16 Record I/O Statements ............ 56
3.1.17 Stream I/O Statements ............ 56
2
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 11

3.1.18 Expression . . . . . . .
3.1.19 Types of Reference . . . . .
3.1.20 Constant and Isub . . . . .
3.1.21 Types of Value . . . . . .
3.1.22 Types of Picture . . . . .
CHAPTER 4: THE TRANSLATOR . . . . .
4.0 Introduction . . . . . . .
4.1 Translate . . . . . . . .
4.2 Forming the Concrete Procedure . . .
4.2.1 Low-level-parse . . . . .
4.2.2 Middle-level-parse . . . . .
4.2.3 High-level-parse . . . . .
4.3 Completion of the Concrete Procedure .
4.3.1 Reorganize . . . . . . .
4.3.1.1 Complete-options . . . .
4.3.1.2 Modify-statement-names . . .
4.3.1.3 Complete-attribute-implications
4.3.1.4 Defactor-declarations . . .
4.3.2 Construct-explicit-declarations .
4.3.2.1 Declare-parameters . . . .
4.3.2.2 Declare-statement-names . .
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
4.3.2.3 Construct-statement-name-declarations
4.3.3 Complete-structure-declarations . .
4.3.3-l Determine-structure . . . .
4.3.3.2 Expand-like-attribute . . . .
4.3.3.3 Convert-to-logical-levels . . .
4.3.3.4 Propagate-alignment . . . .
4.3.3.5 Find-applicable-declaration . .
4.3.3.6 Find-fully-qualified-name . . .
4.3.4 Construct-contextual-declarations . .
4.3.5 Construct-implicit-declarations . .
4.3.6 Complete-declarations . . . . .
4.3.6.1 Test-attribute-consistency . . .
4.3.6.2 Test-invalid-duplicates . . .
4.3.6.3 Append-system-defaults . . . .
4.3.6.4 Apply-defaults . . . . . .
4.3.6.5 Test-default-applicability . . .
4.3.6.6 Copy-descriptors . . . . .
4.3.6.7 Test-offset-in-description . . .
4.3.6.8 Test-descriptor-extent-expressions .
4.3.7 Validate-concrete-declarations . . .
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
4.3.7.1 Check-attribute-completeness-and-delete-attributes . .
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
4.4 Create-abstract-equivalent-tree . . . . .
4.4.1 Creation of Elocks and Groups . . . .
4.4.1.1 Create-procedure . . . . . .
4.4.1.2 Create-begin-block . . . . . .
4.4.1.3 Create-block . . . . . . .
4.4.1.4 Replace-concrete-designators . . .
4.4.1.5 Create-group . . . . . . .
4.4.1.6 Create-entry-or-executable-unit-list .
4.4.1.7 Create-executable-unit-list . . .
4.4.1.8 Create-executable-unit . . . . .
4.4.1.9 Create-entry-point . . . . . .
4.4.1.10 Create-statement-name-list . . . .
4.4.1.11 Create-condition-prefix-list . . .
4.4.1.12 Create-condition . . . . . .
4.4.2 Creation of Statements . . . . . .
4.4.2.1 Create-assignment-statement . . .
4.4.2.2 Create-by-name-assignment . . . .
4.4.2.3 Data-descriptions Proper for Assignment
4.4.2.4 Create-by-name-parts-list . . . .
4.4.2.5 Find-by-name-parts . . . . . .
4.4.2.6 Create-allocation . . . . . .
4.4.2.7 Create-format-statement . . . .
4.4.2.8 Create-format-iteration . . . .
4.4.2.9 Create-freeing . . . . . . -
4.4.2.10 Create-if-statement . . . . .
4.4.2.11 Create-balanced-unit . . . . .
4.4.2.12 Create-locate-statement . . . .
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
Contents
58
59
60
60
60
63
63
63
64
b4
65
66
68
68
68
69
70
71
71
72
72
73
75
76
76
77
78
78
79
80
82
82
83
85
86
a7
87
88
91
32
92
93
94
95
95
95
96
96
97
37
98
98
98
99
99
100
101
101
101
102
102
103
103
104
104
105
105
106
106
3
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 12

4.4.2.13 Create-on-statevent . . .
4.4.3 Create-declaration . . . . .
4.4.3.1 Create-named-constant . . .
4.4.3.2 Create-variable . . . .
4.4.3.3 Create-bound-pair-list . . .
4.4.3.4 Create-data-description . .
4.4.3.5 Create-data-type . . . .
4.4.3.6 Create-entry . . . . .
4.4.3.7 Create-refer-option . . .
4.4.3.8 Create-identifier . . . .
4.4.3.9 Create-initial-element . . .
4.4.4 Create-expression . . . . .
4.4.5 Create-reference . . . . .
4.4.5.1 Collect-subscripts . . . .
4.4.5.2 Apply-by-name-parts . . .
4.4.5.3 Apply-subscripts . . . .
4.4.5.4 Create-value-referen& . . .
4.4.5.4.1 Trim-da . . . . .
4.4.5.5 Create-named-constant-reference
4.4.5.6 Create-argument-list . . .
4.4.5.7 Create-builtin-function-reference
4.4.5.8 Create-pseudo-variable-reference
4.4.5.9 Create-entry-reference . .
4.4.5.10 Test-matching . . . .
4.4.5.11 Select-generic-alternative .
4.4.5.12 Test-generic-matching - .
4.4.5.13 Test-generic-aggregation .
4.4.5.14 Test-generic-description .
4.4.5.15 Test-generic-precision - .
4.4.6 Create-picture . . . . .
4.4.6.1 Create-nuaeric-picture . .
4.4.7 Create-constant . . . .
4.5 Validation of the Abstract Procedure
4.5.1 Validate-declaration . . .
4.5.2 Validate-automatic-declaration .
4.5.3 Validate-based-declaration . .
4.5.4 Validate-controlled-declaration
4.5.5 Validate-defined-declaration .
4.5.6 Validate-parameter-declaration .
4.5.7 Validate-static-declaration .
4.5.8 Validate-descriptor . . .
4.5.9 Evaluate-restricted-expression .
4.5.10 Apply-constraints . - - -
4.5.11 Test-constraints . . . .
4.6 Validate-program . . . - -
4.6.1 Validate-external-declaration .
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
CHAPTER 5: THE PWI INTERPRETER . . .
5.0 Introduction . . . . . .
5.1 The Interpretation-state . . .
5.1.1 Directories . . . - -
5.1.2 Block State . . . . .
5.1.3 File Information . . - .
5.1.4 Storage and Values . . . -
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
I
.
.
.
.
.
.
.
.
.
.
.
.
s
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
I
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
I
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
5.1.5 Generations, Evaluated Data Descriptions, and Evaluated Targets
5.1.6 Dataset ..............
5.2 Terminology and Definitions ...........
5.2.1 Current ..............
5.2.2 Block ...............
5.3 The Interpret Operation and the Initialization of the Interpretation State
5.3.1 Interpret . . . . . . . . . . . . . . .
5.3.2 Initialize-interpretation-state . . . - . . . . .
5.3.3 Build-file-directory-and-informations . . . . . . . .
5.3.4 Build-fdi . . . . . . . . . . . . . . .
5.3.5 Build-controlled-directory . . . . . . . . . . .
5.3.6 Allocate-static-storage-and-build-static-directory . . . . .
5.3.7 Program- epilogue . . . . . . . . . . . . .
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
I
.
.
.
.
.
.
.
.
.
.
.
.
.
107
107
108
109
110
110
112
113
114
114
114
115
117
120
120
121
121
122
122
123
123
124
124
125
126
127
127
128
130
131
132
134
137
137
137
138
138
139
139
139
133
140
140
141
142
142
143
143
143
143
143
145
146
147
147
148
148
148
148
148
143
149
150
150
151
151
4
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 13

CHAPTER 6: FLOW OF CONTROL . . . . . .
6.0 Introduction . . . . . . . .
6.1 Program Activation and Termination . . .
6.1.1 Program Termination . . . . .
6.1.1-l Execute-stop-statement . . . .
6.1.1.2 Stop-program . . . . . .
6.2 Block Activation and Termination . . .
6.2.1 Activate-procedure . . . . . .
6.2.1.1 Instal-arguments . . . . .
6.2.2 Activate-begin-block . . . . .
6.2.3 Prologue . . . . . . . .
6.2.4 Epilogue . . . . . . . .
6.3 Control within a Block . . . . . .
6.3.1 Normal-sequence . . . . . .
6.3.1.1 Advance-execution . . . . .
6.3.2 Execute-executable-unit . . . .
6.3.3 Execute-begin-block . . . . .
6.3.4 Execute-group . . . . . . .
6.3.4.1 Establish-controlled-group . . .
6.3.4.2 Initialize-spec-options . . .
6.3.4.3 Test-spec . . . . . . .
6.3.4.4 Establish-next-spec . . . .
6.3.4.5 Test-termination-of-controlled-group
6.3.5 Execute-if-statement . . .
6.3.5.1 Establish-truth-value . .
6.3.6 Execute-call-statement . . .
6.3.6.1 Entry-references . . .
6.3.6.1.1 Evaluate-entry-reference
6.3.6.1.2 Establish-argument . .
6.3.7 Execute-goto-statement . . .
6.3.7.1 Local-goto . . . . .
6.3.7.2 Trim-group-control . . .
6.3.8 Execute-null-statement . . .
6.3.9 Execute-return-statement . .
6.3.10 Execute-end-statement . . .
6.4 Conditions and Interrupts . . .
6.4.1 Conditions . . . . . .
6.4.1.1 Raise-condition . . .
6.4.1.2 Test-enablement . . .
6.4.1.3 Execute-signal-statement .
6.4.1.4 Evaluate-named-io-condition
6.4.2 Interrupts . . . . . .
6.4.2-l Execute-on-statement . .
6.4.2.2 Execute-revert-statement .
6.4.3 Interrupt . . . . . .
6.4.4 System-action . . . . .
6.4.4.1 Comment . . . . .
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
CHAPTER 7: STORAGE AND ASSIGNMENT . .
7.0 Introduction . . . . . .
7.1 The Generation . . . . . .
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
I
.
.
.
.
.
.
.
.
.
.
I
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
7.1.1 The Number of Elements in the Storage-index-list of a Generation
7.1.2 Correspondence between an Item-data-description and a Easic-value
7.1.3 Value of a Generation ........ . . .
7.1.4 Value of Storage Index ........ . . .
7.2 The Allocation of Storage ........ I . .
7.2.1 Execute-allocate-statement ....... . . .
7.2.2 Allocate-controlled-storage ...... . . .
7.2.3 Allocate-based-storage ........ . . *
7.2.4 Evaluate-in-option ......... . I .
7.2.5 Allocate ........... . . .
7.2.6 Suballocate .......... . . .
7.2.7 Evaluate-data-description-for-allocation ... . . .
7.2.8 Find-directory-entry ........ . . .
7.2.9 Make-allocation-unit ........ . . .
7.2.10 Initialize-refer-options ....... . . .
7.2.11 Find-block-state-of-declaration ..... . . .
7.3 Initialization ........... . . .
7.3.1 Initialize-generation ........ . a .
7.3.2 Initialize-scalar-element ....... . . .
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
I
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
I
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
153
153
153
153
153
153
154
154
155
155
156
156
157
157
157
157
158
158
159
159
161
161
162
163
163
163
164
164
165
166
166
167
167
167
168
169
169
169
169
170
171
171
171
172
172
174
174
175
175
175
175
176
177
178
180
180
180
181
182
182
183
184
185
185
186
187
188
188
189
Contents 5
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 14

7.3.3 Initialize-array . . - . .
7.4 The Freeing of Storage '. . . . .
7.4.1 Execute-free-statement . . . .
7.4.2 Free-controlled-storage . . .
7.4.3 Free-based-storage . . . . .
7.4.4 Deduce-in-option . . . . .
7.4.5 Free . . . . . . . .
7.5 Assignment . . . . . . . .
7.5.1 The Assignment Statement . . .
7.5.2 Target References . . . . .
7.5.2.1 Evaluated Targets . . . .
7.5.3 The Assignment Operation . . .
7.5.3.1 Promote-and-convert . . .
7.5.3.2 The Set-storage Operation . .
7.5.4 Pseudo-variables . . . . .
7.5.4.1 Imag-pv . . . . . .
7.5.4.2 OnChar-FV . . . . . .
7.5.4.3 Onsource-pv . . . . .
7.5.4.4 Pageno-pv . . . . . .
7.5.4.5 Real-pv . . . . . .
7.5.4.6 String-pv . . . . . .
7.5.4.7 Substr-pv . . . . . .
7.5.4.8 Unspec-pv . . . . . .
7.6 Variable-reference . . . . . .
7.6.1 Evaluate-variable-reference . .
7.6.1.1 Connected Generations . . .
7.6.2 Select-based-generation . . .
7.6.3 Check-based-reference . . . .
7.6.4 Overlay-strings . . . . .
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
7.6.5 Evaluate-data-description-for-reference
7.6.6 Select-qualified-reference . . . .
7.6.7 Select-subscripted-reference . . .
7.6.8 Evaluate-by-name-parts-list . . .
7.6.9 Evaluate-defined-reference . . . .
7.6.10 Evaluate-simply-defined-reference - .
7.6.11 Adjust-bound-pairs . . . . . .
7.6.12 Evaluate-isub-defined-reference . .
7.6.13 Expand-list-of-subscript-lists . . .
7.6.14 Transform-subscript-list . . . .
7.6.15 Evaluate-string-overlay-defined-reference
7.6.16 Check-simply-defined-reference .
7.6.17 Extract-slice-of-array - . .
7.7 Reference to Named Constant . - -
7.7.1 Evaluate-named-constant-reference
7.7.2 Search-file-directory . .
CHAPTER 8: INPUT/OUTPUT . . . .
8.0 Introduction . . . . .
8.1 Datasets . . . . . .
8.1.1 Record Datasets . . .
8.1.2
Stream Datasets - . .
8.2 Files . . . . . . .
8.2.1 Record Files . . . .
8.2.2 Stream Files . . . .
8.3 I/O Conditions . . . . .
8.3.1 Raise-io-condition . . .
8.4 Evaluate-file-option . . .
8.5 File Opening and Closing . .
8.5.1 The Open Statement . . .
8.5.1.1 Execute-open-statement .
8.5.1.2 Execute-single-opening -
8.5.1.3 Open . . . . .
8.5.1.4 Evaluate-tab-option .
8.5.1.5 Evaluate-title-option .
8.5.1.6 Evaluate-filename . .
8.5.2 The close Statement . .
8.5.2.1 Execute-close-statement
8.5.2.2 Execute-single-closing .
8.5.2.3 close . . . . .
8.6 The Record I/O Statements . .
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
I
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
I
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
I
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
I
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
*
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
189
191
191
191
192
193
193
194
194
194
195
196
196
197
198
198
199
200
200
201
202
203
204
205
205
206
207
207
208
209
210
212
213
213
214
215
215
217
217
218
218
219
220
220
223
225
225
225
225
225
226
226
226
226
226
227
228
228
228
228
229
230
231
231
232
232
232
232
233
6
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 15

8.6.1 The Read Statement . . . . .
8.6.1.1 Execute-read-statement . . .
8.6.1.2 Read . . . . . . .
8.6.2 The Write Statement . . . .
8.6.2.1 Execute-write-statement . .
8.6.2.2 Write . . . . . . .
8.6.3 The Locate Statement . . . .
8.6.3.1 Execute-locate-statement . .
8.6.4 The Rewrite Statement . . . .
8.6.4.1 Execute-rewrite-statement . .
8.6.4.2 Rewrite . . . . . .
8.6.5 The Delete Statement . . - .
8.6.5.1 Execute-delete-statement . .
8.6.5.2 Delete . . . . . . .
8.6.6 Operations Applicable to Record I/O
8.6.6.1 Evaluate-from-option . . .
8.6.6.2 Evaluate-into-option . . .
8.6.6.3 Evaluate-pointer-set-option .
8.6.6.4 Evaluate-key-option . . .
8.6.6.5 Evaluate-keyfrom-option . .
8.6.6.6 Evaluate-ignore-option . . .
8.6.6.7 Evaluate-keyto-option . - -
8.6.6.8 Construct-record . . - -
8.6.6.9 Insert-record . . . . .
8.6.6.10 Position-file . . . . .
8.6.6.11 Evaluate-size . s . . .
8.6.6.12 Exit-from-io . . . - -
8.6.6.13 Trim-io-control . . - -
8.7 The Stream I/O Statements . . - .
8.7.1 The Get Statement . . . . .
8.7.1.1 Execute-get-statement . . .
8.7.1.2 Execute-get-file . . . .
8.7.1.3 Execute-get-string . . . .
8.7.1.4 Get-list . . . . . .
8.7.1.4.1 Parse-list-input . . .
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
*
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
f
.
.
.
.
.
.
.
.
.
.
.
.
.
.
*
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
8.7.1.4.2 Parsing Categories for List Directed Input .
8.7.1.5 Get-data . . . . . . . . . .
8.7.1.5-l Parse-data-input-name . . . . . -
8.7.1.5.2 Parse-data-input-value . . . . . .
8.7.1.5.3 Parsing Categories for Data Directed Input - .
.
.
.
.
.
.
.
.
.
f
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
0
.
.
.
.
.
.
.
.
.
.
.
.
.
I
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
8.7.1.6 Get-edit . . . . . .
8.7.1.6.1 Execute-input-contrcl-format
8.7.1.6.2 Execute-input-data-format -
8.7.1.6.2.x Validate-input-format .
8.7.1.7 Input-stream-item . . . .
8.7.1.8 Basic-character-value . . .
8.7.1.9 Basic-bit-value . . . .
8.7.1.10 Input-stream-item-for-edit . .
8.7.2 The Put Statement . . . . .
8.7.2.1 Execute-put-statement . . .
8.7.2.2 Execute-put-file . . . -
8.7.2.3 Execute-put-string . . . .
8.7.2.4 Put-list . . . . . .
8.7.2.5 Put-data . . . . . -
8.7.2.6 Put-edit . . . . . .
8.7.2.6.1 Execute-output-control-format
8.7.2.6.2 Execute-output-data-format .
8.7.2.6.3 Edit-numeric-output . -
8.7.2.7 Output-string . . . . -
8.7.2.8 Output-string-item . . . -
8.7.2.9 Output-stream-item . . . -
8.7.2.10 Tab . . . . . . .
8.7.2.10.1 Output-tab . . . . .
8.7.2.11 Put-line . . . . . .
8.7.2.12 Fut-page . . . . . -
8.7.3 Operations Applicable to Stream I/O
8.7.3.1 Skip . . . . . - -
8.7.3.2 Evaluate-current-column . .
8.7.3.3 Evaluate-current-line . . .
8.7.3.4 Establish-next-data-item . .
8.7.3.4.1 Expand-edd . . . . .
8.7-3.4.2 Expand-generation . . .
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
‘
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
I
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
*
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
I
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
*
.
.
Contents
233
233
234
235
235
236
237
237
238
238
239
240
240
241
241
241
242
242
242
242
243
243
243
244
244
245
246
246
247
247
247
247
248
249
250
251
252
255
256
257
257
258
259
261
263
264
264
264
265
265
265
266
267
268
270
271
272
273
275
275
276
276
277
277
277
278
278
278
279
279
282
282
7
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 16

8.7.3.4.3 Make-name-and-subscript-list .
8.7.3.4.4 Expand-name-and-subscript . .
8.7.3.4.5 Subscript-to-comma-subscript .
8.7.3.4.6 Identifier-to-dotname . . .
8.7.3.5 Establish-next-format-itea . . .
8.7.3.6 Evaluate-format-item . . . .
8.7.3.6.1 Evaluate-format-expression . .
CHAPTER 9: EXPRESSIONS AND CONVERSION . . .
9.0 Introduction . . . . . . . .
9.1 Aggregate Expressions . . . . . .
9.1.1 Scalar and Aggregate TyFeS . . . .
9.1.1.1 Aggregate Type of a Data Description
9.1.1.2 Scalar Eleuents . . . . .
9.1.1.3 Treatment of Scalars . . . .
9.1.1.4 Compatibility . . . . . .
9.1.1.5 Correspondence . . . . . .
9.1.1.5.1 Correspondence of Scalar Elements
9.1.1.5.2 Correspondence of Data Types .
9.1.1.6 Generate-aggregate-result . . .
9.1.2 Integer Type . . . . . . .
9.1.2.1 Evaluate-expression-to-integer . .
9.1.3 Derived Data Types . . . . . .
9.1.3.1 Derived Base, Scale, and Rode . .
9.1.3.2 Converted Precision . . . .
9.1.3.3 Derived String Type . . . .
.
.
I
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
I
.
.
.
.
I
.
.
9.1.3.4 Further Definitions for Character and Bit Strings
9.1.4 Arithmetic Results ......
9.1.4.1 Conditions in Expressions ...
9.1.5 Expressions .......
9.1.6 Value References ......
9.1.7 Constants ........
9.x.8 Isubs .........
9.1.9 Parenthesized Expressions ....
9.1.10 Arguments ........
9.2 Prefix Operators .......
9.2.1 Prefix Expressions ......
9.2.2 Definition of the Prefix operators . .
9.2.2.1 Prefix-minus ......
9.2.2.2 Prefix-not .......
9.2.2.3 Prefix-plus ......
9.3 Infix Operators ........
9.3.1 Infix Expressions ......
9.3.2 Definition of the Infix Operators . .
9.3.2.1 Infix-add .......
9.3.2.2 Infix-and .......
9.3.2.3 Infix-cat .......
9.3.2.3.1 Concatenation of String Values .
9.3.2.4 Infix-divide ......
9.3.2.5 Infix-eq .......
9.3.2.5.1 Compare ......
9.3.2.6 Infix-ge .......
9.3.2.7 Infix-gt .......
9.3.2.8 Infix-le .......
9.3.2.9 Infix-lt .......
9.3.2.10 Infix-multiply ......
9.3.2.11 Infix-ne .......
9.3.2.12 Infix-or .......
9.3.2.13 Infix-power ......
9.3.2.14 Infix-subtract ......
9.4 Builtin-functions .......
9.4.1 Builtin-function Reference ....
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
9.4.2 Special Terms Defined for Builtin-functions
9.4.2.1 Definition of N . . . . . .
9.4.2.2 The Arguments p and g . . . . .
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
9.4.3 Operations Used in Builtin-function Definitions
9.4.3.1 Get-established-onvalue . . . . .
9.4.4 Definition of the Builtin-functions . . .
9.4.4.1 Abs-bif . . . . . . . . .
9.4.4.2 Aces-bif . . . . . . . . .
8
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
‘
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
283
284
285
285
285
286
287
289
289
289
289
289
289
290
290
291
291
292
293
295
295
295
235
296
237
297
298
299
299
299
300
300
300
300
301
301
301
301
302
302
303
303
303
304
304
305
305
305
306
306
308
309
309
310
310
311
311
312
313
314
314
314
314
315
315
315
316
316
317
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 17

9.4.4.3 Add-bif . .
9.4.4.4 Addr-bif . .
9.4.4.5 After-bif . .
9.4.4.6 Allocation-bif .
9.4.4.7 Asin-bif .
9.4.4.8 Atan-bif .
9.4.4.9 Atand-bif .
9.4.4.10 Atanh-bif .
9.4.4.11 Before-bif .
9.4.4.12 Binary-bif .
9.4.4.13 Bit-bif .
9.4.4.14 Bool-bif .
9.4.4.15 Ceil-bif .
9.4.4.16 Character-bif
9.4.4.17 Collate-bif
9.4.4.18 Complex-bif
9.4.4.19 Conjg-bif .
9.4.4.20 Copy-bif .
9.4.4.21 Cos-bif .
9.4.4.22 Cosd-bif .
9.4.4.23 Cosh-bif .
9.4.4.24 Date-bif .
9.4.4.25 Decat-bif .
9.4.4.26 Decimal-bif
9.4.4.27 Dimension-bif
9.4.4.28 Divide-bif .
9.4.4.29 Dot-bif .
9.4.4.30 Empty-bif .
9.4.4.31 Erf-bif -
9.4.4.32 Erfc-bif .
9.4.4.33 Every-bif .
9.4.4.34 Exp-bif .
9.4.4.35 Fixed-bif .
9.4.4.36 Float-bif .
9.4.4.37 Floor-bif .
9.4.4.38 Hbound-bif .
9.4.4.39 High-bif .
9.4.4.40 Imag-bif .
9.4.4.41 Index-bif .
9.4.4.42 Lbound-bif .
9.4.4.43 Length-bif .
9.4.4.44 Lineno-bif .
9.4.4.45 Log-bif .
9.4.4.46 LoglO-bif .
9.4.4.47 LogZ-bif .
9.4.4.48 Low-bif .
9.4.4.49 Max-bif .
9.4.4.50 Min-bif .
9.4.4.51 Mod-bif .
9.4.4.52 Multiply-bif
9.4.4.53 Null-bif .
9.4.4.54 Offset-bif .
9.4.4.55 Cnchar-bif .
9.4.4.56 Oncode-bif .
9.4.4.57 Cnfield-bif
9.4.4.58 Onfile-bif .
9.4.4.59 Onkey-bif .
9.4.4.60 Onloc-bif .
9.4.4.61 Cnsource-bif
9.4.4.62 Pageno-bif .
9.4.4.63 Pointer-bif
9.4.4.64 Precision-bif
9.4.4.65 Prod-bif .
9.4.4.66 Real-bif .
9.4.4.67 Reverse-bif
9.4.4.68 Round-bif .
9.4.4.69 Sign-bif .
9.4.4.78 Sin-bif .
9.4.4.71 Sind-bif .
9.4.4.72 Sinh-bif .
9.4.4.73 Some-bif .
9.4.4.74 Sqrt-bif .
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
I
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
a
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
I
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
*
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
f
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
I
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
f
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
a
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
f
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
f
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
I
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
*
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
D
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
f
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
317
318
318
313
319
320
321
321
322
322
323
323
324
325
325
326
326
327
327
328
328
328
329
330
330
331
332
332
333
333
334
334
335
335
336
336
337
337
338
338
339
339
339
340
340
341
341
342
343
344
345
345
345
346
346
346
347
347
347
348
348
340
349
350
350
351
352
352
353
353
354
354
Contents 9
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 18

9.9.4.75 String-bif .
9.4.4.76 Substr-bif .
9.4.4.77 Subtract-bif
9.4.4.78 Sum-bif .
9.4.4.79 Tan-bif .
9.4.4.80 Tand-bif .
9.4.4.81 Tanh-bif .
9.4.4.82 Time-bif .
9.4.4.83 Translate-bif
9.4.4.84 Trunc-bif .
9.4.4.85 Unspec-bif .
9.4.4.86 Valid-bif .
9.4.4.87 Verify-bif .
9.5 Conversion . . .
/- - l - -
. . . . .
. . . . .
. . . . .
. . . . .
. . . . .
. . . . .
. . . . .
. . . . .
. . . . .
. . . . .
. . . . .
. . . . .
. . . . .
9.5.1 Conversion of Scalar Values . .
9.5.1.1 Informal Invocation of Convert .
9.5.1.2 Convert-to-fixed . . . .
9.5.1.3 Convert-to-float . . . .
9.5.1.4 Convert-to-bit . . . . .
9.5.1.5 Convert-to-character . . .
9.5.1.6 Conversion to Float Decimal .
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
9.5.1.7 Conversion from String or Picture to Arithmetic
9.5.1.8 Basic Numeric Value of a String . . . .
9.5.1.9 Evaluate-real-constant . . . . . . .
9.5.2 Numeric Pictures . . . . . . . . .
9.5.2.1 Editing Numeric Pictures . . . . . .
9.5.2.2 Editing a Numeric Picture Field . . . .
9.5.2.3 Validity of a Numeric Pictured Value . . .
9.5.2.4 Validity of a Field of a Numeric Pictured Value
9.5.3 Character Pictures . . . . . . . . .
9.5.3.1 Test-char-pit-char . . . . . . . .
INDEX . . . . . . . . . ..I..
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
355
355
356
356
357
357
358
358
359
360
360
361
361
363
363
366
366
367
368
369
370
371
372
373
374
375
376
380
381
382
382
383
Tables
Table 4.1 Concrete Terminals of Significance to Test-generic-description . 129
Table 4.2 Table of *bit-value% as a Function of $symbol)s and
(radix-factor)s for Create-constant . . . . . . . . 136
Table 9.1 Table of Converted Precisions as a function of Target and
Source Attributes . . . . . . . . . . . . 296
Table 9.2 Table of Scalar-results as a Function of <kit-value+s for
Bool-bif . . . . . . . . . . . . . . . 324
Table 9.3 Table of $symbol)s as a Function of *suppression-type%for
Edit-numeric-picture-field . . . . . . . . . . 379
Table 9.4 Table of $symbol)s as a Function of <numeric-picture-element>s
and *character-string-value% for Edit-numeric-picture-*ield . 379
10
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 19

Chapter 1: Scope and Overviews
1.0 Scope
This document defines the computer programming language PL/I. It is intended to serve as
an authoritative reference rather than as a tutorial introduction.
The definition is accomplished by specifying a conceptual PL/I machine which translates
and interprets intended PL/I programs. Section 1.1 provides a brief introduction to the
statements and data types included in the language, to the structure and use of the
document, and to the method of definition. The relationship between an actual
implementation and the conceptual machine of this document is described in Section 1.2,
and the detailed specification of the notation to be used follows in Section 1.3. The
main body of the definition is then begun at Section 1.4, and is completed by Chapters 2
through 9.
1.1 An Informal Guide to the PL/I Definition
1.1.1 A SUMMARY OF PL/I
A PL/I program consists of a set of procedures, each of which is written as a sequence of
statements. The XINCLUDE construct may be used to include text from other sources during
program translation.
All of the statement types are summarized here in groupings which are presented as a
means of obtaining an overview of the language and which may be related to the
organization of the document.
Structural:
Declarative:
Flow of Control:
Storage:
PROCEDURE
ENTRY
BEGIN
DO
END
DECLARE
DEFAULT
FORMAT
CALL
RETURN
IF
GO TO
Null Statement
STOP
ON
REVERT
1
Interrupt Handling
SIGNAL
ALLOCATE
FREE
Assignment Statement
Chapter (1: Scope and Overviews 1
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 20

Input/output: OPEN
CLOSE
GET
>
Stream I/0
PUT
READ
WRITE
LOCATE Record I/O
REWRITE
DELETE
Names may be declared to represent data of the following types , either as single values,
or as aggregates in the form of arrays or structures:
Arithmetic
>
or
CHARACTER PICTURE
BIT
AREA
ENTRY
FILE
FORMAT
LABEL
OFFSET
POINTER
Values may be computed by expressions written using a specific set of operators and
builtin functions, most of which may be applied to aggregates as well as to single
values, together with user-defined procedures which, likewise, may operate on and return
aggregate as well as single values.
1.1.2 THE FORM OF THE DEFINITION
The conceptual PL/I machine is a processor which has a set of operations acting on
information stored in its memory. The operations are specified as algorithms, and may be
viewed as the component parts of one single algorithm, "define-program", which carries
out the entire translation and interpretation process. The information in memory is all
held in the form of tree structures.
The definition algorithm operates as follows.
Sequences of symbols which are intended to represent PL/I external procedures (i.e.
procedures not contained in any other procedure) are processed by a Translator. This
processing involves systematically analyzing, transforming, and validating each external
procedure. The analysis uses a grammar known as the Concrete Syntax to produce the
concrete form of an external procedure as a tree structure. This is transformed to the
abstract form, which is a tree satisfying the Abstract Syntax, designed to be more
convenient for interpretation. Further validation is then carried out on the abstract
form. The Translator finally performs some validation of the mutual consistency of the
set of external procedures which comprise the complete program.
The semantics of the program when applied to given initial datasets (i.e. collections of
data) are then provided by an Interpreter. The datasets are part of the PL/I machine's
memory, which is a tree satisfying the Machine-state Svntax, and it is the sequence of
changes in the datasets which constitutes the defined semantics.
In addition to translating and interpreting all programs which are valid according to
this definition, the PL/I machine detects as non-standard all those which violate a
requirement involving the words "must- or "must not" in the algorithms performed during
translation or interpretation. The implications for an actual implementation are
described in Section 1.2.
2
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 21

The method of definition may be seen in outline as follows:
SYMBOL-LISTS
REPRESENTING
EXTERNAL
PROCEDURES PROGRAM TREE ------------> ____________ > r--------------1 (ABSTRACT FORM)
TRANSLATOR 1 __--- -__--- ____-- > ;--------------1 INTERPRETER 1 ------------> L--------------J I. -------------- -1 n
/’ /
/ l! DATASETS
It may also be helpful to visualize the abstract machine on which both the Translator and
the Interpreter are "executed":
r-----‘--------------------------------
I OPERATIONS
I
(unchanging set of
algorithms) ,-----------------MEMOR;---------------
i (changeable machine-state tree)
1
machine-state
/ \
I / I ' SYMBOL LISTS 1 --------e-e----->
---------------->
DATASETS I L----------------------------------J
The inputs from outside the machine occur at initialization of the Translator and
Interpreter; the datasets may change during interpretation. However, there are no
outputs defined since the datasets are treated for the purposes of this definition as
being a part of the storage of the machine, i.e. as being "on-line" when needed.
1.1.3 SUMMARY OF CBAPTER STRUCTURE
r-------------------------------------’-------------------------,
TOP-LEVEL OF MACHINE-STATE AND OPERATIONS ___----__------~------~~~--~~~--~~~-----~~~----~-----
CONCRETE SYNTAX
ABSTRACT SYNTAX
TRANSLATOR ~--;-----------------------------------------------------------~
. INTERPRETATION-STATE + TOP-LEVEL OF INTERPRETER
1 6. FLOW OF CONTROL Concerned with the
STORAGE AND ASSIGNMENT three parts of the
INPUT/OUTPUT interpretation-state
EXPRESSIONS Common Subroutines
for Chapters 5-8 L-----,------------------------------------------------------J
The operations of Chapter 1 serve to drive the Translator and Interpreter.
The operations of the Translator are all contained in Chapter 4, and use the syntaxes of
Chapters 2 and 3.
The operations of the Interpreter comprise all the operations in Chapters 5-9. After the
initialization in Chapter 5, the relevant operations will be in Chapters 6, 7, or 8
depending on the type of statement being interpreted. All of these chapters invoke
operations in Chapter 9 where necessary.
Chapter I: Scope and Overviews 3
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 22

Within each chapter, the sections are logically organized, and the Table of Contents may
be used to obtain an overview of the structure.
All readers are recommended to acquire a good understanding of Chapter 1 in its entirety.
Thereafter, it is possible to read the definition as a systematic whole, or to use the
document to locate answers to specific questions by combining an appreciation of the
overall structure of the definition with judicious use of the index. To illustrate this
latter usage, we consider each chapter in turn together with a sample question answerable
from it.
Chapter 2 contains the definition of the Concrete Syntax. The Concrete Syntax consists
of rules describing valid forms of PL/I constructs in concrete tree form. The syntax is
permissive in the sense that some of the constructs permitted as being syntactically
correct may later be found to be meaningless.
I************+**+**+*++*******+*****+*************************************************
QUESTION
Is the following statement correct?
GET LIST (A(I,J) DO I = 1 TO M,N);
ANSWER
The first possibility is that there may be an error according to the Concrete
Syntax. The index entries for "get-statement" lead to the Middle-level Syntax, and
study of rules CMllO, 111, 119, 122, and 123 reveals that an extra pair of
parentheses is required around the form {input-target-commalist) DO *do-specp.
This is in order to resolve the ambiguity exhibited in this example. The correct
form is either
GET LIST ((A(I,J) DO I = 1 TO M),N);
or GET LIST ((A(I,Jl DO I = 1 TO M,N));
depending on whether the last input value is intended to be assigned to N or to
A(N,J).
I*********+*******++***++****************************************************************
Chapter 3 contains the definition of the Abstract Syntax. Many parts of the Abstract
Syntax description intentionally bear a strong resenblance to the ccrresponding parts cf
the Concrete Syntax. Names in the Abstract Syntax have been chosen to resemble those of
corresponding parts of the Concrete Syntax in order to make obvious as far as possible
the relationship between the syntaxes.
QUESTION
May the XEYTO option on a READ stateuent specify that the key be assigned to a
substring of a variable?
ANSWER
The Concrete Syntax for a Ckeyto-cptionp shows nerely that a {reference? must be
specified. However v the Abstract Syntax shows the foru of a program after the
Translator has completed all declarations, and has thus been able to associate each
reference with the appropriate declaration and make more subtle distinctions. The
rule All7 for <keyto-option> shows " <target-reference> (scalar & character) ", and
Al79 shows that this permits a <pseudo-variable-reference>, e.g. the SUBSTR pseudo-
variable. The parenthesized constraint "(scalar & character)" shows that it must
be a single target (not an array or structure) which is character-valued.
No further restrictions are found by using the index to search the Translator and
Interpreter, so that the answer is: yes, provided the sutstring is a scalar of
character type.
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 23

Chapter 4 defines the Translator whereby each of the individual PL/I program portions
(external procedures1 is translated from the submitted character string form to tree form
and appended to the program tree. This process involves the parsing of each external
procedure using the Concrete Syntax to obtain a concrete tree, insertion of missing
options and completion of attribute sets in that concrete tree, conversion from that
concrete tree to an abstract tree, and, finally, validation of the whole program. Onae
formed, the abstract tree is not modified.
C***********+************+**************************************************************
QUESTION
What file is implied in PUT LIST(X); ?
ANSWER
This seems at first sight as though it might be a sexantic question abcut the <put-
statement>. However, the Abstract Syntax shows that a <file-option> must be
present in <put-file> (A1241, and this neans that if it was absent in the concrete
form, it must have been supplied by the Translator if the statement was valid.
In fact, immediately after parsing the input, the Translator completes the concrete
procedure in various ways, one of which is to insert the equivalent of
FILE(SYSPRINT) into our fput-statement) (Step 2 of the cperation complete-options,
Section 4.3.1.11.
The reason that it has to be handled early in the Translator is that it will lead
to a contextual declaration of the name SYSPRINT if our statement is not within the
scope of an existing declaration for SYSPRINT. It is necessary to complete all
declarations prior to execution in order to resolve references correctly.
Chapter 5 contains the definitions of the Machine-state Syntax, the initialization of the
machine-state tree, and the starting of the interpretation process.
~*****+************t*******************************************************************
QUESTION
May the first procedure executed in a program have arguments passed to it?
ANSWER
For the initialization of execution, we consult Sections 1.4.3.3 and 5.3.1.
In Step 1 of operation interpretation-phase (Section 1.4.3.3) an <entry-value+ is
obtained from outside the definition. It designates the first procedure in the
<program> to be activated. The "must not" condition specifies that the document
gives no meaning to a program whose <entry-point> designated by the fentry-value+
contains a <parameter-name-list>. (See Section 1.3.3.4 for the definition of
"must".)
Additionally, in Step 2 of operation interpret (Section 5.3.1). an fevaluated-
entry-reference+ containing only an fentry-value> and not an festablished-argument-
list+ (see production rule M25 in Section 5.1.2) is used to activate the first
procedure.
Thus the passing of an argument-list to the first procedure would be an extension
beyond the language defined in this document.
Chapter 1: Scope and Overviews 5
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 24

chapter 6 describes the operations of the Interpreter affecting the flow of control
through blocks, groups, and statements of the program. Normal flow of control consists
of the execution of a list of executable units within a block. The definition also
defines calling, parameter/argument matching and result returning, and abnormal flow of
control caused by interrupts.
QUESTION
Is it permissible to REVERT a condition for which no ON statement has been
executed?
ANSWER
This is a semantic question about interrupt handling. The operation execute-
revert-statement (Section 6.4.2.2) deletes a member from the current testablished-
on-unit-list+ if an appropriate one exists, otherwise it performs normal-sequence
to pass on to the next statement. Since the action of execute-on-statement is
merely to append <established-on-unit% to such lists, it is clear that execute-
revert-statement is indifferent to the absence of such actions. The answer to the
question is that it is permissible and has no effect on the interrupt handling
mechanism.
Chapter 7 defines the use of storage including the allocation, freeing and initialization
of storage, and the referencing of variables and named constants. The assignment
statement, aggregate assignment, and pseudo-variables, are also defined.
t******************+****t***+*********************************************~***************
QUESTION
Does DECLARE A(5) INITIAL (01; lead to all 5 elements of A being set to 0 when A is
allocated?
ANSWER
The allocation and initialization of storage is treated in Chapter 7.
The operation initialize-array (Section 7.3.31 iterates over Step 4 while tint.
Since nt in this case is 1 (see Step 2) and n is initially 1 and is incremented in
step 4.7, the iteration is performed once only, thus initializing the first element
A(l) only.
1 **+*************+****~*****t********t+************************************************
Chapter 8 deals with the transmission of data between external media and internal
storage, including the opening and closing of files, stream and record transmission, and
nterrupts applicable to I/O operations.
QUESTION
Is DECLARE F RECCRD; OPEN FILE(F) PRINT; valid?
ANSWER
Although most of the declarative structure is evident in the Abstract Syntax and
checked by the Translator , this is not altogether true cf file attributes since
they may still be incomplete until the file is Opned during execution. Therefore
their valid combinations have to be tested also at this late stage.
The open operation (Section 8.5.1.31 shows that <Print> iuplies <strean> in Step 2,
and that <stream> and <record> cause the result *fail> - to be returned from the
attempt to open the file. -Thus if the <open-statement> is executed, it will lead
to the raising of the UNDEFINEDFILE condition when an attempt is made to open the
file (see Step 6 of execute-single-opening, Section 8.5.1.2).
6
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 25

Chapter 9 describes the evaluation of expressions, also conversions between data types,
which can take place in expression evaluation, and the builtin functions of PWI.
++*******+*+**+********+*++*****++*+**+************************************************
QUESTION
Is it possible to add an array to a structure?
ANSWER
The evaluation of expressions in Chapter 9 begins by considering the general
treatment of aggregate operands. The definition of compatibility in Section
9.1.1.4 leads us to Case 3, yielding the answer that the array must be such that an
element of it is compatible with the given structure. Applying the test for
compatibility again, this would be true if the element is a scalar (Case 1) or a
compatible structure (Case 2 and further recursions).
4 ************+*****+****+**+*+***+*++*****************************~******************
1.1.4 INTRODUCTICN TC 'IRE METALANGUAGE
An informal discussion of the main features of the metalanguage is now offered before
proceeding to its more rigorous definition in Secticn 1.3.
1.1.4.1 Tree Concepts
The definition of PL/I deals with three classes of trees (concrete parse, abstract text.
and machine-state), and a uniform concept of tree is enployed throughout. This is of a
tree which is a directed graph with a label (e.g. <procedure>) at each node, and where
the subtrees of any node are ordered. (Although the ordering is often irrelevant, it is
needed, e.g., in the concrete parse and in lists, and it was decided that the simple
uniformity of concept outweighed the advantages of explicitly distinguishing instances
where the order was significant.) Moreover, each node implicitly has a unique-name by
which it can be "designated" when required. A copy of the tree has the same ordered
structure and labelling, but new unique-names tc designate its nodes. Equality of trees
requires equality of all except the unique-names of nodes, and identity requires that
these also match.
The explicit label of a node may be either a grammatical category-name, or some other
type of value such as an integer or a designator (i.e. a copy of the unique-name
implicitly associated with some node). Thus a single value may be handled as a
degenerate tree with only a root-node labelled with this value, and data objects
referenced in the metalanguage are uniformly regarded as trees.
The terminology applied to trees is developed from the starting-point of a tree
consisting of a node (the root-node) and an ordered, possibly empty, Set of immediately
contained trees.
A tree, X, is said to be contained in a tree, Y, if X is immediately contained in Y, or
if X is immediately contained in some tree contained in Y. X is simply contained in Y if
and only if it is contained in Y and it is not contained in any tree, Z, contained in Y
and having a root-node equal to that of X or of Y. For example, if we refer to an
<expression> simply contained in a tree representing some statement in a program, we mean
the complete <expression> tree and not some subexpression which might be a tree rooted in
<expression> at some lower level within it. Since simple containrent is a frequently
required concept, the use of any form of possessive phrase not employing the verb
"contain" or the noun "component", is taken to imply simple containment, e.g. "if y has
an <expression>' (meaning a tree with root-node labelled <expression>), "the <expression>
of y", or "its <expression>".
Chapter li Scope and Overviews 7
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 26

The terminology makes it possible to perform the three essential types of operation on
trees, namely, to test them 'for the presence or type of their subtrees, to select
subtrees from them. and to construct or alter them. Eiowever, construction can be
laborious if phrased as *root-node <a> with two immediate subtrees, the first being <b>
with immediate subtrees <c> and <d>, and the second being x" (where x is the name of some
tree, meaning that a copy of it is to be made in constructing the new tree). This may
therefore be abbreviated as
<a>:
<b>:
<c>
<d>;
x:
The indentation is inessential, although helpful with large trees. One or more trailing
semicolons at the end of such a constructed tree may optionally be replaced by a period.
1.1.4.2 Syntaxes
Proceeding from consideration of particular trees to classes of trees, we encounter
grammars composed of sets of production-rules couched in a slightly extended Backus-Naur
Form. The interpretation we make of such rules is that they describe the structure of a
tree belonging to the class described by the grammar - it is only in the traditional
syntactic use of these rules that the sequence of terminal nodes of a tree acquires a
special importance as being the sequence of characters representing an utterance in the
language.
As an illustration, we will construct a tree that conforms to a rule of the form
<p>::= <q>l{<r-list>l<s>I*<t>l<u>l
A grammatical rule in the metalanguage is rather like a statement in a language which it
may be used to define, i.e. a metalanguage rule itself has a grammatical structure which
has to be validated and correctly interpreted. We have chosen to escape from circularity
or regress at this point by not giving a formal grammar for the metalanguage, but
describing in prose how a rule is to be interpreted. So in the above illustration, the
rule indicates that any node labelled <p> must have subtrees as described by the right-
hand side of the rule. There are three types of metalanguage operator, for concatenation
(indicated by juxtaposition in the rule), permutation ("*"I and choice between
alternatives ("I"), in descending order of precedence. Parenthesized expressions are
regarded as single operands, and the first stage in using a rule to construct a tree is
to partition the syntax expression into subexpressions separated by "I", the operator of
lowest precedence, and to choose one of these alternative subexpressions. Suppose we
discard the alternative of having <q> alone, and next partition the other alternative by
the "au operator and decide to use the permutation which reverses these partitions to
yield
<g>[<u>lC<r-list>l<s>I
The brackets indicate that their contents may be optionally omitted, which we choose to
do here, and the braces enclose a syntax expression which must be interpreted according
to the method just described. Choosing the alternative <r-list> we would then have the
tree
<p>:
<g>
<r-list>;
me underlining of t indicates that it always labels a terminal node in the grammar in
(restion, i.e. it does not occur on the left-hand side of any production-rule. <r-1isU
haicates by convention that it is to have one or more imediate subtrees of type <r>
beneath it. We then apply the rule for <r> separately to each of these. The
construction is colnpleted when each terminal node of the tree is either of a type which
is always terminal or of a type which has produced an empty set of SUbtreeS.
8
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 27

1.1.4.3 Alqorithm Concepts
The operations of the abstract machine are defined by algorithms, which may be compared
with the logic or microcode supporting the operation codes of a computer. These
operations inspect the <machine-state+ or change the fmachine-state>, or both, i.e. the
memory of the abstract machine , which contains all the information directly or indirectly
affecting semantics, including some form of the <program> itself. (The metabrackets -f'
and "9" are used in the PL/I definition of the machine-state nodes, except for those
belonging to the grammar of the abstract program which are distinguished by *<" and ">"
to help the reader.)
An operation of the machine is defined by a sequence of Steps, or a set of mutually
exclusive Cases, numbered from 1 to n. The i'th Step or Case may itself contain a
sequence of Steps or a set of Cases numbered from i-1 to i-m, and so forth. Each Step or
Case specifies actions to be performed, using various informal "statement types" in the
metalanguage. A Case begins by stating the predicate that must be satisfied for it to be
applicable.
6.2.3 PROLOGUE
This operation is invoked at the beginning of every blcck activation to establish
the <automatic> and <defined> variables local to that block. The <autoaatic>
variables are initialized if their <declaraticn>s specify initialization. Any
<expression>s evaluated during the prologue, such as in <extent-expression>s or
<expression>s in <initial>, are not allowed tc reference other <autonatic> or
<defined> variables local to this blcck. The operation find-directory-entry will
impose the restriction when it finds a reference to a variable declared in a block
for which there exists a tprologue-flaq>. The tproloque-flaq, is only present
while the operation prologue is active.
Operation: proloque
Step 1. Attach a tproloque-flag, to the current <linkage-part+.
Step 2. For each <declaration>,d, of the current block, that contains <automatic>
or <defined>, perform step 2.1.
Step 2.1. Let id be the <identifier> immediately contained in d, and let dd be
the <data-description> immediately contained in the <variable> of d.
Perform evaluate-data-description-for-allocation to obtain an
*evaluated-data-description+,edd.
Case 2.1.1. d contains <automatic>.
Step 2-1.1.1. Perform allocate(edd) to obtain a tgeneration+,g.
Step 2.1-1.2. Append to the current <automatic-directory-entry-list+ an
<automatic-directory-entry+: id g.
Step 2.1.1.3. If d contains <initial> then perform initialize-
generation(g,d).
Case 2.1.2. d contains <defined>.
Append to the current <defined-directory-entry-list+ a
<defined-directory-entry+: id edd.
Step 3. Delete the fproloque-flag+ of the current <linkage-part+.
Step 4. Replace the current *statement-control9 by a
*statement-control+:
*operation-list+:
<operation9 for advance-execution.
Example 1.1. The Prologue Operation.
Chapter 1:' Scope and Overviews 9
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 28

As an example of an operation, consider the definition of the prologue operation in
Example 1.1. This begins with an introductory paragraph which gives soze guidance to the
reader, but the definitive material is not reached until the "Operation:" heading.
Step 1 consists of an attach action, reaning that one tree is to be copied as a subtree
of another in the positicwhich it may validly occur according to the syntactic rules
governing the type of the latter. The word "current" has been defined precisely with
respect to the particular fmachine-state> used in defining FL/I - the current flinkage-
part, is the *linkage-part, simply contained in the <block-state, corresponding to the
block currently being executed.. step 2 indicates iteration with a for each specification
applied to the first form of the perform action, which refers to one or more Steps in the
same operation. A name such as d introduced after a comma is a variable, local to the
operation, whose value (v, say) is a designator of the tree mentioned immediately
preceding the -,d". On subsequent uses of d, it is dereferenced, i.e. it means "the tree
designated by v" except when it is redefined in a way similar to its original definition
(as would happen here on the next iteration of "for each"), or by its occurrence after
the word "let". The let statement is merely a more explicit syntax for this kind of
definition of a local var=le, and Step 2.1 contains an instance of this. If d contains
<automatic>, the predicate of Case 2.1.1 is satisfied and Steps 2.1.1.1 to 2.1.1.3 will
be performed in sequence.
The second form of perform, which is used to invoke another operation as a subroutine or
function, occurs in Step 2.1.1.1. Here it is a function that is invoked "to obtain" a
resulting value, which is then given a name. edd is passed as an argument to this
operation, whose definition will name a parameter to correspond to it. Argument passing
is by reference, i.e. a designator to the argument is passed and becomes the value of the
parameter, which is then dereferenced on use, behaving just like a local variable.
The append action in Step 2.1.1.2 attaches a tree at the end of a list, and Step 2.1.1.3
exemplifies an if statement containing a subroutine call. Steps 3 and 4 consist of the
other two treF actions which we use, delete and replace. When a tree is replaced by
another, the first tree is deleted and a copy of the second tree is made with its root at
the same position, and having the same implicit name, as had the root of the tree just
deleted. Assignment of a value to a variable is permitted in the syntactic form of a set
action, e.g. "Set tv to t-3",, but this is only an alternative form of replacement wz
the usual implied dereferencing of the variable name, meaning "replace the tree
designated by the strict designator value of tv by ftrues".
Other statement types not illustrated here are go to (used sparingly), terminate an
operation at some point other than the end of the last Step, and return a value from a
function (which also terminates the operation). Values are returned by reference, so
that if the result is a tree constructed within the function it must be copied back to
the caller who will then receive a designator of the copy; values local to an operation
are deleted when it terminates.
We have now reached a point at which it is necessary to indicate a mechanization of the
metalanguage which will suffice to bolster intuition in its weaker moments. Step 4
provides the cue for this, since it constructs a tree which is to be placed in the
*machine-state, at exactly the point at which it tails off into informality. Of the
<operation-list% in the fmachine-state,, just one is said to be "active" and has a
conceptual processor associated with it which may execute operations. In particular, the
PL/I machine has a current *block-state> which has a <statement-control+ where one could
mechanize the actions of the metalanguage involved in interpreting the statements Of the
current block.
The <operation-list* is to be conceived as a push-down stack, where each toperation, will
have a subtree which is not formally defined, but contains such information as the name
of the operation, a list of its parameters and local variables with their current values,
all trees constructed during execution of the operation or copied back to it from invoked
functions, an indication of whereabouts in which Step or Case it is executing its
algorithm, how far it has proceeded through a "for each" iteration, and so forth. When a
"perform" invokes another operation, this pushes down the stack and becomes the active
operation; when an operation terminates, its whole tree (including its local values) is
deleted and the stack pops up, activity in the operation now at the head of the stack
being resumed immediately after the point in its algorithm at which it was suspended.
To return to the analogy suggested at the beginning of this section, it is as though an
abstract processor had a machine cycle which could cause the execution of microcoded
operations with their own local memories per invocation, together with the sophistication
of the stacking capability.
10
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 29

1.2 Relationships between an Implementation and this Definition
The inputs of the conceptual PL/I machine are one or more (symbol-1ist)s representing
PL/I external procedures, an <entry-value+, and a fdataset-list,. This combination will
be referred to as a proqram-run-
The standard definition of PL/I for a particular implementation is completed by defining
(not necessarily in the style of this document1 the implementation-defined features
listed in Section 1.2.2, together with the representation of a program-run's elements
($symbol-list+, <entry-value+, and {dataset-list?) in the implementation's operating
environment. With this information available, the conceptual PWI machine gives one or
more interpretations to a program-run.
The main purpose of this document is to define the semantics of interpreting valid PL/I
program-runs. These semantics are constituted solely by the sequence of changes in the
<data-set* of the -chine-state+, and an implementation is free to achieve them by any
means. The operations and other parts of the <machine-states which are the mechanisms
used in this document to define the semantics need not be reflected directly in
implementation.
An implementation's interpretation of a program-run conforms to the standard if and only
if it conforms to one of the conceptual interpretations as follows:
(1) If the conceptual interpretation rejects a program-run (via failure of a "must"
test) or if it never completes the translation-phase, then any interpretation by
the implementation conforms. In particular, an implementation may or may not
reject a program-run at the same point as it is rejected by the standard, or at
all.
(21 Otherwise, the implementation's interpretation conforms if it makes the same
sequence of changes to datasets as does the conceptual interpretation.
(3) The implementation's interpretation also conforms if it deviates from (1) and
(2) only as permitted by the flexibilities of interpretation specified in
Section 1.2.1.
Note that this implies that an implementation may provide extensions beyond the language
defined in this standard, but is still required to conform for a program not using those
extensions just as if the extensions were not available.
1.2.1 FLEXIBILITIES OF INTERPRETATION
Through use of the terms "optionally" and "in any order', the operation definitions of
the conceptual PWI machine permit most of the flexibility necessary for efficient
implementation of PL/I. However, there are some rules that, if given formally with a
metalanguage of the kind used here , would require constructions so elaborate as to impede
understanding seriously. These rules are given in this section using informal language
and making direct reference to possible actions of an implementation.
1.2.1.1 Rejection of Proqrams
If some part of a program is such that its interpretation would cause the program to be
rejected, then an implementation may reject the program even if the conceptual
interpretation does not reach the offending part.
Chapter 1: Scope and Overviews 11
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 30

1.2.1.2 Quantitative Restrictions
An implementation may make quantitative restrictions not contained in the standard. For
example, restrictions may be made in the following contexts:
(1) Where syntaxes allow iterative or recursive constructs of arbitrary size. an
implementation may restrict the size of these constructs provided the construct
is not deleted. In particular, a standard implementation may limit the maximum
length of an (identifier), provided it is not less than 31 characters.
(2) The quantity of information existing during translation or execution may be
restricted.
(3) The time permitted for interpretation of a program-run may be bounded.
1.2.1.3 Operatinq Environment
An implementation may make restrictions at the interface with its environment, for
example, in the composition of external identifiers or titles.
An implementation may require appropriate extralingual information in order to execute a
program in conformance with this standard.
1.2.1.4 Expression Evaluation
The operations "evaluate-expression", "evaluate-variable-reference", and "evaluate-
target-reference" define one or more strict orders of evaluation. An implementation may
deviate from these strict orders in the following respects:
(1) Not all of the interrupts raised by any of the strict evaluations need be raised
by an implementation.
(2) The order in which interrupts are raised by an implementation may differ from
the order in which they are raised by strict evaluation.
(3) An implementation may evaluate an <expression>, <value-reference>, <variable-
reference>, or <target-reference> by evaluating its contained ?xpression>s,
<value-reference>s, and <variable-reference>s in any order. Note that this does
not permit the association of operators with their operands to be altered, since
the association is fixed in the tree structure of an <expression> by the
Translator.
(4) If an implementation can determine the result produced by evaluating an
<expression>, <variable-reference>, or <target-reference> without evaluating it,
the implementation need not perform the evaluation. However, if the result
depends on the value returned by a contained <procedure-function-reference>,
then this <procedure-function-reference> must be evaluated.
When determining whether or not an evaluation must be performed, the
implementation may ignore the possibility that this evaluation could raise one
or more interrupts.
1.2.1.5 Interrupts and Assiqnment
The operation "execute-assignment-statement" determines a strict order of evaluation. An
implementation may deviate from one or more strict orders in the number and order in
which it raises interrupts as described in Section 1.2.1.4, parts 1 and 2. When an
implementation exceeds one or more of its implementation-defined limits, it may raise one
or more of the following conditions: overflow, underflow, fixedoverflow, zerodivide,
size, stringsize, stringrange, subscriptrange, storage, area, error.
12
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 31

1.2.1.6 Input/Output
(1)
(2)
(3)
(4)
(5)
(6)
The <environment> attribute and option are provided for the specification of
implementation-defined information concerned with the manipulation of the ffile-
value% and fdataset>s. If present, an <environment> may affect the algorithms
described in this standard by either causing (if necessary) the operation
evaluate-expression to be performed or affecting the correspondence between open
ffile-valuess and fdataset+s. If no <environment> is present, the algorithms
work as given. An <expression> appearing in an <environment> is evaluated at an
implementation-defined point.
If there is more than one <single-opening> in an <open-statement>, an
implementation may deviate from the strict order of execution of the raise-io-
condition(<undefinedfile-condition>,fvl operation performed by the execute-
single-opening operation. In particular, an implementation may defer the raise-
io-condition operation(s) until all other operations in the statement have been
performed. The conditions must be raised in the same order as they would have
been raised had the strict order of execution been followed.
If there is a <copy-option> in a <get-statement>, an implementation may deviate
from the strict order of execution of the output-string-item operation. In
particular, an implementation may interleave the execution of this operation
with the execution of any other operations in the <get-statement> which follow
it. If there are two or more output-string operations, they must be executed in
the same order with respect to each other as they would have been performed
under the strict order of execution, independently of their order with respect
to the other operations of the <get-statement>.
During the execution of any input/output statement which contains a <file-
option> or a <copy-option> an implementation may raise the <transmit-condition>.
Continued execution of the program beyond the point where the condition is
raised may be undefined depending on the use and validity of the data
transmitted by the input/output statement. The <transmit-condition> is raised
by performing raise-io-condition(<transmit-conditiondition>,file-value,key) where
file-value and key depend on the input/output statement. In the event that an
output operation is being executed as a part of file closing during program
epilogue and circumstances are such that the <transmit-condition> is to be
raised, the implementation may perform an implementation-defined action.
A <get-string> must not immediately contain an <expression> that simply contains
a <variable-reference> that identifies an <allocation-unit> that is referenced
by the evaluation of the <input-specification> of the <get-string>.
A <put-string> must not immediately contain a <target-reference> that identifies
an *allocation-unit9 that is referenced by the evaluation of the <output-
specification> of the <put-string>.
Chapter 1': Scope and Overviews 13
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 32

1.2.1.7 On-units
In order to avoid defining certain "side-effects" of <on-unit>s, and to avoid overly
defining the state of the machine upon entry to <on-unit%, a program must satisfy the
following constraints not explicitly enforced by the PL/I machine:
(1) An <on-unit>, or any of its dynamic descendants, entered for the underflow,
conversion, or stringsize condition must not allocate, free, or assign a value
to any <allocation-unit+ used in the block of interrupt, unless the <on-unit>
terminates by executing a <goto-statement>.
Let B be the current &lock-state+ at the time the interrupt was raised; i.e.
the block of interrupt. An <allocation-unit, is used in block B if it is
referenced by any operation of the PL/I machine while B is the current tblock-
state,.
(2) Let B be a *lock-state+; let E be an <executable-unit> executed while B is the
current *block-state+. During the execution of E, various <generation%5 are
returned by invocations of evaluate-tarqet-reference. Let T be the set of
<allocation-unit% designated by these <generation%, i.e., the targets of a
given statement execution.
During the execution of E whenever the "interrupt" operation is invoked, all
tbasic-value% contained in any member of T are set *undefined>. If control
reaches step 4 of the "interrupt" operation, the original &asic-value% are
restored, unless their containing <allocation-unit* was assigned one or more
basic-value% by the <on-unit> or its dynamic descendants. In the case of such
assignment, the <basic-value% of the <allocation-unit> are set <undefined+.
1.2.2 IMPLEMENTATION-DEFINED FEATURES
The PWI language features listed below are termed implementation-defined: their
specification is regarded as completing the definition of the language for a particular
implementation. A brief description of each feature is given, with references in
parentheses to the sections of this document where further details can be found.
(1) Circumstances in which TRANSMIT condition is raised (1.2.1.6).
(2) Actions performed, instead of raising the RECORD, KEY, or TRANSMIT condition, when
an output operation is being executed as a part of a file closing during program
epilogue and circumstances are such that , in all other contexts, the condition would
be raised (X.2.1.6, 8.5.2.3, and 8,.6.6-g).
(3) Determination of the fdataset-list, passed to the 'interpret" operation (1.4.3.3).
(4) Determination of the tentry-value, passed to the "interpret" operation (1.4.3.3).
(5) ENVIRONMENT attribute and option syntax (2.4.4.4) and semantics (Chapter 8).
(6) OPTIONS attribute and option syntax (2.4.4.4) and semantics.
(7) Extralingual characters in data character set (2.5.5 and 2.6..2).
(8) The form of the *text-name) in the INCLUDE construct (2.5.7).
(9) Collating sequence, hardware representations, graphic representations, and symbol
names of an implementation's character set (2.6 and 9.4.4.17).
(10) Default precisions of arithmetic data (4.3.6.3).
(11) Default AREA size (4.3.6.3).
(12) Consistency requirements for ENVIRONMENT and OPTIONS attributes in EXTERNAL
declarations (4.6.1). and far OPTIONS attributes in ENTRY references (6.3.6.1.1).
(13) Size of an farea-value> passed as a dummy argument (6.3.6.1.2).
14
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 33

(14) Information output when SNAP is specified in ON statement (6.4.3).
(151 Value returned by ONCODE builtin function (6.4.3).
(16) Standard system action for STORAGE condition (6.4.4).
(17) Standard system action for ERROR condition (6.4.4).
(18) Form of comment output as standard system action (6.4.4.1).
(19) Situations when ERROR is raised.
(20) Situations when STORAGE is raised (7.2.5).
(21) Use of AREA size specification (7.2.6).
(22) Interpretation of UNSPEC pseudo-variable (7.5.4.8).
(23) Concrete representation of a <dataset+ (8.1).
(24) The "size" of a <record, (8.1.1).
(25) Length of a key (8.1.1).
(26) Representation of stream dataset control items (8.1.2).
(27) Determination of a <dataset+ on file opening (8.5.1.3).
(28) Default LINESIZE for a STREAM OUTPUT file (8.5.1.3).
(29) Default PAGESIZE for a PRINT file (8.5.1.3).
(30) Default tab positions for a PRINT file (8.5.1.3).
(31) Length of file title (8.5.1.5).
(32) Circumstances in which the KEY condition is raised (8.5.2.3, 8.6.2.2, 8.6.3.1, and
8.6.6.10).
(33) Circumstances under which records are written, or not written, when the RECORD
condition is raised and normal return occurs, and the values of those records
(8.6.3.1, 8.6.4.2, and 8.6.6.9).
(34) Raising of RECORD condition by WRITE and LOCATE statements (8.6.6.9).
(35) Position of records in a KEYED SEQUENTIAL file (8.6.6.9).
(36) Items output by 'PUT DATA;" (8.7.2.5).
(37) Maximum <number-of-digits> used in editing relative to a <fixed-point-format>
(8.7.2.6.3).
(38) Maximum <number-of-digits> for each combination of <base> and <scale> (4.4.3.5,
9.1.3.2, 9.5.1.9, and elsewhere).
(39) Precision of integer-type (9.1.3.2).
(40) Determination of floating-point results of expressions and builtin functions
(9.1.4).
(41) Results of ROUND builtin function with floating-point argument (9.4.4.68).
(42) Length of string returned by TIME builtin function (9.4.4.82).
(43) Result of UNSPKC builtin function (9.4.4.85).
(443 Results of numeric conversions (9.5.1.2, 9.5.1.3, and 9.5.1.6).
(45) Number of digits in the exponent of a floating-point number (9.5.1.5).
(46) Representation of currency symbol and digit and sign symbols (9.5.2.2).
Chapter 1: scope and Overviews 15
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 34

1.3 The Metalanguage
Following the introductory material in Section 1.1.4, this section now gives a more
precise and careful definition of the metalanguage.
The definitive part of this document consists of:
a set of production-rules
a set of operations
constraints
attribute definitions and argument names
tables
definitions of terms
together with the section describing the relationship between an implementation and this
mechanized definition.
The metalanguage in which the definition is expressed has three main notational parts, to
be presented later in this section:
a notation for trees, the fundamental type of data in the metalanguage;
a notation for production-rules , which define classes of trees;
a notation for operations, which manipulate trees.
Other definitive material follows headings "Constraints:", 'Attributes:", or
"Arguments:".
Tables are enclosed in a frame of straight lines.
At the point where a new term (other than a syntactic category) is defined, the term is
underlined to indicate this; subsequent uses of the term are not underlined.
Examples, which are not part of the definition, appear in a frame with lines at the sides
and asterisks at the top and bottom. Introductory paragraphs to syntaxes and operations
are likewise without any definitive force.
1.3.1 TREES
Trees are the sole type of data manipulated by the actions of the process defined by this
document. All of the internal operations of th; process use only trees, all of the
inputs to the process from its environment are suitably constructed trees, and all
interpretations of the semantics defined by this process must be in terms of the tree
manipulations performed by it. For uniformity even simple values, such as numbers or
characters, are regarded within the process as single node trees.
In a strict mechanization of the process defined by this document, there could in fact be
only a single tree used to hold the entire ‘state” of the process, and all of the trees
discussed here would be subtrees of this single tree. Since, however, this document
leaves certain informal 'gaps" in its tree definitions, it is also possible to regard the
process as one that operates on a set of independent trees, one for the "state' and
others which are more local to particular phases of the definition.
The general abstract form of trees as employed in this document plus the technical terms
used in discussing trees are defined in Section 1.3.1.1. These trees are then made more
specific by discussing in Section 1.3.1.2 the basic nature of the objects used in
composing tree nodes. In Sections 1.3.1.3 and 1.3.1.4 the written notations used for
individual nodes and then for whole trees are discussed. Section 1.3.1.5 deals with
copies of trees.
Following this general section on trees and tree notations , Section 1.3.2 then discusses
production-rules, which function in a declarative manner to specify the particular
classes of trees used in this document.
16
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 35

1.3.1.1 Tree Definitions
A node is an ordered pair of objects, termed the e of the node, and the unique-name of
the node.
A tree is a finite set of one or more nodes together with some structuring relationships
among these nodes. These relationships are such that:
(1) There is a specified node termed the root-node of the tree.
(2) Excluding this root-node, the remaining nodes (termed the subnodes of the tree)
are divided up into zero or more disjoint sets, each of which in turn forms a
tree. These trees are termed the immediate subtrees of the defined tree.
(3) There is a specified linear ordering among these immediate subtrees.
A tree, X, is said to be a subtree of a given tree, Y, if X is either:
(1) an immediate subtree of Y, or
(2) a subtree of an immediate subtree of Y.
A subtree, X, of a given tree, Y, is said to be a simple subtree
exist a tree, Z, such that:
(1) 2 is a subtree of Y,
(2) X is a subtree of Z. and
of Y if there does not
(3) the root-node of 2 has the same type as either the root-node of X or the root-
node of Y.
Terminology based on the word "contained' is used consistently as follows: subnodes and
subtrees are said to be contained in the given tree, and immediate subtrees and simple
subtrees are said to be immediately contained and simply contained respectively.
Similarly for the word "component", i.e. subnodes and subtrees are said to be components
of the given tree, immediate subtrees and simple subtrees are said to be immediate
components and simple components respectively. The root-nodes of immediate subtrees and
simple subtrees are said to be immediate subnodes and simple subnodes respectively.
The concept denoted by the term "simply contains" is used so frequently in the sequel
that the words themselves are usually elided. Any relational statement between two nodes
that implies containment or possessionr without using explicitly any form of the words
'contain" or 'component", is to be interpreted as implying the simple containment
relation. For example, "A simply contains B" may be expressed as 'A with a B", 'A has
B"* 'the B in the A", or even just *B A". These abbreviated forms may also be compounded
in a single sentence.
Two trees are said to be equal if they contain the same number of nodes and if:
(1) when they each contain a single node, their respective types are the same, or
(2) when they each contain more than one node, the respective types of their root-
nodes are the same, they have the same number of immediate subtrees, and their
respective immediate subtrees, taken pairwise, are equal.
A tree, X, is said to immediately follow a tree, Y, if they are both immediate subtrees
of some tree, 2, and if X is next afterin the linear ordering of the subtrees of Z.
A tree, X, is said to immediately precede a tree Y if Y immediately follows X.
A tree, X, is said to follow a tree, Y, if any of the following is true:
(11 X immediately follows Y.
(2) There is a tree Z such that X follows Z and Z follows Y.
(3) There is a tree Z such that Z contains X and Z follows Y.
(4) There is a tree Z such that Z contains Y and X follows 2.
Chapter 1: scope and Overvievs 17
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 36

A tree, X, is said to precede a tree, Y, if Y follows X.
Note that the above definitions do not define a linear ordering on all of the subnodes of
a tree, just a partial ordering. In particular, any tree which contains another does not
either precede or follow the contained tree.
The words first and last applied to any distinct set of trees have their usual sense of
"have none that precede" and "have none that follow". Although these definitions are not
such as to give a unique first or last tree for some sets of trees, "first" and "last'
will be applied in this document only to sets such that there is a unique result.
Similarly, the next tree following a given tree is aefined in the usual sense of "first
that follows" and=1 also be used in contexts where it is unique.
A node, M, contained in a tree, X, is said to correspond to a node, N, contained in a
tree, Y, if M and N occupy the same ordinal position in the ordered set of irranediate
subtrees of either:
(1) the root-nodes of x and Y, or of
(2) corresponding nodes of x and Y.
(Chapter 9 (see Section 9.1.1.5) contains further definitions of "correspond" useful in
certain special contexts.)
1.3.1.2 Node Objects
The definition of node given in Section 1.3.1.1 leaves undefined the nature of the
objects used for node types and node unique-names. The sets from which these objects are
selected are limited as described in Section 1.3.1.2.1 and 1.3.1.2.2.
1.3.1.2.1 Unique-names
The set of objects which may be employed as node unique-names plays two roles. All node
unique-names are selected from this set, but in addition, some node types may be selected
from this set. Any potentially infinite set of objects which are distinguishable from
the otluer objects used as node types will suffice.
The unique-name component of a node is so called for two reasons. First, at no time a0
two distinct nodes have equal unique-name components. second, no node is ever created
with a unique-name component equal to that of any node which has ever been created. The
unique-name component of a node does not change during the life of the node and thus
serves to identify, or designate, the node (see Section 1.3.1.2.2).
1.3.1.2.2 Types
The set of objects which may be employed as node types is the union of the following
disjoint sets:
(1) The set of cateqorv-names. This is a finite set of the objects employed in
production-rules. This set can be further subdivided into several logically
coherent subsets, each with a definite notational convention.
(2) The set of real numbers, or possibly some implementation-dependent subset of
them which includes the integers. The integers are used throughout for such
purposes as indices and ordinals. Real numbers (including possibly integers)
are used as the values of arithmetic variables.
(3) The set of unique-names, as defined in Section 1.3.1.2.1. A member of this set
used as a node type is termed a desiqnator. Designators are used explicitly for
the purpose of uniquely picking out, or designating, nodes of a tree. A
designator (or a tree containing a single designator) desiqnates exactly that
node which has the same object as unique-name. Note that this construction,
together with the uniqueness rule in Section 1.3.1.2.1, means that it is
possible to examine a designator and constructively determine if the potentially
designated node has been in fact deleted.
18
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 37

As a general rule, the type of a node does not change during the life of the node.
Modifications to the tree occur by removing old nodes and constructing new nodes with new
types and new unique-names. The single exception to this is the replace instruction (see
Section 1.3.3).
1.3.1.3 Node Notation
Throughout the metalanguage, the unique-names of nodes have only an implicit and
essentially invisible function in guaranteeing unique designation and proper subtree
distinction. The metalanguage discussion of nodes is always in terms of their types.
Particular objects from the set of unique-names are never referred to directly, so no
written notation for them is required.
The written notation used for real numbers is just ordinary decimal notation throughout.
In the sequel, context is sufficient to distinguish numbers used as node types from
numbers used for other purposes.
The written notation used for category-names varies, depending on the logical nature of
the use of the particular category-name. These various notations and their meaning is as
follows:
(1)
(2)
(3)
Named Categories. NalIleS formed of lower case letters, hyphens, and numbers,
including surrounding brackets of the form "f" and ")*, I<" and ">", or "f" and
"3" are used as the denotation of some category-names. Optionally the name
exclusive of the brackets may be underlined. In general, mnemonic English words
or abbreviations are chosen to indicate the function of the category-name. In
addition, the three types of brackets indicate whether the category-name
functions primarily in the concrete, abstract, or interpretation phases of the
definitional process respectively. The underlining, if present, indicates that
the category-name occurs only as the type of a node that has no components.
PL/I Characters. The 57 characters of the PL/I language character set are used
as category-names. Two denotations are used. In the great majority of
situations, where no confusion is liable to arise, they are denoted by straight-
forward individual character denotations. Capital letters are used for the
letters, while the quoted symbol (i.e. that which is inside the following
quotes) W' is used for blank. In situations where confusion might arise, the
concrete brackets are used around the straightforward denotations, e.g. C,).
PL/I Keywords. Certain category-names represent PL/I keywords, i.e. selected
sequences of letters or numbers that have particular significance in PWI. Two
denotations are used. In the great majority of situations where no confusion is
liable to arise, they are denoted by a straightforward concatenation of the
denotations for the individual letters or digits that form these keywords in
PL/I, written without intervening spaces on the page. Examples are the quoted
symbols "LIKE" and "FLOAT". In situations where confusion might arise, the
concrete brackets are used around the straightforward denotations.
Nodes which have a type of either of the classes (2) or (3) above are said to possess a
concrete-revresentation , which is a (non-tree) character string composed from any of the
57 PWI language characters. Par these nodes this concrete-representation is just the
simple denotation of the node type, with a blank space substituted for 1. Any possible
subnodes of the category {extralingual-character) are assumed to have a concrete-
representation, each of which is different from that of any PL/I character.
Any tree that satisfies the restriction that all of its nodes which contain no components
have a type which possesses a concrete-representation, may also be said to possess a
concrete-representation. This representation is just the character string formed by
concatenating, in precedence order. the concrete-representations of these nodes.
Chapter 1: scope and Overviews 19
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 38

1.3.1.4 Tree Notation
1.3.1.4.1 Enumerated Trees
A particular tree may be completely specified by stating its root-node and describing
each subnode in terms of immediate components down to the terminal nodes.
This may be expressed more concisely as an enumerated-tree in a notation which specifies:
(1) node type, for the root-node, optionally followed by a comma and a name by
Which this node can be designated (see Section 1.3.1.31,
optionally followed by
(2) a colon,
the immediate components of the root-node (which may themselves be enumerated-
trees, or may be names designating other trees which are to be copied) which may
be enclosed in brackets ("i" and "1') denoting a component that is to be omitted
if and only if its value is <absent+,
and a semicolon.
For example, the tree which consists of a <data-type> which immediately contains a <non-
computational-type> which immediately contains <format> and <local>, can be written as:
<data-type>,dt: <non-computational-type>: <format> <local>;;. Indentation is often used ~ - as a visual aid, e.g.
<data-type>.dt:
<non-computational-type>:
<format>
<local>;;.
Semicolons at the end of an enumerated-tree specification may be omitted immediately
before a period.
Use of an enumerated-tree in the metalanguage indicates the creation of a local-tree (see
Section 1.3.3.1) having the structure and node types indicated, with appropriate copies
inserted of the trees to be copied (see (2) above), and with the designators
corresponding to the names optionally used as in (1) above set to designate the nodes
there created.
A frequently used abbreviation for a particular class of enumerated-trees is to enclose a
potential PWI concrete-representation in double quotes. This is an abbreviation for
that tree rooted in (symbol-list) which has this enclosed string as its concrete-
representation.
1.3.1.4.2 Forms
Patterns to be searched for in trees may be indicated in the metalanguage by a notation
which is the same as that for enumerated-trees, except that the names of trees to be
copied may not be included.
Use of such a notation in the metalanguage is always preceded by the word form. Its use
indicates that a search for, or test of conformance to, the pattern is to be made,
yielding true or false, and that the designators corresponding to the names used as in
(11 of Section 1.3.1.4-l are to be set to designate the nodes corresponding to them if
and only if the search, or test, returns true. Use of brackets in a form indicates the
bracketed components may be either present or absent.
20
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 39

1.3.1.5 Tree Copies
A copy of a given tree is constructed as follows:
(1) Construct a tree which is equal to the given tree.
(2) For each designator node X in the given tree which designates a node Y also in
the given tree, change the constructed node which corresponds to X so that it
designates the constructed node which corresponds to Y.
1.3.2 PRODUCTION RULES
The trees actually employed in this document are a limited subset of all the possible
trees that could be formed according to the definitions given in Section l-3.1.
Production rules serve as the declarative portion of the metalanguage and do so by
specifying restrictions on the forms assumed by the trees used in the definitional
process of this document.
1.3.2.1 Production Rules and Syntaxes
A production-rule is written with an optional label formed of capital letters and digits,
and consists of a category-name, followed by the quoted symbol -::=" and then followed by
either a syntactic-expression (see Section 1.3.2,.3) or. in a few instances, an English
Language phrase. Such a production-rule is termed a defininq production-rule for that
category-name written before the ": :=I. Within this document, there is at most one
defining production-rule for any given category-name.
The basic function of a production-rule is to define a set of possibilities for the
number, type(s), and order of the immediate subnodes of a node whose type is the defined
category-name. This is done by interpreting the syntactic-expression of the production-
rule according to the algorithm given in Section 1.3.2.4.
Production-rules are augmented in their function of specifying immediate subnodes by a
notational convention used for creating lists of repetitive immediate subnodes. This
convention applies to the bracketed category-names whose denotation, exclusive of the
brackets, terminates in the quoted symbols "-list" or "-commalist", and it substitutes
for the explicit appearance of a defining production-rule for such category-names (i.e.
such category-names have no defining production-rule written in this document).
A syntax is any set of production-rules. For example, the set of all of the production-
rules in this document is a syntax. Five (disjoint) subsets of the production-rules in
this document have enough logical coherence that they have been given names, i.e. the
High-level Concrete Syntax. the Middle-level Concrete Syntax, the Low-level Concrete
Syntax, the Abstract Syntax. and the Machine-state Syntax. In order to distinguish these
syntaxes, the production-rules comprising them have been given numbered labels starting
with the quoted symbols "CH", 'CM", "CL", "A", and "M" respectively. The unlabelled
production-rules of this document do not belong to any of these syntaxes.
If a defining production-rule for a category-name occurs in a syntax, then that category-
name is said to be non-terminal with respect to that syntax. Any category-name that
occurs somewhere within the syntactic-expressions of the production-rules of the syntax,
but has no defining production-rule in the syntax, is said to be:
(1) non-terminal with respect to that syntax if its denotation exclusive of the
brackets, ends with "-list", '-commalist", or '-designator', and
(2) terminal with respect to that syntax otherwise.
A category-name that is non-terminal with respect to the syntax composed of all the
production-rules occurring in this document is said to be just non-terminal; similarly
for terminal.
The Abstract Syntax additionally allows constraints to be specified for certain category-
names. The constraint, written in parentheses after the relevant category-name, is
applied by the Translator, or during the interpretation phase, but has no effect on the
constitution of a tree specified by the syntax.
Chapter 1: Scope and Overviews 21
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 40

The production-rule
A17. <bound-pair>::= <lower-bound> <upper-bound> 1 <asterisk>
defines two possibilities, which may be written as (a) or drawn as (b)
(al <bound-pair>: <bound-pair>:
<lower-bound> or <asterisk>;
<upper-bound>;
(b) <bound-pair> <bound-pair>
I or I r ---- ---------- ---,
I I I <lower-bound> <upper-bound> <asterisk>
A node whose type is <identifier-list> may have any non-zero number of immediate
subnodes of type <identifier>, i.e.
<identifier-list>: or <identifier-list>: or <identifier-list>:
<identifier>; <identifier> <identifier>
<identifier>; <identifier>
<identifier>;
and so on.
A node whose type is {parameter-commalist) may have any non-zero number of
Cparameter) immediate subnodes, but with nodes whose type is the PL/I character C,)
interspersed between adjacent ones, i.e.
{parameter-commalist): or {parameter-commalist): or {parameter-commalist):
{parameter)
f.2 {parameter)
f &amet er*;
.- {parameter); - .- *parameter) . .-
&Zrameter*;
and so on.
The production-rule
A67. <repeat-option>::= <expression> (scalar)
specifies that only <expression>s yielding scalar values
values) are valid immediate subnodes of a <repeat-option>.
(i.e. not aggregate
C+++tt**+***t****+**++++++*++*++****************************************************************
Example 1.2. Examples of Syntax.
1.3.2.2 Complete and Partial Trees
Given a syntax, a complete tree with respect to that syntax is any tree which can be
obtained by starting from a node of a given type, and repeatedly attaching subnodes to
the nodes of the tree being developed according to the algorithm of Section 1.3.2.4,
until an interpretation has been obtained for every node of the tree. A complete tree
with respect to the syntax composed of all the production-rules occurring in this
document is said to be just a complete tree.
22
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 41

A partial tree is any tree which is not a complete tree but which can be obtained by
deleting some nodes from some complete tree.
The trees utilized by the definition process of this document are only complete trees or
partial trees. Other possible forms of trees are never utilized. Furthermore, it is the
usual case that complete trees are utilized, or at least utilized at the interfaces
between the various operations of the definition. Although there are a few specific
exceptions, it is a general rule that partial trees occur only in a very local context in
the process of building up a complete tree. Several of the "instructions" (see Section
1.3.3.4) of the metalanguage are in fact designed to assist in the process of building
complete trees.
1.3.2.3 Syntactic-expressions and Syntactic-units
Given a syntax, a syntactic-expression is defined to be either a single syntactic-unit,
or several syntactic-units any of the adjacent pairs of which is possibly separated by a
q " or a Ima. I The symbols are called the or-symbol and the bullet respectively.
Given a syntax, a syntactic-unit is defined to be one of the following:
a single category-name,
a syntactic-expression enclosed in the brackets "C" and "I", or
a syntactic-expression enclosed in the braces "<- and -1".
1.3.2.4 Application of the Production Rules
Given a syntax and a category-name, the algorithm shown just below obtains a (possibly
empty) ordered set of category-names , termed here an interpretation with respect to the
given syntax of the given category-name. In the process of construag a complec tree
with respect to the given syntax, any such ordered set may then be used as the
corresponding types of an ordered set of immediate subnodes connected to any node whose
type is the given category-name.
An interpretation of a category-name is defined as follows:
Case 1. The denotation of the given category-name, excluding any terminating bracket.
ends with "-list".
An interpretation consists of an ordered set of any non-zero number of instances
of that category-name whose denotation is obtained by deleting the "-list" from
the denotation of the given category-name.
Case 2. The denotation of the given category-name, excluding any terminating bracket.
ends with "-commalist".
An interpretation consists of an ordered set which:
(1) contains any non-zero number, nr of instances of the category-name whose
denotation is obtained by deleting the "-commalist" from the denotation
of the given category-name, and which
(2) contains n-l instances of the category-name C,), and which
(3) is arranged so that no two instances of the same category-name are
adjacent.
Case 3. The denotation of the given category-name, excluding any terminating bracket,
ends with "-designator".
An interpretation is a single member of the set of unique-names. If the
category-name is of the form 'x-designator', the unique-name must be of that of
a node of type "x".
Chapter 1: Scope and Overviews 23
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 42

Case 4. There is in the given syntax a defining production-rule for the given category-
name.
An interpretation is an interpretation of
following the I::="
the syntactic-expression written
in the defining production-rule.
Case 5. (Otherwise).
The given category-name is a
interpretation is the empty set.
terminal with respect to the given syntax: the
An interpretation of a syntactic-expression is defined as follows:
case 1. The syntactic-expression is a syntactic-unit.
Case 1.1. The syntactic-unit is a single category-name.
An interpretation consists of the ordered set containing just this single
category-name.
Case 1.2. The syntactic-unit is a syntactic-expression enclosed in the brackets "Co
and "I".
An interpretation consists either of an interpretation of the enclosed
syntactic-expression, or of the empty set.
Case 1.3. The syntactic-unit is a syntactic-expression enclosed in the braces "C" and
"I".
An interpretation consists of an interpretation of the enclosed syntactic-
expression.
Case 2. The syntactic-expression is a sequence of two or more syntactic-units possibly
separated by a "1" or a "o" .
Case 2.1. An or-symbol occurs as at least one such a separator.
Consider all or-symbols occurring thus in the given syntactic-expression to
partition it into a sequence of inner syntactic-expressions. An
interpretation is one of any of these inner syntactic-expressions chosen
arbitrarily.
case 2.2. A bullet occurs as such a separator and an or-symbol does not.
Consider all bullets occurring thus in the given syntactic-expression to
partition it into a sequence of inner syntactic-expressions. An
interpretation is the same as one of a syntactic-expression formed by
arranging these inner syntactic-expressions in an arbitrary order and
omitting these bullets.
Case 2.3. (Otherwise).
(The syntactic-expression is a sequence of syntactic-expressions optionally
separated by blanks.) An interpretation consists of the concatenation, in
order, of interpretations of the syntactic-expressions of the sequence.
1.3.3 OPERATIONS
The procedural part of the metalanguage provides for the writing of algorithms termed
aerations.
&unatical
These are expressed in a semi-formal programming language which uses the
flexibility of ordinary English prose, while at the same time attaching
precise meaning to certain words and phrases, in order that the flow of control and the
tree manipulations in the operations be well defined. Completely formal notation is used
to describe trees in accordance with their syntactic definitions.
24
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 43

1.3.3.1 Nature of an Operation
An operation, applied to zero or more operands, may be performed by the processor (see
Section 1.3.4). with the effect of:
(1) changing the <machine-state>, or
(2) changing an operand, or
(3) returning a result, or
(4) any combination of cl), (21, and (31
Within an operation either operand-names or local-variable-names may be used for
accomplishing this effect. Thesenames serve as designators of trees, which may be
either portions of the fmachine-state,, or local-trees created within this or another
operation.
When an operation is performed, its operand-names are set to designate the operand trees
it has been passed. It then has the following data available to it:
(1) The whole fmachine-state+, which is directly accessible for inspection or change
at any time.
(2) The operands which it has been passed, which can be inspected or changed.
(3) Local-trees local to itself, which it can freely construct, inspect, or modify.
These trees are deleted when the operation terminates.
The operation may also apply any operation defined in this document (including itself) to
any operands which it may select from among the trees available to it.
Upon completion of its actions, an operation may return a result, which then becomes
available to whichever operation applied the given operation to its operands. As with
operands, this result may be selected from among the trees available to the operation.
In the case that (a portion of) a local-tree local to this operation is selected, this
tree is not deleted, but is copied to become a local-tree local to the applying
operation. (See Section 1.3.3.4 on "perform".)
1.3.3.2 Nondeterministic Operations
The phrases "optionally" and "in any order" are used in some operation descriptions.
They indicate that the processor is to make a choice each time that part of the operation
is executed. In general, then, the conceptual PL/I machine defines a set of possible
interpretations for a program.
1.3.3.3 Format of an Operation
The written description of an operation has a format consisting of a heading and a body.
The heading may have three parts:
(1) There is always a specification of the form "Operation:" followed by the
underlined name of the operation, optionally followed by a parenthesized list of
the names used to refer to the operands passed to the operation, the names being
separated by commas.
(2) For an operation which has operands, the word -where* then precedes a
description of the type(s) of tree to which each operand name may refer.
Brackets around a node type indicate that the operand is optional, which is an
abbreviation for stating that it may alternatively have the value <absent+.
(3) For an operation which returns a result, a final part of the heading is of the
form "result:" followed by a description of the type(s) of tree which may be
returned.
Chapter 1': Scope and Overviews 25
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 44

Operation: evaluate-in-option(al,vrI
where al is an <allocatiorD,
vr is a <variable-reference>,.
result: a *generation+.
Example 1.3. An Example of an Operation Returning a Result.
The body of an operation consists of either a Step-list or a Case-set. Each Step or Case
is a numbered section containing written instruction descriptions of arbitrary
complexity, and may itself contain's Step-list or Case-set, numbered with an additional
index position and indented to indicate this containment.
Steps are normally performed sequentially.
Each Case consists of a condition part followed by an executable part. Within each case-
set, the set of condition parts is such that, at any execution point, exactly one of the
condition parts is satisfied. (The conventional condition part "(Otherwise)" is satisfied
whenever none of the other condition parts of the case-set is satisfied.) The execution
of a case-set consists of executing the executable part of the one case whose condition
part is then satisfied.
The normal sequence for Steps and Cases may be modified by explicit instructions using
such terms as ‘go to' or "perform" which are defined in Section X.3.3.4. The terminology
for selecting some actions, either in a defined left-to-right order or in an unspecified
order, is "in left-to-right order" and "in any order" respectively. When selection is
between two options, "in either order" may be used instead of "in any order". Optional
selection as to whether an action will be carried out is indicated by words such as
"optionally perform".
~***+*+*+*+**++**+*+**++++******+***********************************************************
Operation: every-bif(rdd,x)
step 1. Perform evaluate-expression(x) to obtain an <aggregate-value9,u.
step 2. In any order, convert each scalar-element of u to Kbb>, to obtain v.
step 3.
Case 3.1. Every scalar-element of v that does not contain fnull-bit-strinqs has
a fbit-string-value+ with every <bit-values containing *one-bit,.
Let r be <one-bie.
Case 3.2. (Otherwise).
Let r be <zero-bit,.
step 4. Return an <aggregate-value> containing a <bit-string-value, containing r.
~**+**+*+**+***+*++****+**+*+*+++++*++*+*************~*************~*******************
Example 1.4. An Example of an Operation Ccntaining a Step-list and a Case-set.
26
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 45

1.3.3.4 Instructions
An instruction is a specification of some action involving the creation, destruction,
inspection, or modification of soxe tree(s), or causing some departure to be made fror
the normal sequential flow of control through an operation.
Operand-names and local-variable-names strictly dencte designators of trees rather than
tree values themselves; however, apart from their use in a context where they acquire
designator values, are passed as OFerandS, or are returned as results, references to
these names are always taken to be an abbreviation for references to the tree designated
by the strict value.
The "let" instruction is used to indicate that the named variable is henceforth to
reference the specified tree , which may be an existing tree or one newly created, e.g. by
use of the enumerated-tree notation or by copying an existing tree. Use of a name for a
tree following some description of the root-node and a comma, e.g. in the enumerated-tree
notation, is equivalent to use of a "let" instruction. NO change to any tree previously
designated by the named variable or operand occurs as a result of the "let" instruction.
In contrast, the "replace" instruction is used to indicate that the tree referenced by
the named variable or operand is to be replaced with the specified tree. The replacement
occurs exactly at the root of the referenced tree, and the named variable or operand
henceforth references the replacement.
The "append" instruction, as in "append b to c", indicates that b is to be attached as
the last immediate subtree of c. c may be any existing tree, or if it is uniquely
specified, may be non-existent. This latter case causes the construction of the (single)
node c- In order to make the specification of the potentially missing node c unique. the
notation illustrated by "append b to c in d" can be employed to indicate that c may be
missing and is to be constructed as a simple component of d.
The "attach" instruction, as in "attach b to c", indicates that b is to be attached to c
as a component of c. If b can be an immediate component of c, then it is attached as an
immediate component. Otherwise, there will be a unique way that a node of type b can be
a simple component of c, and exactly the minimal necessary nodes which are both contained
by c and also contain b are created so as to attach b as a simple component.
The "delete" instruction, as in "delete b" indicates that b is to be removed from its
containing tree (and discarded). In addition, if b is a mandatory component of the tree
which immediately contained b, say C# then the "delete" instruction is applied to c
(i.e., c is discarded and the process continues with the tree which immediately contained
cl.
The "perform" instruction either calls another operation, possibly passing operands,
possibly receiving a result, or calls for the execution of some Steps with an operation
out of the normal sequence. In calling other operations, references to trees specified
by the argument list are passed to the named operation as operands (any missing arguments
are given the value <absent> in the called operation). If the operation returns a
result, then the term "toxn" is used to indicate the obtained result. When other
Steps in an operation are performed, control returns to the instruction following the
"perform" instruction.
The "go to" instruction indicates that the normal sequence of control is broken and that
the named step is to be executed next.
The "terminate" instruction indicatgs that the execution of the current operation is to
be terminated and control is to be returned to the calling operation. If control reaches
the end of an operation that does not return a result, an implicit "terminate"
instruction is assumed. The "return" instruction indicates that the current operation is
to be terminated with a reference to a specified tree as the result.
The "if" instruction indicates that if the specified condition is true then the
instruction list following the "then" is executed and the instruction list (if any)
following the "otherwise" is skipped. If the condition is false the instruction list
following the "then" is skipped and, if there is one, the instruction list following the
"otherwise" is executed.
The "for each" instruction specifies actions that are to be carried out once for each
member of a set of objects, in some sequence which may be in any order or in some
specified order.
Chapter 1:' Scope and Overviews 27
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 46

The 'must9 instruction specifies a test to be performed. If the test is not satisfied,
the program (when combined,with the particular initial entry-point and datasets if the
interpretation-phase has begun) is rejected by the standard, and the conceptual PVI
machine stops. This is the only sense in which "must" and “must not" are used in
operation descriptions.
1.3.3.5 Convert
Convert is an exception to the general rules for naming and performing (see Section
1.3.3.41. Use of this operation is generally specified in an informal style, e.g.
'convert the value of the <expressio@,x to integer-type'. Details are given in Section
9.5.1.1.
1.3.3.6 Additional Notational Conventions
Common mathematical symbols are used with their usual meaning. In addition, the
following notational conventions are used:
l denotes multiplication;
/ denotes division;
t denotes exponentiation;
E
denotes iterated addition (summation); the result is taken as zero if the
iteration range is empty;
l-r denotes iterated multiplication (product); the result is taken as one if
the iteration range is empty;
II denotes subscripting in the metalanguage as a notational convenience used
with local names;
F& denotes the mathematical constant of that name;
e denotes the mathematical constant of that name;
2 denotes the square root of -1;
ceil(x) denotes the smallest integer larger than or equal to x;
floor(x) denotes the largest integer smaller than or equal to x;
min(x,y) denotes the value of x if xly. otherwise the value of y;
max(x,yl denotes the value of x if xLy, otherwise the value of y;
log(x) denotes the natural logarithm of x;
1x1 denotes the absolute value of x.
1.3.3.7 Arithmetic
In the metalanguage, an arithmetic expression denotes the exact mathematical value. The
situations in which the result may be approximated by the PVI machine are defined in
such operations as arithmetic-result.
In general the distinction between numbers and trees containing them is ignored. For
example, if z is a tree of the form
*basic-value+:
*complex-value,:
freal-number+,x
<real-number+,y;;
then the arithmetic expression "z+l* denotes the complex number (x+l)+iy.
28
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 47

1.3.4 THE PROCESSOR
The processor is the active agent capable of performing various actions on the tmachine-
state, tree. These actions are carried out as directed by the written algorithm which
comprises the PWI definition.
A portion of the tmachine-state+ tree, either the icontrol-state, or a tstatement-
control,, holds the information which controls the processor. An *operation% is known by
the processor if it is a simple component of the <control-state+ or a fstatement-
control,. There is at most one active operation. An <operation, is active if:
(1) it is the last member of its immediately containing *operation-lists, and
(2) either:
(2.11 the <operation* is simply contained in an <operation-list+ which is simply
contained in a <statement-control+ which is simply contained in a fblock-
state, that is the last member of its immediately containing fblock-state-
list>, or
(2.2) there are no <statement-control% contained in the fmachine-state), and the
<operation+ is simply contained in an <operation-list+ which is simply
contained in the <control-state>.
(See Section 1.4.1 and Section 5.1 for the definitions of these category-names.)
The processor carries out actions as follows:
Whenever there exists a known active <operation+, the processor carries out the
actions specified by the written description of the corresponding operation.
Whenever there exists no known active <operation,, the processor does nothing.
1.3.5 MECHANIZATION OF TEE METALANGUAGE
A deeper understanding of the metalanguage may be obtained by considering how its formal
mechanization may be carried out as an extension of the <machine-state,. Each
ioperatioti in an <operation-list* may be given a subtree containing the name of the
operation, the names and designator values of its operands, the names and designator
values of its local variables, the local-trees constructed during the performance of this
operation, and control information indicating which step or Case is currently being
executed, which members of an iterative "for each" have still to be performed, and so on.
When a "perform" instruction causes a new operation to be invoked, a new *operation+ tree
will be appended to the <operation-list+ and activity in the present operation will be
suspended. On return from an operation, any local-tree returned by it as a result is
copied back as a subnode of the preceding <operation* tree, and the <operation, tree for
the terminating operation is deleted together with all its local information. Activity
in the invoking operation is then resumed from the point at which it was suspended.
Chapter 1': Scope and Overviews 29
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 48

During the translation-phase, the only <operation-list> is the one in the tcontrol-
state+. During most of the interpretation-phase , the +operation% in the tcontrol-
state+ will be dormant while there is an active *operation, in a *statement-
control5 as shown below.
<machine-state9
i
-------m-----e- +--e----------i
<program> *control-state* ainterpretation-state9
I I *operation-list+
I I r'--"'-"--', I
I I . . . foperatio* I (DORMANT)
/
i <program-state+
I r-------------~---------‘7
I I I <block-state-list,
I
I
I . . . I *block-state>
I
i i i *block-control*
I r----------t-----------,
I I I <statement-control*
I *operation-list,
I r”-“-““-----J
I I . . . *operation+
(ACTIVE)
Example 1.5. Example Showing *operation-list+s of a *machine-state*.
30
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 49

1.4 Initialization of the Machine-state
1.4.1 THE MACHINE-STATE
Ml. fmachine-state>::= <program>
fcontrol-state,
Iftranslation-state9 1 <interpretation-state+]
M2. <control-state*::= <operation-list+
M3. <translation-state+::= [{concrete-external-procedure*1
M4. {concrete-external-procedure)::= [{declaration-commalist) {procedure)
M5. 4operation9 :I=
The exact structure of <operation, is left unformalized and unspecified. It must
have adequate structure and capacity to represent the carrying out of the actions of
operation. This
$sition within it,
includes designating the particular operation and the current
holding the operands given to the operation, and holding the
values of any variables used by the operation (see Section 1.3.5).
The definitions of $declaration) and {procedure$+ are given in Chapter 2; the definition
of <program> is given in Chapter 3; the definition of <interpretation-state, is given in
Chapter 5.
1.4.2 INITIALIZATION
The PL/I definition process begins by creating an initial Unachine-state, tree,
consisting of:
fmachine-state,:
<program>
<control-state+:
*operation-list>:
&operation+ for define-program.
The processor then performs the *operation, for define-program.
1.4.3 THE TOP-LEVEL OPERATIONS
1.4.3.1 Define-vroqram
Operation: define-program
step 1. Perform translation-phase.
step 2. Perform interpretation-phase.
Step 3. No action. (Reaching this point indicates the successful completion of the
definition algorithm.)
Chapter 11 Scope and Overviews 31
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 50

1.4.3.2 Translation-phase
Operation: translation-phase
Step 1. Append ftranslation-state9 to the fmachine-state,.
Step 2.
step 2.1. Obtain, from a source outside this definition, a sequence of characters
composing a putative PL/I external procedure, constructed in the form of a
{symbol-list),sl.
step 2.2. Perform translate(s1) to obtain an <abstract-external-procedure>,aep.
Append aep to the <abstract-external-procedure-list> in the <program>.
step 2.3. Optionally go to Step 2.
Step 3. Perform validate-program.
Step 4. Delete the *translation-state,.
1.4.3.3 Interpretation-phase
Operation: interpretation-phase
step 1. Obtain, from a source outside this definition, the following items:
(1) A collection of information to be used for input/output, constructed in the
form of a suitable fdataset-list9,dl.
(2) A designation, as the first to be activated, of one of the <entry-point>s of
a <procedure> simple component of <program>, constructed in the form of a
suitable fentry-value*,ev. Such an <entry-point> must exist and must not
have <parameter-name-list> or <returns-descriptor> components.
Step 2. Perform interpret(d1,ev.l (see Section 5.3.1).
32
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 51

Chapter 2: Concrete Syntax
2.0 Introduction
The Concrete Syntax of PI& is specified mainly by means of production-rules using the
notation defined in Chapter 1. The first such rule defines a qprocedure?, and subsequent
rules define the permitted forms of a {procedure) and its components in increasingly fine
detail, until every component is ultimately described in terms of sequences of characters
of the language character set.
2.1 The Intent of this Definition
As the first stage of translation (Chapter 41, any given sequence of symbols is parsed to
determine whether that sequence indeed represents a Cprocedure? valid according to a set
of rules known in this document as the "Concrete Syntax".
2.1.1 CONCRETE AND ABSTRACT SYNTAXES
This formal Concrete Syntax is permissive in the sense that some of the constructs
permitted are not actually valid $procedure)s. Thus, for example, the sequence of
symbols "DCL X FLOAT FIXED;" is a syntactically correct construct that may be parsed as a
{declare-statement). Errors of this sort will be detected later in the translation,
because of a failure to satisfy the Abstract Syntax (Chapter 3).
2.2 Organization of the Concrete Syntax
The rules of the Concrete Syntax, which are context-free, are arranged in three levels,
so that two context-dependent features of this grammar, namely the presence of blanks and
comments, and the so-called "multiple closure', may be resolved at the interface between
the levels.
The three levels of syntax correspond to the three levels of the parse algorithm
described in Chapter 4.
2.3 The High-level Syntax of PL/I
2.3.1 PROCEDURE
CHl. $procedure)::= $prefix-list) sprocedure-statement) [{unit-list)3 Cending)
2.3.2 UNIT
criz. {unit)::= C@tatement-name-list?1 i$declare-statement2 } {default-statement)1 1
{statement-name-list) *entry-statement) 1
*prefix-list) *format-statement) 1
{procedure) 1
{executable-unit?
Chapter 2: Concrete syntax 33
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 52

2.3.3 EXECUTABLE UNITS
CH3.
CH4.
CHS I
CH6.
CH7.
CH8.
CH9.
CHlO.
{executable-unit)::= [{prefix-list)]
Cfgroup) 1 @begin-block) 1
{on-statement) 1 Tif-statement) 1
{executable-single-statement))
*if-statement)::= fif-clause* Csexecutable-unit? 1
{balanced-unit? ELSE Cexecutable-unit23
Cbalanced-unit)::= [*prefix-list)1
{{executable-single-statement+ I {group) I Cbegin-block+ I
{on-statement) I
$if-clause> Cbalanced-unit* ELSE {balanced-unit))
<group)::= {do-statement) C-it-list)3 $endingg
{begin-block)::= *begin-statement) [*unit-list)] {ending)
{on-statement)::= ON {condition-name-commalist) [SNAP1 {{on-unit) I SYSTm;l
*on-unit)::= [({condition-prefix-commalist)):l
Csexecutable-single-statement) I {begin-bloc-3
(ending)::= [(statement-name-list?] <end-statement)
Example 2.1 illustrates the tree of the high-level structure of a simple {procedure).
Example 2.2 illustrates the high-level structure of a simple $if-statement).
c***********************+***********+***********************************************
{procedure)
I r---------‘--“------ -------------------‘-‘--‘--------------,
{prefixllist) {procedure!statement) I $.unit-list*
$pr!fix) I
$endbg+
r------'-----, {end-statement)
I I Cunit) sunit*
(declare-Statement) I
I <executable-unit)
I Cexecutable-single-statement)
A sample concrete representation having the structure illustrated above:
T: PROCEDURE;
DCL PI INIT (3.14159);
PUT LIST (SIN(PI18));
END T;
Example 2.1. The High-level Structure of a Simple {procedure).
34
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 53

{executable-unit?
I
$if-sta!ement)
c-‘---‘--‘-----‘---‘--‘---‘-----------------------------------,
I I t I *if-clause) Cbalanced-unit) ELSE {executable-unit)
I I
I I sexecutable- {executable-
single-statement) single-statement)
A sample concrete representation having the structure illustrated above:
L****t****t***+*+**********+*+*++*************************************************
Example 2.2. The High-level Structure of an *if-statement>.
4
2.4 The Middle-level Syntax of PL/I
2.4.1 SENTENCE
A Csentence) in the middle-level syntax corresponds to that which comes between semi-
colons in a PL/I Cprocedure).
CM1 . {sentence)::= [*prefixed-clause-list)3 $single-statement? 1 Celse-part)
CM2. (else-part)::= ELSE fsprefixed-clause-list21 {single-statement2
CM3. {prefixed-clause)::= [{prefix-list)]
C$if-clause) 1
ON (condition-name-commalist? [SNAP11
CM4. {prefix)::= (*condition-prefix-commalist)): I {statement-name)
CM5. {if-clause)::= IF $expression) THEN
Chaptek 2: Concrete syntax 35
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 54

2.4.2 STATEMENT
CM6. <single-statement)::=
Note: funmatched)
Csstatement-name-list)3
{{declare-statement? 1 {default-statement) 1
{end-statement) 1 1
sstatement-name-list) {entry-statement? 1
[{prefix-list)1 {{executable-single-statement21
*begin-statement) CCunmatcheQl 1
fdo-statement) [{unmatched)1 I <format-statement) I
SYSTEM ; I {procedure-statement? [funmatchedpll
is used only by the operation high-level-parse.
CM7. {executable-single-statement?::= {allocate-statement?
(assignment-statement* i
{call-statement)
fclose-statement)
{delete-statement)
*free-statement)
{get-statement)
Cgoto-statement*
Clocate-statement)
#null-statement*
{open-statement* I
{put-statement*
{read-statement? I {return-statement) 1
{revert-statement) 1
{rewrite-statement) I
Csignal-statement* 1
{stop-statement+
Cwrite-statementp
2.4.3 PREFIXES
2.4.3.1 Condition Prefixes
CM8. Ccondition-prefix+::= {computational-condition? I
{disabled-computational-conditio*
cM9. (computational-condition?::= CONVERSION I FIXEDOVERFLOW I OVERFLOW 1 SIZE 1
STRINGRANGE I STRINGSIZE I SUBSCRIPTRANGE 1
UNDERFLOW I ZERODIVIDE
CMlO. {disabled-computational-condition)::= NOCONVERSION I NOFIXEDOVERFLOW I
NOOVERFLOW I NOSIZE I NOSTRINGRANGE I
NOSTRINGSIZE I NOSUBSCRIPTRANGE I
NOUNDERFLOW I NOZERODIVIDE
2.4-3.2 Statement Name Prefixes
CMll. (statement-name)::= (identifier) [({signed-integer-commalist))l:
CM12. (signed-integer)::= C+ I -1 {integer?
2.4.4 DATA DECLARATION
CM13. {declare-statement)::= DECLARE {declaration-conunalisy;
CM14. *declaration)::= C$level)l {$identifier+ I ({declaration-commalist~))
[(dimension-suffix)1 [{attribute-list)]
CMl5. (level)::= (integer)
36
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 55

2.4.4.1 Dimension Attribute and Dimension Suffix
CM16. Cdimension-attribute)::= DIMENSION [{dimension-suffix)]
CM17. {dimension-suffix)::= ({bound-pair-commalist))
CM18. sbound-pair)::= [{lower-bound+:] {upper-bound? 1 *
CM19. flower-bound)::= Cextent-expressionp
CM20. {upper-bound*::= Textent-expressionp
CM21. Cextent-expression)::= {expression+ [{refer-option+1
CM22. Trefer-option>::= REFER ({unsubscripted-reference))
2.4.4.2 Attributes
CM23. {attribute)::= Cdata-attribute2 I KEYED
AUTOMATIC 1 LIKE sunsubscripted-reference) i
EASED [($reference)ll 1 LOCAL
BUILTIN I CONDITION 1 y&g=*
CONSTANT 1 PARAMETER I CONTROLLED 1 POSITION t($expressio~ll I DEFINED {{reference> I
(Creference))} I PRINT
DIRECT I RECORD
Cenvironment) I SEQUENTIAL i
EXTERNAL I STATIC
(generic-attributep 1 STREAF I {initial) I UPDATE
INPUT I VARIABLE I
INTERNAL
2.4.4.3 Data Attributes
CM24. {data-attribute)::= ALIGNED I LABEL I AREA [({area-sizepll I MEMBER
BINARY [{precisionpI 1 NONVARYING I BIT [((maximum-IenqthP)l 1 OFFSET [(+ceference*ll 1
CHARACTER t(+naxim&iength)ll i PICTURE [jpicture)l- j
COMPLEX CKnrecision>l 1 POINTER I
DECIMAL [&ecision+l i PREcIsIoN C$precision)li
Fdimension-attribute) I REAL I$precision)l I ENTRY [([{description-
commalist~lll I Creturns-descriptor)
FILE 1 STRUCTURE I
FIXED [$precision)l 1 UNALIGNED
FLOAT [({number-of-digits911 I VARYING I
FORMAT
CM25. farea-size*::= {extent-expression) I +
CM26. {precision)::= (Number-of-digits) [,$scale-factor?31
cM27. fnumber-of-digits)::= {integer)
CM28. {scale-factor)::= {signed-integer)
CM29. $maximum-length*::= {extent-expression> I *
CM30. {description)::= [$level)l [{dimension-suffix)1 [{data-attribute-lisql
Constraint: At least one subnode must be present.
CM31. {picture*::= fsimple-character-string-constant?
Chapter 2: Concrete Syntax 37
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 56

2.4.4.4 Environment and Options
CM32. {environment*::= ENVIRONMENT ({environment-specification>)
CM33. {environment-specification?::=
CM34. {options)::= OPTIONS (*options-specification))
CM35. *options-specification*::=
2.4.4.5 Generic
CM36. $generic-attribute)::= GENERIC [({generic-element-commalisy)l
CM37. Cgeneric-element)::= <reference) WHEN ([{generic-description-commalist)
CM38. {generic-description)::= [{level)] [{asterisk-bounds21
Csgeneric-data-attribute-list)] 1 *
Constraint: At least one subnode must be present.
CM39. $generic-data-attribute)::= ALIGNED
AREA
BINARY [{generic-precision)1
BIT
CHARACTER
COMPLEX [(generic-precision)1
DECIMAL [{generic-precision+1
DIMENSION fasterisk-hounds?
ENTRY [([{description-
commalis~l)l
FILE
FIXED [{generic-precision)]
FLOAT [{generic-precisioel
FORMAT
1 LABEL
1 MEMBER
1 NONVARYING i 1 OFFSET
1 PICTURE Tpicture)
I POINTER 1 I PRECISION sgeneric-
precision) I
I REAL [(generic-
precision)1 I
I weturns-descriptor) I
I STRUCTURE
I UNALIGNED I 1 VARYING I
CM40. {asterisk-bounds)::= ({*-commalist))
CM41. sgeneric-precision)::= (snumber-of-digits) C:$number-of-digits)]
C,+cale-factor) C:$scale-factor)111
2.4.4.6 Initial
CM42. *initial)::= INITIAL [({initial-element-commalist))l
CM43. {initial-element)::= * I {parenthesized-expression? I
({iteration-factorpl [{initial-constant-two) I * I
(sinitial-element-commalist+ I
(initial-constant-one+
CM44. {initial-constant-one)::= Ifprefix-operator)] {simple-string-constant) I
Cinitial-constant-two)
CM45. (initial-constant-two)::= [{prefix-operator)1
{{reference) I {replicated-string-constant) I
{arithmetic-constant0) I
[+ I -1 Qeal-constant) C+ 1 -1 Cimaginary-constant)
CM46. *iteration-factor)::= Cexpression)
38
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 57

2.4.4.7 The Default Statement
CM47. {default-statement)::= DEFAULT {Cdefault-specification) 1 NONE 1 SYSTEM);
CM48. {default-specification)::= (<predicate-expression?)
{ERROR 1 {default-attributes-commalist))
CM49. {predicate-expression)::= {predicate-expression-three) 1
Cpredicate-expression) $1) {predicate-expression-three)
CMSO. <predicate-expression-threea::= Cpredicate-expression-two* 1
Cpredicate-expression-three) 8
{predicate-expression-two?
CM51. <predicate-expression-two)::= {predicate-expression-one* 1
-. Cpredicate-expression-two)
CM52. {predicate-expression-one::= ({predicate-expression)) 1 {attribute-keyword) 1
(range-specification)
cr453. *ange-specification?::= RANGE ({*identifier) I Qetter) : {letter) I l )I
CM54. @attribute-keyword)::= EpED
AUTOMATIC
BASED
BINARY
BIT
BUILTIN
CHARACTER
COMPLEX
CONDITION
CONSTANT
CONTROLLED
DECIMAL
DEFINED 1 INTERNAL
DIMENSION 1 KEYED
~ DIRECT I LABEL
ENTRY I LOCAL , ENVIRONMENT I MEMBER
EXTERNAL 1 NONVARYING
FILE 1 OFFSET
FIXED I OPTIONS
FLOAT 1 OUTPUT
FORMAT 1 PARAMETER
GENERIC I PICTURE
INITIAL I POINTER
, INPUT I POSITION
CM55. *default-attributes)::= {attribute-list?
I PRECISION
1 PRINT
I R= I RECORD
I RETURNS
I SEQUENTIAL
I STATIC
I s- I STRUCTURE
1 UNALIGNED
I UPDATE
I VARIABLE
1 VARYING
2.4.5 THE PROCEDURE STATEMENT
cM56. {procedure-statement*::= PROCEDURE [(entry-informational;
CM57. *entry-information)::= [({parameter-name-commalist))l
{[<returns-descriptorJ+l l C$options~l l [RECURSIVE11
CM58. {parameter-name)::= *identifier)
cM59. (returns-descriptor*::= RETURNS [({description-commalist))l
2.4.6 THE ENTRY STATEMENT
cM60. {entry-statement)::= ENTRY I$entry-information)l;
2.4.7 THE BEGIN STATEMENT
CM61. @&n-statement)::= BEGIN I$options)l;
Chapter 2: Concrete Syntax 39
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 58

2.4.8 THE DO STATEMENT
CM62. {do-statement)::= DO; 1 DO {while-option); 1 DO {do-spec);
CM63. {do-spec)::= {reference) = fspec-commalist+
CM64. $spec)::= {expression) C$to-by) 1 {repeat-option)1 [{while-option*1
CM65. {to-by): := {to-option? C$by-option11 1 Cby-option? [{to-option)]
CM66. {to-option)::= TO $expression$
CM67. *by-option)::= BY Cexpression)
CM68. {while-option)::= WHILE (CexpressiorQ)
CM69. $repeat-option+::= REPEAT $expression)
2.4.9 THE END STATEMENT
CM70. {end-statement)::= END C$identifierpl;
2.4.10 FLOW OF CONTROL STATEMENTS
2.4.10.1 The Call and Return Statements
CM71. $call-statement)::= CALL {reference?;
CM72. freturn-statement)::= RETURN [($expression))l;
2.4.10.2 The Go To Statement
CM73. {goto-statement)::= {GOT0 1 GO TO3 {reference);
2.4.10.3 The Null Statement
CM74. {null-statement)::= ;
2.4.10.4 The Revert and Siqnal Statements
CM75. Crevert-statement)::= REVERT Ccondition-name-commalist);
CM76. {signal-statement)::= SIGNAL {condition-name);
CM77. {condition-name)::= {computational-condition) 1 marned-io-condition) I
{programmer-named-condition) I AREA I ERROR I
FINISH I STORAGE
CM78. {named-io-condition?::= {io-condition) ((reference))
CM79. {io-condition)::= ENDFILE I ENDPAGE I KEY I NAME I RECORD I TRANSMIT 1
UNDEFINEDFILE
CM80. {programmer-named-condition)::= CONDITION (fidentifier))
2.4.10-S The Stop Statement
CM81. @itop-statement)::= STOP;
40
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 59

2.4.11 STORAGE CONTROL STATEMENTS
cM82. {assignment-statement)::= freference-commalist+ = {expression) C,BY NAMEI;
CM83. {allocate-statement)::= ALLOCATE $allocation-commalist?;
CM84. {allocation?::= (identifier) {[{set-option)3 l [{in-option)11
(385. +&at-option)::= SET ($reference$+)
CM86. {in-option)::= IN (Creferencegl
CM8-7. {free-statement)::= FREE {freeing-commalist+;
CM88. $freeing+::= [{locator-qualifier)1 Fidentifierp [fin-option)1
2.4.12 INPUT/OUTPUT STATEMENTS
2.4.12-l The Open and Close Statements
CM89. {open-statement*::= OPEN {single-Opening-commalist);
CM90.
CM91.
CM92.
CM93.
CM94.
CM95.
CM96.
CM9-l.
{single-opening)::= Cfile-option) l Iftab-option21 l [Ctitle-option?] l
[Tlinesize-option)1 0 C$pagesize-option$l l
[STREAM] . [RECORD] . [INPUT3 l COUTPUTI l [UPDATE] .
[SEQUENTIALI l [DIRECT] l [PRINT] . [KEYED] .
[<environment)3
$file-option)::= FILE ({reference))
{tab-option)::= TAB ({expression-commalist+)
Ctitle-option)::= TITLE ($.expression))
{linesize-option)::= LINESIZE ({expressions)
{pagesize-option?::= PAGESIZE ({expression))
{close-statement)::= CLOSE {single-closing-commalist);
{single-closing)::= {file-option) l [{environment)1
2.4.12.2 Record I/O
CM98. {delete-statement)::= DELETE (ffile-option? l [fkey-option+ll;
CM99. {locate-statement)::= LOCATE Cidentifier) tffile-option) l [{pointer-set-option~l~
Cqkeyfrom-optiorQl1;
CMlOO. {pointer-set-option)::= SET (ireference))
CMlOl. $read-statement?::= READ ITfile-option) l {{into-optio* 1 {pointer-set-option) 1
fignore-option)) l
Cskey-option? 1 Tkeyto-optionfll;
CM102. sinto-option)::= INTO (Creferencep)
CM103. <ignore-option)::= IGNORE ({expression>)
CM104. {key-option)::= KEY (Cexpression))
Chapter 2: Concrete Syntax 41
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 60

CM105. $keyto-option)::= KEYTO (sreference))
CM106. *ewrite-statement)::= REWRITE {{file-option) 1 {file-option) l [{key-option)1 l
{from-option));
CM107. {write-statement)::= WRITE c$file-option) l {from-option) l
[{keyfrom-option)l>;
CM108. *from-option)::= FROM ({reference))
CM109. {keyfrom-option)::= KEYFROM ($expression+)
2.4.12.3 Stream I/O
CMllO.
CMlll.
CMl12.
09113.
CMll4.
CM115.
CM116.
CM117.
CM118.
{get-statement)::= GET {*get-file) 1 {get-string*);
$get-file)::= [*file-option)] l [{copy-option)] l
[{skip-option)3 l [{input-specification)3
$cOpy-option)::= COPY [(*reference*)1
$skip-option)::= SKIP C($expression))l
{get-string)::= STRING ($expressionp) l {input-specification) l t$copy-option)]
Cput-statement)::= PUT {{put-file* 1 Fput-string?);
(put-file)::= [{file-option)1 l [<skip-optionpl l [(line-option)] l [PAGE] .
Cfoutput-specification)1
{line-option)::= LINE ({expressionp)
$put-string)::= STRING (Creferenceg) l {output-specification)
2.4.12.3.1 Stream Input Specification
CM119. (input-specification)::= Cdata-directed-input) 1 Qist-directed-input) 1
sedit-directed-input)
CM120. {data-directed-input)::= DATA [({data-target-commalist))l
CM121. {data-target)::= Cunsubscripted-reference)
CM122. (list-directed-input)::= LIST (finput-target-commalist))
CM123. {input-target)::= *reference) 1 (sinput-target-commalist) W {do-spec))
CM124. {edit-directed-input)::= EDIT Tedit-input-pair-1isQ
CMl25. {edit-input-pair)::= ({input-target-commalist>) ((format-specification-commalist))
42
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 61

2.4.12.3.2 Stream Output Specification
CM126. Coutput-specification)::= *data-directed-output) 1 (list-directed-output) 1
*edit-directed-output?
CM127. Cdata-directed-output*::= DATA C($data-source-commalist+)
CM128. (data-source)::= <reference) 1
(+data-source-commalist) Do Cdo-spec*)
CM129. *list-directed-output)::= LIST ((output-source-commalist))
CM130. (output-source)::= Cexpression) 1 ({output-source-commalist# DO {do-spec))
CM131. {edit-directed-output)::= EDIT *edit-output-pair-list+
CM132. {edit-output-pair*::= (foutput-source-commalist))
({format-specification-commalist))
2.4.12.3.3 Format Specification Lists and the Format Statement
CM133. (format-specification)::= {format-item) 1 {format-iteration+
CM134. {format-iteration)::= $format-iteration-factor)
{{format-item+ 1 ({format-specification-commalist)))
CMl35. {format-iteration-factor)::= Cintegera I ({expression))
CM136. <format-item)::= +data-format) I (control-format) I <remote-format)
CM137. *data-format)::= *real-format) 1 *complex-format) 1 {picture-format) I
{string-format)
CM138. Creal-format)::= $fixed-point-format) I Cfloating-point-format)
CM139. Cfixed-point-format)::= P ({expression) C,$expressio* C,$expression)ll)
CM140. {floating-point-format*::= E ({expression+ C,$expression* C,$expression)ll)
CM141. {complex-format)::= C (fsreal-format) I spicture-format?)
[,$real-format) I ,$picture-formaql)
CM142. <picture-forma*::= P {picture)
CM143. {string-format)::= {character-format) I Chit-format)
CM144. {character-format)::= A [({expression))]
CMl45. Chit-format)::= *radix-factor) [({expression))1
CM146. {control-format*::= (tab-format) 1 {line-format) I {space-format) I
Cskip-format) I (column-format) I PAGE
CM147. {tab-format)::= TAB [((expression))3
CM148. {line-format)::= LINE ({expression))
CM149. {space-formay::= X ((expression))
CM150. {skip-format)::= SKIP C($expression))l
CM151. {column-format)::= COLBMN ({expression))
CM152. (remote-format)::= R ({reference))
CM153. {format-statement)::= FORMAT (<format-specification-commalist));
chapter 2: Concrete Syntax 43
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 62

2.4.13 EXPRESSIONS
CM154. *expression)::= {expression-seven) 1 $expression) $1) sexpression-seven)
CM155. {expression-seven)::= {expression-six? 1 {expression-seven) & {expression-sixI+
CM156. {expression-six)::= {expression-five) 1
{expression-six? {comparison-operator) {expression-fiv*
CM157. {comparison-operator)::- > 1 >= 1 = 1 < 1 <= I -> I Y= 1 -.<
CM158. {expression-five)::= {expression-four) I sexpression-five* $1 I) (expression-four)
CM159. {expression-four)::= *expression-three? I
{expression-four) f + 1 - 1 {expression-three)
CM160. {expression-three)::= {expression-two) 1
{expression-three) C * 1 / 3 {expression-two)
CM161. <expression-two)::= $primitive-expression) 1 {prefix-expression) I
$parenthesized-expression) 1 $expression-one)
CM162. {expression-one)::= CCprimitive-expression) 1 (parenthesized-expression))
** $expression-two*
CM163. {prefix-expression)::= {prefix-operator* {expression-two)
CM164. {prefix-operator)::= + 1 - I q
CM165. {parenthesized-expression)::= (+Iexpression))
CM166. {primitive-expression)::= freference) I $constant$ I {isub)
CM167. {reference)::= [{locator-qualifier)] {basic-reference) [{arguments-list11
CM168.. {locator-qualifier)::= *reference* ->
CM169. $arguments)::= ([+&script-commalisy3)
CM170. $basic-reference)::= t$structure-qualification?1 {identifier+
CMl71. Cstructure-qualification)::= {basic-reference) [{argumentsql.
CM172,. {subscript)::= {expression) I *
CM173. funsubscripted-reference+::= [{unsubscripted-reference) .I $identifierI+
CM174. @on&ant)::= {arithmetic-constant) I Cstring-constant2
CM175. $&ring-constant)::= Csimple-string-constant) I {replicated-string-constant?
CM176. {simple-string-constant)::= {simple-character-string-constantI# I
{simple-bit-string-constan*
CM177. eeplicated-string-constant)::= ((integer*) $simple-string-constant)
44
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 63

The following are examples of two middle-level parses. As in the previous examples, each
is accompanied by an example of a construct that matches the given syntax.
e=$expression)
e
I
e7
I
e6
I
e5
I e4
I r------------------+-------------------,
I I I e4 e3
I e3
.--------I-------,
r-------+--;
I e3 / e2
i i e3 *
I e2
{parentI!esixed-
expression)
r-~----+------, I
( e !
I e7
i e2
{primltive-
expression)
$refe!ence)
$basiL-
reference*
$iden!ifier)
I e2
I {primitive-
expression)
I {reference)
$basib-
reference)
I (identifier)
I {primitive-
expression*
I fconstantl)
I {arithmetic-
constant?
I e6
I e5
I e4
r--------;---I ---;
I e4 + e3
I I e3 e2
I e2 $prim!tive-
{primltive-
expression?
I expression* {reference?
I {reference) {has&!-
@asid-
reference*
reference) {idenkfier)
$iden!if ierp
A sample concrete representation of the above structure.
(A+B)*C-D/3
Example 2.3. An Example of the Middle-level Structure of an {expression).
Chapter 2: Concrete Syntax 45
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 64

{do-st:tement)
r-------+-------,
I
DO Cdo-Qpecl I
i *reference)
$3asic-Jference)
I <identifier)
i i = *spec-commalist)
I
C-w=?
I r--------------+--------------,
I e *while!option)
I I f---+---7 I
e7 <to-option) WHILE 1 A !
I . I . r-------1 i . I I
i TO e I
{reference)
I ! i I e7 e7
. I
i . I . fidentifier*
i I
I f.refe;ence)
I .
.
i {identifier)
i e6
I
/
r----+-- ----,
I ! I e6 e5
I I . .
. .
i i Creference) {arithmetic-
I constant)
.
.
i *identifier)
It*+*++*~++***+*++++*+++*++*r+++**********************************************~*********~*****~
Example 2.4. An Example of the Middle-level Structure of a Cdo-statement).
46
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 65

2.5 The Low-level Syntax of PL/I
2.5.1 PL/I TEXT
CLl. {pli-text)::= Ifdelimiter-list)3 [{delimiter-pair-list21
CL2. {delimiter-pair)::= {non-delimiter) {delimiter-list?
CL3. {delimiter*::= + 1 - I* I/ 1 ** I> I< I= I>= 1 <= I--> I-< 1 -a= I- 1
& I Cl? I $11) I ( I ) I - I I I ; I : I -> I 13 I Cco~=W I Qnclude)
CL4. (non-delimiter)::= {identifier) I {arithmetic-constant) I
fsimple-bit-string-constant* 1
{simple-character-string-constant+ I fisub)
2.5.2 COMMENT
CL5. {comment)::= /* C$comment-body-list)] $+-list) /
CL6. {comment-body)::= [f*-listfl {comment-character+ 1 /
CL-7. {comment-character)::= {letter) 1 Cdigit) 1 - 1 H I ' 1 = I +
: I & I $1) I je&zak,hg?al-character)
-I>l<lX
Note: Rules CL%7 effectively state that a comment beg ins with /* and ends
with */ and that any characters may appear between these except the
consecutive pair */,
2.5.3 IDENTIFIER
r) Esletterp I Fdigit) [ -1
INIOIP IQIR I
CL8. <identifier)::= fletter) 1 {identifie
CL9. {letter)::= AlBlClDlElFlGlHlIlJlRlLlM
CLlO. {digit)::= 011l2131~1516171819
2.5.4 ARITHMETIC CONSTANT
SlTlUl”lWlXlYlz
CLll.
CL12.
CL13.
CL14.
CL15.
cL16.
CL17.
cL18.
CL19.
cL20.
CL21.
{arithmetic-constant)::= *real-constant* I {imaginary-constant*
{real-constant)::= {decimal-constant) I {binary-constant)
Cdecimal-constant)::= (decimal-number) [{scale-type) FexponentTl [PI
{decimal-number)::= Cinteger) [.[$digit-list)11 I -*digit-list+
{integer)::= Cdigit-list)
(scale-type)::= E I F
{exponent)::= [ + I - 1 Qnteger)
(binary-constant)::= {binary-number? [Cscale-type) (exponent)] B [PI
{binary-number)::= {binary-digit-list? C.[$binary-digit-list)13 I
- Cbinary-digit-list)
$binary-digit)::= 0 I 1
@maginary-constant)::= Creal-constant? I
Chapter 2: Concrete syntax 47
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 66

2.5.5 STRING CONSTANTS AND PICTURES
CL22. {simple-bit-string-constant)::= 'C+string-or-picture-symbol-list*]* {radix-factor)
CL23. {radix-factor)::= BIBl(B21B31B4
CL24. {simple-character-string-constant)::= '[{string-or-picture-symbol-list)]'
CL25. {string-or-picture-symbol+::= {letter) 1 Fdigit) I - I 13 I '* 1 = I + 1 - I * I
‘I (I) I v I I$l;l:l~lCl~l~l>l
< I X 1 $extralisgual-character)
Note: A {string-or-picture-symbolp may be two consecutive characters " or any
character other than '
CL26. {extralingual-character)::=
The category $extralingual-character? in rule CL26 is implementation-defined, as
specified in Section 2.6.2.
2.5.6 ISUB
CL27. {isub*::= {integer) SUB
2.5.7 INCLUDE
CL28. @nclude)::= XINCLUDE {text-name+;
CL29. +text-name)::=
The category Ttext-name) in rule CL29 is implementation-defined and such that if
it contains C;), then that $;+ must be contained in a +aimple-character-string-
constant? which is the only immediate subnode of {text-name?.
2.6 Character Sets
The character set used in the formation of PL/I text is a finite set of symbols,
comprising 57 language characters, and zero or more {extralingual-character)s which are
distinct from these and from each other and are implementation-defined.
-l*l~l(l~l~
7 I < I > 1 % I {extralingual-character)
This document does not specify internal or external hardware representations of the
characters, nor does it define a collating sequence for them. These may, however, be the
subject of other standards.
2.6.1 LANGUAGE CHARACTER SET
The names of the symbols in the language character set, together with the graphic
representations of them to be used in this document, are given in Sections 2.6.1.1 and
2.6.1.2.
48
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 67

2.6.I.l Letters and Digits
Name
Letter A
Letter B
Letter C
Letter D
Letter E
Letter F
Letter G
Letter H
Letter I
Letter J
Letter K
Letter L
Letter M
Letter N
Letter 0
Letter P
Letter Q
Letter R
Letter S
Letter T
Letter D
Letter V
Letter W
Letter X
Letter Y
Letter Z
2.6.1.2 Special Characters
Name
Plus
Minus
Asterisk
Slash
Greater than
Less than
Equal
Not
And
or
Percent
Graphic
A
B
C
D
E
F
G
H
I
J
K
L
'M
N
0
P
Q
R
S
T
u
V
W
X
Y
2
Graphic
+
*
>
< =
&
I x
Name Graphic
Digit 0 0
Digit 1 1
Digit 2 2
Digit 3 3
Digit 4 4
Digit 5 5
Digit 6 6
Digit I 7
Digit 8 8
Digit 9 9
Name
Period
Comma
Semicolon
Colon
Blank
Single quote
Left parenthesis
Right parenthesis
Break
Dollar
Graphic
2.6.2 DATA CHARACTER SET
Data in stream-datasets or in character-string-values may
be represented by characters
from the language character set plus any other characters permitted by the particular
implementation's Cextralingual-character)s.
Chapter 2: Concrete Syntax 49
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 68

2.7 Abbreviations
Abbreviations are provided for certain keywords (see Section 4.2.2) and builtin-function-
names. The abbreviations will be recognized as synonymous in every respect with the full
denotations, except that in the case of builtin-function-names the abbreviations have
seoarate declarations (explicit or contextual) and name scopes. The abbreviations are
shown to the right of the-full denotations in the following-list.
ALLOCATE
ALLOCATION
AUTOMATIC
BINARY
CHARACTER
COLUMN
COMPLEX
CONDITION
CONTROLLED
CONVERSION
DECIMAL
DECLARE
DEFAULT
DEFINED
DIMENSION
ENVIRONMENT
EXTERNAL
FIXEDOVERFLOW
INITIAL
INTERNAL
NOCONVERSION
NOFIXEDOVERFLOW
NONVARYING
ALLOC
ALLOCN
AUTO
BIN
CHAR
COL
CPLX
COND
CTL
CONV
DEC
DCL
DFT
DEF
DIM
ENV
EXT
FOFL
INIT
INT
NOCONV
NOFOFL
NONVAR
NOOVERFLOW
NOSTRINGRANGE
NOSTRINGSIZE
NOSUBSCRIPTRANGE
NOUNDERFLOW
NOZERODIVIDE
OVERFLOW
PARAMETER
PICTURE
POINTER
POSITION
PRECISION
PROCEDURE
SEQUENTIAL
STRINGRANGE
STRINGSIZE
SUBSCRIPTRANGE
UNALIGNED
UNDERFLOW
UNDEFINEDFILE
VARYING
ZERODIVIDE
NOOFL
NOSTRG
NOSTRZ
NOSUBRG
NOUFL
NOZDIV
OFL
PARM
PIG
PTR
POS
PREC
PROC
SEQL
STRG
STRZ
SUBRG
UNAL
UFL
UNDF
VAR
ZDIV
50
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 69

Chapter 3: Abstract Syntax
3.0 Introduction
This chapter specifies the Abstract Syntax of PL/I, which is the syntax of programs in a
tree representation which is convenient for the definition of semantics. The notation for
this syntax is defined in Chapter 1, together with some examples of its use. Further
examples are provided at the end of this chapter.
Many parts of the Abstract Syntax bear a strong resemblance to the corresponding parts of
the Concrete Syntax, and the relationship between them is intended to be, or become,
intuitively obvious. In other parts, notably in the treatment of <declaration>s, the
Abstract Syntax exhibits a structuring and completeness of information which involves a
more complex transformation. The detailed description of the transformation between the
concrete and abstract forms of a program will be given in the next chapter.
There are also many instances of context-dependent constraints which have been inserted
in parentheses in the Abstract Syntax. These are attached to categories such as
<expression>, <target-reference>, <value-reference> and <declaration-designator>, where
these are required to fall within the scope of appropriate <declaration% so that they
have the properties indicated. The Translator (Chapter 4) checks that these constraints
are satisfied.
3.1 Abstract Syntax Rules
3.1.1 PROGRAM
AL <program>::= [<abstract-external-procedure-list,3
A2. <abstract-external-procedure>::= [<declaration-list>1 <procedure>
3.1.2 PROCEDURE
A3. <procedure>::= [<declaration-list>3 [<procedure-list>1 [<format-statement-list>1
t<condition-prefix-list>3 [<recursive>1
<entry-or-executable-unit-list>
3.1.3 DRCLARATION
A4. <declaration>::= <identifier> <scope> <declaration-type> Fdeclaration-designator)
AS. <scope>::= <external> } <internal>
Ab. <declaration-type>::= <variable> 1 (named-constant> 1 <builtin> 1 <condition>
Chapter 3: Abstract Syntax 51
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 70

3.1.4 VARIABLE
A7. <variable>::= <storage-type> <data-description>
A8. <storage-type>::= <storage-class> 1 <defined> 1 <parameter>
A9. <storage-class>::= <automatic> 1 <based> 1 <controlled> I <static>
Al 0,. <based>::= [<value-reference> (scalar & locator) I {reference-designator)3
All. <defined>::= <base-item> [<position>]
Al2. <base-item>::= <variable-reference> (-defined & -based) I
{reference-designator?
Al3. <position>::= <expression> (scalar & computational-type) I Texpression-designator)
3.1.5 DATA-DESCRIPTION
A14. <data-description>::= <dimensioned-data-description> I
<structure-data-description> I <item-data-description>
A15. <dimensioned-data-description>::= <element-data-description> <bound-pair-list>
A16. <element-data-description>::= <structure-data-description> I
<item-data-description>
A17. <bound-pair>::= <lower-bound> <upper-bound> I <asterisk>
A18. <lower-bound>::= <extent-expression>
Al9. <upper-bound>::= <extent-expression>
A20. <extent-expression>::= {<expression> (scalar & computational-type) I
{expression-designator+ [ <integer-value>) [<refer-option>1
A21. <refer-option>::= <identifier-list> I <integer-value>
A22. <structure-data-description>::= [<identifier-list>J <member-description-list>
A23,. <member-description>::= <data-description>
A24. <item-data-description>::= [<alignment>1 <data-type> [<initial>1
Constraint: An <item-data-description> must not have an <initial> component with
an <iteration-factor>, or an <initial-element-list> with more than one
<initial-element> immediate component, unless the <item-data-
description> is a (not necessarily immediate) component of a
<dimensioned-data-description>.
A25. <initial>::= <initial-element-list> I finitial-designator)
A26. <initial-element>::= <asterisk> I <parenthesized-expression> (scalar) I
<iteration-factor> <initial-element-list>
A27. <iteration-factor>::= <expression> (scalar & computational-type)
A28, <alignment>::= <aliqned> I <unaliqned>
52
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 71

3.1.6 DATA-TYPE
A29.
A30.
A31.
A32.
A33.
A34.
A35.
A36.
A37.
A38.
A39.
A40.
A41.
A42.
A43.
A44.
A45.
A46.
A47.
A48.
A49.
ASO..
<data-type>::= <computational-type> 1 <non-computational-type>
<computational-type>::= <arithmetic> 1 *<string> 1 <pictured>
<non-computational-type>::= <area> 1 <entry> I <file> 1
<format> [<local>1
I <label> [<local>3 I
<locator>
<arithmetic>::= <mode> <base> <scale> <precision>
<mode>::= <m> 1 <complex>
<base>::= <binary> I <decimal>
<scale>::= <fixed> 1 <float>
<precision>::= <number-of-digits> [<scale-factor>1
<number-of-digits>::= <integer>
Constraint: The <integer> must not be zero.
<scale-factor>::= <signed-integer>
<string>::= <string-type> <maximum-length> L<varvinq> 1 Ulonvarvinq>l
<string-type>::= <character> I <&>
<maximum-length>::= <extent-expression> I <asterisk>
<pictured>::= <pictured-character> I <pictured-numeric>
<locator>::= <pointer> I <offset>
<offset>::= [<variable-reference> (scalar & area) I {reference-designator)3
<entry>::= [<parameter-descriptor-list>1 [<returns-descriptor>] C<options>l
<parameter-descriptor>::= <data-description>
<returns-descriptor>::= <data-description>
<options>::=
This category is implementation-defined.
<area>::= <area-size>
<area-size>::= <extent-expression> I <asterisk>
3.1.7 NAMED-CONSTANT
A51. <named-constant>::= {<entry> I <file> <file-description> I <format> I <label>)
[<bound-pair=%&>]
A52. <file-description>::= [<stream>1 [<record>1 C<input>l [<output>1 [<update>1
C<sequential>mrect>l [<print>1 [<keyed>1 [<environment>3
A53. <environment>::=
This category is implementation-defined.
Chapter 3: Abstract Syntax 53
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 72

3.1.8 ENTRY-OR-EXECUTABLE-UNIT
A54. <entry-or-executable-unit>::= <entry-point> 1 <executable-unit>
A55. <entry-point>::= [<statement-name>1 <entry-information>
A56. <statement-name>::= <identifier> [<signed-integer-list>1
A57. <entry-information>::= [<parameter-name-list>8 [<returns-descriptor>1 [<options>1
A58. <parameter-name>::= <identifier>
A59. <executable-unit>::= [<condition-prefix-list>1 [<statement-name-list>1
{<begin-block> 1 <group>
<allocate-statement> 1 <null-statement> I <assignment-statement> 1 <on-statement>
<call-statement> 1 <open-statement>
<close-statement> 1 <put-statement> / <delete-statement> I <read-statement> I
<end-statement> i <return-statement> i
<free-statement> 1 <revert-statement> 1
<get-statement>
<goto-statement>
<if-statement>
<locate-statement>
i <rewrite-statement> I
I <signal-statement> !
I <stop-statement> I I <write-statement>.>)
3.1.9 BEGIN-BLOCK
A60. <begin-block>::= [<declaration-list>1 [<procedure-list>1 [<format-statement-list>1
[<options>1 <executable-unit-list>
3.1.10 GROUPS
A61.
A62.
A63.
A64.
A65.
A66.
A67.
A68.
A69.
A70.
A71.
A72.
<group>::= <iterative-group> I <non-iterative-group>
<iterative-group>::= <controlled-group> I <while-only-group>
<controlled-group>::= <do-spec> <executable-unit-list>
<do-spec>::= <target-reference> (scalar) <spec-list>
<spec>::= <expression> (scalar) [<to-by> I <repeat-option>] [<while-option>1
<to-by>::= <to-option> [<by-option>] I <by-option>
<repeat-option>::= <expression> (scalar)
<by-option>::= <expression> (scalar & computational-type)
<to-option>::= <expression> (scalar & computational-type)
<while-option>::= <expression> (scalar & computational-type)
<while-only-group>::= <while-option> <executable-unit-list>
<non-iterative-group>::= <entry-or-executable-unit-list>
3.1.11 ON STATEMENT
A73. <on-statement>::= <condition-name-list> [<->I {<on-unit> I <system-action>)
A74. <on-unit>::= <procedure>
54
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 73

3.1.12 IF STATEMENT
A75. <if-statement>::= <test> <then-unit> (<else-unit>1
A76. <test>::= <expression> (scalar & computational-type)
A77. <then-unit>::= <executable-unit>
A78. <else-unit>::= <executable-unit>
3.1.13 FLOW OF CONTROL STATEMENTS
A79. <call-statement>::= <subroutine-reference>
A80. <goto-statement>::= <value-reference> (scalar & label)
A81. <return-statement>::= [<expression>1
A82. <revert-statement>::= <condition-name-list>
A83. <signal-statement>::= <condition-name>
A84. <condition-name>::= <computational-condition> 1 <named-io-condition> 1
<programmer-named-condition> 1 <area-condition> 1
<error-condition> 1 <finish-condition> I (storaqe-condition>
A85. <condition-prefix>::= <computational-condition> E<enabled> I <disabled>>
A86. <computational-condition>::= <conversion-condition> I <fixedoverflow-condition> I
<overflow-condition> I <size-condition> I
<strinqranqe-condition> I <strinqsixe-condition> 1
<subscriptranqe-condition> I <underflow-condition> 1
<zerodivide-condition>
A87. <named-io-condition>::= <io-condition> <value-reference> (scalar & file)
A88. <io-condition>::= <endfile-condition> I <endpaqe-condition> 1 <key-condition> 1
<name-condition> 1 <record-condition> I <transmit-condition> I
<undefinedfile-condition>
A89. <programmer-named-condition>::= <declaration-designator> (condition)
3.1.14 STORAGE STATEMENTS
A90. <assignment-statement>::= <target-reference-list> <expression>
A91. <allocate-statement>::= <allocation-list>
A92. <allocation>::= <declaration-designator> (based I controlledi
[<set-option>] [<in-option>]
A93. <set-option>::= <variable-reference> (scalar & locator)
Constraint: The <data-description> immediate component of the <variable-reference>
must not have <offset> without a <variable-reference> subnode.
A94. <in-option>::= <variable-reference> (scalar & area)
A95. <free-statement>::= <freeing-list>
A96. <freeing>::= [<locator-qualifier>1 <declaration-designator> (based I controlled)
[<in-option>1
Chapter 3: Abstract Syntax 55
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 74

3.1.15 I/O STATEMENTS
A97. <open-statement>::= <single-opening-list>
A98. <single-opening>::= <file-option> [<tab-option>1 [<title-option>]
C<linesize-option>1 C<pagesize-option>1 Ctstream>l [<record>1
[<input>1 [<output>1 [<update>3 [<sequential>1 [<direct>1
[<print>3 [<keyed>1 [<environment>]
A99. <file-option>::= <value-reference> (scalar & file)
AlOO. <tab-option>::= <expression-list> (scalar & computational-type)
AlOl. <title-option>::= <expression> (scalar & computational-type)
A102. <linesize-option>::= <expression> (scalar & computational-type)
A103. <pagesize-option>::= <expression> (scalar & computational-type)
A104. <close-statement>::= <single-closing-list>
A105. <single-closing>::= <file-option> [<environment>1
3.1.16 RECORD I/O STATEMENTS
A106. <delete-statement>::= <file-option> [<key-option>J
A107. <locate-statement>::= <declaration-designator> (based) <file-option>
[<pointer-set-option>1 [<keyfrom-option>1
A108. <pointer-set-option>::= <variable-reference> (scalar & pointer)
Al09. <read-statement>::= <file-option>
{<into-option> 1 <pointer-set-option> 1 <ignore-option>}
[<key-option> 1 <keyto-option>]
AllO. <into-option>::= <variable-reference>
Alll. <ignore-option>::= <expression> (scalar & computational-type)
All2. <rewrite-statement>::= <file-option>
[[<key-option>1 <from-option>1
Al13. <write-statement>::= <file-option> <from-option>
C<keyfrom-option>1
Al14. <from-option>::= <variable-reference>
Al15. <key-option>::= <expression> (scalar & computational-type)
Al16. <keyfrom-option>::= <expression> (scalar & computational-type)
Al17. <keyto-option>::= <target-reference> (scalar & character)
3.1.17 STREAM I/O STATEMENTS
All8. <get-statement>::= <get-file> 1 <get-string>
Al19. <get-file>::= <file-option> [<copy-option>3 [<skip-option>1 [<input-specification>]
Constraint: At least one of the last two options must be present.
Al20. <skip-option>::= <expression> (scalar & computational-type)
Al21. <copy-option>::= <value-reference> (scalar & file)
56
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 75

A122. <get-string>::= <expression> (scalar & computational-type)
<input-specification> [<copy-option>1
A123. <put-statement>::= <put-file> 1 <put-string>
A124. <put-file>::= <file-optioti [<skip-option>1 [<line-option>1 [<->I
[<output-specification>3
Constraint: At least one of the last four options must be present and the <skip-
option> must not be used together with a <line-option> or <a>.
A125. <line-option>::= <expression> (scalar & computational-type)
A126. <put-string>::= <target-reference> (scalar & character) <output-specification>
A127. <input-specification>::= <data-directed-input> 1 <list-directed-input> 1
<edit-directed-input>
A128. <data-directed-input>::= [<data-target-list>]
A129. <data-target>::= <variable-reference> (computational-type)
Constraint: The <variable-reference> must not contain a <locator-qualifier> or a
<subscript-list>, and must not contain a <by-name-parts-list>.
A130. <list-directed-input>::= <input-target-list>
A131. <input-target>::= <target-reference> (computational-type) 1
<input-target-list> <do-spec>
Al.32. <edit-directed-input>::= <edit-input-pair-list>
A133. <edit-input-pair>::= <input-target-list> <format-specification-list>
A134. <output-specification>::= <data-directed-output> 1 <list-directed-output> 1
<edit-directed-output>
A135. <data-directed-output>::= [<data-source-list>1
A136. <data-source>::= <variable-reference> (computational-type) 1
<data-source-list> <do-spec>
Constraint: The <variable-reference> must not have a <locator-qualifier>.
A137. <list-directed-output>::= <output-source-list7
A138. <output-source>::= <expression> (computational-type) 1
<output-source-list> <do-spec>
A139. <edit-directed-output>::= <edit-output-pair-list>
A140. <edit-output-pair>::= <output-source-list> <format-specification-list>
A141. <format-specification>::= <format-item> I <format-iteration>
A142. <format-iteration>::= <format-iteration-factor> <format-specification-list>
A143. <format-iteration-factor>::= <expression> (scalar)
A144. <format-item>::= <data-format> I <control-format> I <remote-format>
A145. <data-format>::= <real-format> [ <complex-format> I <picture-format> 1
<string-format>
A146. <real-format>::= <fixed-point-format> I <floating-point-format>
Al47. <fixed-point-format>::= (<expression> (scalar & computational-type) I
<integer-value*}
[(<expression> (scalar & computational-type) 1
finteger-value>)
[I<expression> (scalar & computational-type) I
<integer-value+)11
Chapter 3: Abstract Syntax 57
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 76

A148. <floating-point-format>::= {<expression> (scalar & computational-type) 1
<integer-value,)
CI<expression> (scalar & computational-type) 1
<integer-value,)
tf<expression> (scalar & computational-type) 1
*integer-value>)11
A149. <complex-format>::= (<real-format> 1 <picture-format>}
[<real-format> I <picture-format>1
Constraint: A <complex-format> must not contain <pictured-character>.
A150. <picture-format>::= <pictured>
A151. <string-format>::= <character-format> I <bit-format>
A152. <character-format>::= [(<expression> (scalar & computational-type) I
<integer-value+)1
A153. <bit-format>::= <radix-factor> [(<expression> (scalar & computational-type) I
*integer-values11
A154. <radix-factor>::= 1 I 2 I 3 I 4
A155. <control-format>::= <tab-format> I <line-format> I <space-format> I <skip-format> I
<column-format> I <e>
A156. <tab-format>::= E<expression> (scalar & computational-type) 1 *integer-value*)
A157. <line-format>::= (<expression> (scalar & computational-type) 1 *integer-value+)
Al58. <space-format>::= {<expression> (scalar & computational-type) I *integer-value*1
A159. <skip-format>::= f<expression> (scalar & computational-type) 1 *integer-value+)
A160. <column-format>::= {<expression> (scalar & computational-type) 1 *integer-values}
A161. <remote-format>::= <variable-reference> (scalar 6~ format) I
<named-constant-reference> (scalar & format)
A162. <format-statement>::= [<condition-prefix-list>1
<statement-name-list> <format-specification-list>
3.1.18 EXPRESSION
A163. <expression>::= f<value-reference> 1 <constant> I <isub> I
<infix-expression> 1 <prefix-expression> I
<parenthesized-expression>) <data-description>
A164. <infix-expression>::= <expression> <infix-operator> <expression> <data-description>
A165. <infix-operator>::= <or> 1 <a> 1 <g&> 1 <E> 1 <q> 1 <&> 1 <s> 1 <E> 1
<cat> I <add> I <subtract> 1 <multiply> I <divide> I <power>
A166. <prefix-expression>::= <prefix-operator> <expression> <data-description>
A167. <prefix-operator>::= <u> 1 <minus> 1 <not>
A168. <parenthesized-expression>::= <expression> <data-description>
58
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 77

3.1.19 TYPES OF REFERENCE
A169. <value-reference>::= (<variable-reference> 1 <procedure-function-reference> 1
<builtin-function-reference> 1 <named-constant-reference>)
<data-description>
Al-IO. <variable-reference>::= [<locator-qualifier>3 <declaration-designator> (variable)
[<identifier-list>] [<subscript-list>1
[<by-name-parts-list>1 <data-description>
Ali'l. <by-name-parts>::= <identifier-list>
A172. <locator-qualifier>::= <value-reference> (scalar & locator)
A173. <subscript>::= <expression> (scalar & computational-type) 1 <asterisk> 1
<integer-value,
A174. <procedure-function-reference>::= <value-reference> [<argument-list>3
<data-description>
Al75. <argument>::= <expression> [<dummy>1 <data-description>
Al76. <builtin-function-reference>::= <builtin-function> [<arqument-list>3
<data-description> -
Al77. <builtin-function>::= <abs-bif> I <empty-bif>
<aces-bif> I <erf-bif>
<add-bif? I <erfc-bif>
<addr-bif> I <every-bif>
<after-bif> I <exp-bif>
<allocation-bif> I <fixed-bif>
<asin-bif> I <float-bif>
<atan-bif> I <floor-bif>
<atand-bif> I <hbound-bif>
<atanh-bif> 1 <hiqh-bif>
<before-bif> I <imaq-bif>
<binary-bif> I <index-bif>
<bit-bif> I <lbound-bif>
<bool-bif> I <lenqth-bif>
<ceil-bif> I <lineno-bif>
<character-bif> I <loq-bif>
<collate-bif> I <loqlO-bif>
<complex-bif> I <loq2-bif>
<conjq-bif> I <low-bif>
<copy-bif> I Unax-bif>
<cos-bif> I <min-bif>
<cosd-bif> I Unod-bif>
<cash-bif> I Unultiplv-bif>
<date-bif> I <null-bif>
<decat-bif> I <offset-bif>
<decimal-bif> I <onchar-bif>
<dimension-bif> I <oncode-bif>
<divide-bif> I <onfield-bif>
<dot-bif> I <onfile-bif>
I <onkey-bif>
<onloc-bif> I <onsource-bif> I
Cpaqeno-bif> I
<pointer-bif>
<precision-bif
<prod-bif>
<real-bif>
<reverse-bif>
<round-bif>
<sign-bif>
<sin-bif>
<sind-bif>
<sinh-bif>
<some-bif>
I <sqrt-bif>
<strinq-bif> I <substr-bif> I
<subtract-bif> I
<sum-bif> I <tan-bif>
<tand-bif> f <tanh-bif>
<time-bif> f <translate-bif>l
<trunc-bif>
tunspec-bif> I <valid-bif> I <verify-bif>
A178. <named-constant-reference>::= <declaration-designator> (named-constant)
[<subscript-list>1 <data-description>
A179. <target-reference>::= {<variable-reference> I <pseudo-variable-reference>)
<data-description>
Al80. <pseudo-variable-reference>::= <pseudo-variable> [<argument-list>]
<data-description>
A181. <pseudo-variable>::= <imaq-pv> I <onchar-pv> I <onsource-pv> I <paqeno-pv> I
<real-pv> I <strinq-pv> I <substr-pv> I <unspec-pv>
A182. <subroutine-reference>::= <value-reference> [<argument-list>1
Chapter 3: Abstract Syntax 59
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 78

3.1.20 CONSTANT AND ISUB
A103. <constant>::= *basic-value+ <data-type>
A184. <isub>::= <integer>
3.1.21 TYPES OF VALUE
A185. <identifier>::=
A186. <signed-integer>::= C + 1 - ) <integer>
Al87. <integer>::=
The two categories, Al85 and A187, are defined as #symbol-list+ corresponding to
the sequences of characters in an {identifier? or sinteger) respectively. see rules
CL8 and CL15 in Chapter 2.
3.1.22 TYPES OF PICTURE
A188. <pictured-character>::= <character-picture-element-list>
A189. <character-picture-element>::= A 1 X 1 9
A190. <pictured-numeric>::= <numeric-picture-specification> <arithmetic>
Al91. <numeric-picture-specification>::= <fixed-point-picture> [<picture-scale-factor>1 1
<floating-point-picture>
Al92. <fixed-point-picture>::= <numeric-picture-element-list>
A193. <floating-point-picture>::= <picture-mantissa> <picture-exponent>
A194. <picture-mantissa>::= <numeric-picture-element-list>
A195. <picture-exponent>::= <numeric-picture-element-list>
Al96. <picture-scale-factor>::= <signed-integer>
A197. <numeric-picture-element>::= EIII~IRlSlTl~lYl~l$I91+1-1 * 1 <insertion-character> 1 <credit> 1 <debit>
Al98. <insertion-character>::= B I / I . I ,
A199. <credit>::= CR
A200. <debit>::= DB
60
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 79

The abstract text corresponding to the concrete representation
GO TO P -> S.LV;
is as follows:
<goto-statement>
I
<value-reference>
I <variable-reference>
r-------------------------i-------------------------~
I <locator-qualifier> <declaration-designator> <identifier-list>
I (for S)
<value-reference>
I I <variable-reference> <identifier>
I <declaration-designator>
(for P)
I LV
Note: <value-reference> and <variable-reference> nodes immediately contain a
<data-description> (not shown).
The abstract text corresponding to the concrete-representation
DECLARE 2 EXTERNAL STATIC ALIGNED COMPLEX DECIMAL FLOAT(15);
is as follows:
<declaration>
~------------i-------------,
I I <identifier> <scope> <declaration-type>
I I <external> <variable>
I r------------------“----,
I I <storage-type> <data-description>
I I <storage-class> <item-data-description>
I <static> I r----"--------------
l I <alignment> <data-type>
I I <aliqned> <computational-type>
I <arithmetic>
I r--------------------------------,
I I I I <mode> <base> <scale> <precision>
I I I I <complex> <decimal> <float> <number-of-digits>
I <integer>
1
I i5
******+**+*****t************************************************************************~
Example 3.1. Examples of Abstract Text.
Chapter 3: Abstract Syntax 61
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 80

Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 81

Chapter 4: The Translator
4.0 Introduction
This Chapter defines an operation which translates a fsymbol-list+ into an <abstract-
external-procedure>. All the <abstract-external-procedure% that are part of a PL/I
<program> are then combined. The <program> is used by the PL/I machine to determine the
course of execution.
4.1 Translate
A concrete-block is a f.begin-block), {procedure?, or {concrete-external-procedure+.
X is a concrete-block-component of y if y is a concrete-block, and y contains x but does
not contain any other concrete-block which also contains x. In this case y
concrete-block-contains x.
An abstract-block is a <begin-block>, <procedure>. or <abstract-external-procedure>.
X is an abstract-block-component of y if y is an abstract-block, and y contains x but
does not contain any other abstract-block which also contains x. In this case y
abstract-block-contains x.
The informal term block-component is used for either concrete-block-component or
abstract-block-component, and block-contains is used for either concrete-block-contains
or abstract-block-contains where the context makes it obvious which formal term is
required.
A {declarationg, {description>, {default-attributes) or fgeneric-descriptioe, d
declaration-contains a node n, if d contains n and d does not contain a {description+ or
Cgeneric-description) that also contains n.
A node, nr is a declaration-component of a node, d, if d declaration-contains n.
Operation: translate(t)
where t is a <symbol-list*.
result: an <abstract-external-procedure>.
Step 1. Perform parse(t,$procedure+) to obtain a $procedure*,cep. Append a Cconcrete-
external-procedure): cep; to the <translation-state+.
Step 2. Perform complete-concrete-procedure.
step 3. Let aep be an <abstract-external-procedure>.
step 4. For each $declarationq,d which is a block-component of the {concrete-external-
procedure) perform create-declaration(d) to obtain a <declaration>,ad, and
append ad to the <declaration-list> in aep.
step 5. For each <declaration>,d which is a block-component of aep and which contains at
least one Cexpression-designator) or {reference-designator+, perform replace-
concrete-designators(d).
step 6. Let p be the $procedure$ immediate component of the $concrete-external-
procedure?. Perform create-procedure(p) to obtain a <procedure>,ap, and attach
ap to aep.
step 7. Delete the <concrete-external-procedure).
step 8. Perform.validate-procedure(aep).
step 9. Return aep.
Chapiter 4: The Translator 63
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 82

4.2 Forming the Concrete Procedure
The parse operation is applied to a +aymbol-list? to construct a complete tree with
respect to the Concrete Syntax for a specified category-name. If this category-name is
defined in the high-level syntax or the middle-level syntax then some additional mapping
at the interfaces between these syntaxes is required.
If parse is called for a {procedure), it calls itself recursively to build trees
consistent with the low-level, middle-level , and high-level syntaxes, in that order.
Operation: parse(sl,n)
Case 1.
Case 2.
Case 3.
where sl is a +qmbol-list?,
n is a tree with a single node, whose type is a non-terminal category
in the Concrete Syntax.
result: a complete tree with respect to the Concrete Syntax for n.
The type of n is a non-terminal of the high-level syntax.
Perform parse(sl,$sentence-list)) to obtain a {sentence-list?,snl. Perform
high-level-parse(snl,n) to obtain nt.
Return nt.
The type of n is a non-terminal of the middle-level syntax.
Perform parse(sl,$pli-text)) to obtain a $pli-text),pt. Perform middle-level-
parse(pt,nJ to obtain nt.
Return nt.
The type of n is a non-terminal of the low-level syntax.
Perform low-level-parse(sl,nl to obtain nt.
Return nt.
4.2.1 LOW-LEVEL-PARSE
Operation: low-level-parse(sl,n)
Step 1.
Step 2.
where sl is a $symbol-list),
n is a tree with a single node, whose type is a non-terminal category-
name at the low-level syntax.
result: a complete tree with respect to the low-level syntax for n.
There must exist one and only one tree, nt, which is a complete tree with
respect to the low-level syntax for nr such that the following conditions are
true:
(1) the concrete-representation of nt is exactly the same as the concrete-
representation of sl, and
(2) every occurrence of $0) or $*/) in the concrete-representation of nt must
be such that the $/) and $*) are nodes of a {comment) category or are
contained in a <non-delimiter), and
(3) of all possible trees satisfying conditions (1) and (2), nt is that one
containing the least number of Cdelimiter-pair)s and $delimiter)s.
Return nt.
64
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 83

4.2.2 MIDDLE-LEVEL-PARSE
A keyword is a category-name specified in the middle-level syntax as a sequence of
uppercase letters.
{delimiter-or-non-delimiter?::= fdelimiter) 1 {non-delimiter)
Operation: middle-level-parse(pt,cn)
where pt is a $pli-text),
cn is a tree with a single node, whose type is a non-terminal of the
middle-level syntax.
result: a complete tree with respect to the syntax composed of all the
production-rules occurring in the middle-level syntax and in the
low-level syntax with the root-node cn.
Step 1. Let t be a Fdelimiter-or-non-delimiter-list) which contains a copy of the
{delimiter) and {non-delimiter+ components of pt in the same order.
Step 2. Repeat steps 2.1 through 2.5 as long as there is a Cdelimiter-or-non-
delimiter),d: {delimiter): {include);; in t.
Step 2.1. Let s be Tsymbol-list): Csymbol): 16.
Step 2.2. Append to s any $symbol~s obtained in an implementation-defined way from the
{text-name) in d. Append {symbol): b; to s.
Step 2.3. Perform low-level-parse(s,$pli-text)) to obtain a {pli-text),tx.
Step 2.4. Let tl be a {delimiter-or-non-delimiter-list> which contains a copy of the
{delimiter) and {non-delimiter? components of tx in the same order.
Step 2.5. Replace d by the immediate components of tl in the same order.
Step 3. Delete from t any {delimiter+ containing a b or a <comment). This must not
cause t to be deleted.
Step 4. Let nt[il. i=l,...,n, be the ordered list of nodes which are the immediate
components of the $delimiter@ and fnon-delimiter)s in t.
Step 5. There must exist one and only one tree, mt, which is a complete tree with
respect to the middle-level syntax for the root-node cn and satisfies any
additional constraints specified with that syntax, and which is such that mt
contains m terminal nodes nmt[jl, j=l,...,m and there is a one-to-one
correspondence between the ntCi1, i=l,...,n and nmtCj1, j=l,...,m taken in left-
to-right order as specified by Case 5.1 through Case 5.5 except for the
following instance:
If nmt[jl is an $environment-specification) or an {options-specification) then
it corresponds to k nodes nt[ll, l=i,...,i+k-1 such that
(1) no ntC11, l=i, . . ..i+k-1 is a C;);
(2) all nodes nt113, l=i ,---, i+k-1 which are either a $0 or a $1) must b’e
matched in the normal way for balancing parentheses.
Case 5.1. nmtCj3 is a keyword.
nt[il must be an Qdentifier) containing the same terminals as the
characters appearing either in the denotation of nmt[jl or in the
abbreviation for nmtCj1. (See Section 2.7.)
Case 5.2. nmt[jl is a non-bracketed category-name other than a keyword.
nmt[jl and ntCi1 must be equal.
Chapter 4: The Translator 65
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 84

Case 5.3. nmt[jl is a {radix-factor+.
ntCi1 must be an <identifier* such that the ordered sequence, seq, of its
terminals can be a denotation of an immediate component of a {radix-factor).
Replace nmtCj1 by a <radix-factor*: seq.
Case 5.4. nmtCj1 is an {imaginary-constant), a {real-constant*, or an {integerg-
ntCi1 must be an {arithmetic-constant) containing just a node, lc, with the
same type as nmtljl. (There may be intermediate nodes between nt[il and lc,
but no side branches.)
Replace nmtfjl by the tree with root-node lc.
Case 5.5. nmtljl is one of the following:
{identifier*
{arithmetic-constant>
fsimple-bit-string-constant+
{simple-character-string-constant?
$isub*
<letter)
nttil and nmtljl must be of the same type.
Replace nmtCj1 by nt[il.
Step 6. Each {description) and qgeneric-description? must contain a subtree.
Step 7. Return mt.
4.2.3 HIGH-LEVEL-PARSE
Operation: hiqh-level-parse(sl,cn)
where sl is a Csentence-list),
cn is a tree with a single node, whose type is a non-terminal of the
high-level syntax.
result: a complete tree with respect to the Concrete Syntax for cn.
Step 1. For each {single-statement),s, component of sl, in left-to-right order perform
Steps 1.1 through 1.3.
Step 1.1. If s contains a {begin-statement), $do-statement?, or {procedure-statement+,
then attach {unmatched) to s.
Step 1.2. If s contains an {end-statement+ not containing an Qdentifier) then remove
the rightmost preceding {unmatched).
Step 1.3. If s contains an {end-statement) containing an $identifier),id then perform
Steps 1.3.1 and 1.3-2.
Step 1.3.1. Let rw be the rightmost preceding {single-statement) containing
$unmatched) such that rpu contains also a tree of the form Cstatement-
name),sn: $identifierg,idsn; and idsn is equal to id. sn must not
contain a {signed-integer-commalist). rpu must exist.
Step 1.3.2. Let k be the number of $unmatched~ components of sl following rpu and
preceding s. Let es be a
*sentence):
(single-statement):
send-statement?:
END{;+
Attach k copies of es to sl immediately preceding s-. Delete the k+l
instances of {unmatched) which immediately precede s-
66
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 85

Step 2. Let ntCi1, i=l,...,n be the ordered sequence of components of sl which are such
that for each component, c, the following conditions are satisfied:
(1) the category-name of c is terminal with respect to the middle-level syntax,
and
(2) c is not contained in any component of sl whose category-name is terminal
with respect to the middle-level syntax.
Step 3. There must exist one and only one tree, ht, with root-node of the same type as
cn such that the following conditions are true:
(1) ht is a complete tree with respect to the syntax composed of all the
production-rules occurring in the high-level syntax, and in the middle-level
syntax, and
(2) ht contains n terminal nodes htn[jl, j=l,...,n, such that for every i,
i=l,...,n, the type of the node ntCi1 is the same as the type of htn[il.
Step 4. For each i, i=l,-,..,n, replace htn[il by nt[il. Return ht.
Chapter 4: The Translator 67
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 86

4.3 Completion of the Concrete Procedure
The {concrete-external-procedure+ is "completed" in the sense that all declarations are
constructed or completed.
Operation: complete-concrete-procedure
Step 1. Perform reorganize.
Step 2. Perform construct-explicit-declarations.
Step 3. Perform complete-structure-declarations.
Step 4. Perform construct-contextual-declarations.
Step 5. Perform construct-implicit-declarations.
Step 6. Perform complete-declarations.
Step 7. Perform validate-concrete-declarations.
4.3.1 REORGANIZE
The {concrete-external-procedure) is reorganized in various ways to simplify and complete
it.
Operation: reorganize
Step 1. Perform complete-options.
Step 2. Perform modify-statement-names.
Step 3. Perform complete-attribute-implications.
Step 4. For each {declaration-commalist)rds immediate component of a Cdeclare-statement?
component of the {concrete-external-procedure+ perform defactor-declarations(ds)
to obtain a {declaration-commalist)rdds and replace ds by dds.
4.3.1.1 Complete-options
Various modifications are made to {put-statement+, {get-statemenys, and {format-
statement&a to complete their options. These are performed before the application of any
(default-statement)s.
Operation: complete-options
Step 1. For each Cget-statement),gs component of the {concrete-external-procedure?
perform Steps 1.1 and 1.2.
Step 1.1. If gs does not contain a {file-option) or a {get-string*, then perform
parse("FILE(SYSIN)",Cfile-option)) to obtain a {file-option),fo, and attach
fo to gs.
Step 1.2. If gs contains a {copy-option>,co which does not contain a $reference$, then
perform parse("COPY(SYSPRINT)" ,$copy-option)) to obtain a Tcopy-option),nco,
and replace co by nco.
Step 2. For each /put-statement),ps component of the Cconcrete-external-procedure*, if
does not contain {file-option* or a
~~rse("FILE(SYSPRINT)",&le-option+)
Cput-string) then perform
to obtain a (file-option),fo, and attach
fo to ps.
Step 3. For each $tab-format*,tf component of the sconcrete-external-procedure) where tf
does not contain an $expression) perform parse("TAB(l)",$tab-format)) to obtain
a {tab-formatf,ntf, and replace tf by ntf.
68
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 87

step 4. For each *skip-option),sk component of the Cconcrete-external-procedure?, if sk
does not contain an $expression+ then perform parse('SKIP(l)*,$.skip-option)) to
obtain risk, and replace sk by nsk.
Step 5. For each {skip-format),sf component of the fconcrete-external-procedure?, if sf
does not contain an $expression) then perform parse("SKIP(l)*,$skip-format)) to
obtain nsf, and replace sf by nsf.
Step 6. For each *radix-factorq,rf component of the fconcrete-external-procedure), if rf
contains only B then replace rf by a (radix-factor*: Bl.
4.3.1.2 Modify-statement-names
The {statement-name) components of a Cdeclare-statement+ or *default-statement) are
removed and attached to fnull-statementps. (It is not possible for control to branch to a
declaration or a default during execution.) Multiple occurrences of Tstatement-name)s in
the {prefix-list) component of a {procedure), or in $unit*s which contain Tentry-
statementIs, are also simplified.
Operation: modify-statement-names
Step 1. For each +XnitP,u immediate component of a #nit-list),ul component of the
{concrete-external-procedure*, where u immediately contains a *statement-name-
list),snl and a {declare-statement) or {default-statement?, perform Steps 1.1
and 1.2.
Step 1.1. Let pl be a Cprefix-list). For each $statement-namep,sn of snl perform Step
1.1.1.
Step 1.1.1. Append (prefix?: snc; to pl, where snc is a copy of sn.
Step 1.2. Let un be a
<unit):
{executable-unit+:
Pl <executable-single-statement*:
{null-statement*:
S;?*
Attach un to ul immediately preceding u. Delete snl.
Step 2. For each $procedureZ+,pc, where pc is a component of the Cconcrete-external-
procedure? and pc immediately contains a Cprefix-list?,pf, perform Steps 2.1 and
2.2.
Step 2.1. pf must contain at least one {statement-name?. For each {statement-name),sn
component of pf, after the leftmost one, perform Steps 2.1.1 through 2.1.3.
step 2.1.1. Let snc be a copy of sn. Let unl be a
*unit?:
{statement-name-list>:
snc;
(entry-statementP,es:
ENTRY
c;*-
Step 2.1.2. If the Fprocedure-statement) immediate component of pc contains an
sentry-informationq,ei then let eic be a copy of ei, delete any
RECURSIVE subnode of eic, and attach eic to es.
Step 2.1.3. Attach unl to pc as the first component of the {unit-list) imnediate
component of pc.
Step 2.2. Delete every {statement-name) except the first from pf.
step 3. For each $unit),u component of the Cconcrete-external-procedure), where u
immediately contains a sstatement-name-1ist)rsnl and an {entry-statement),es and
snl contains more than one {statement-name?, perform Steps 3.1 and 3.2.
Chapter 4: The Translator 69
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 88

Step 3.1. For each Cstatement-name),sn component of snl after the first, perform steps
3.1.1 and 3.1.2.
Step 3.1.1. Let snc be a copy of sn. Let esc be a copy of es. Let unl be a
@nit*:
*statement-name-list+:
snc;
est.
Step 3.1.2. Attach unl to the Qunit-list* which immediately contains u so that unl
immediately precedes u.
Step 3.2. Delete every {statement-name+ from snl except the first.
4.3.1.3 Complete-attribute-implications
The {dimension-attribute*, +.precision), and the Cdata-attribute?: FIXED; can be implied
without the use of the keywords DIMENSION, PRECISION, and FIXED. These implications are
replaced by explicit declarations of these attributes.
Operation: complete-attribute-implications
Step 1.
Step 2.
Step 3.
Step 4.
For each $dimension-suffix$,ds component of the {concrete-external-procedure?
such that ds is not a component of a *dimension-attribute) append a
sdata-attribute?:
fdimension-attribute?:
DIMENSION
dsc;:
where dsc is a copy of ds, to the node immediately containing ds, and delete ds.
For each <generic-descriptionargda component of the Cconcrete-external-
procedure*, if gda immediately contains an {asterisk-bounds),ab then append a
*generic-data-attribute?: DIMENSION ab: to the <generic-data-attribute-list+
component of gda and delete ab.
For each {attribute?: {data-attributep,atr; or Fdata-attribute).atr which is an
immediate component of a list, 1, component of the {concrete-external-
procedure), if atr simply contains a (precision+,p but not PRECISION then append
to 1 a
Cdata-attribute?:
PRECISION
pl;
where pl is a copy of p, and delete p.
For each {data-attribute-1ist)ral or Fattribute-list),al of the +concrete-
external-procedure), if al simply contains a fdata-attribute) with $precision),p
and p contains a {scale-factor) then append to al a
{data-attribute+:
FIXED.
70
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 89

4.3.1.4 Defactor-declarations
The syntax of sdeclare-statement? allows Cidentifierls to be factored together to give
them the same structuring or attributes. This factoring is unravelled to provide a
single $declaration) for each {identifier?.
Operation: defactor-declarations0
where dc is a {declaration-commalist).
result: a {declaration-commalist?.
step 1,. Let ds be a copy of dc.
step 2. For each $declarationZ+,d immediate component of ds, if d immediately contains a
{declaration-commalistq,dcl then perform defactor-declarations(dcl1 to obtain a
sdeclaration-commalistq,dcll and replace dcl by dell.
step 3. For each $declaration%,d immediate component of dsr if d immediately contains a
$declaration-commalista,dl then perform Steps 3.1 through 3.3.
Step 3.1. If d immediately contains a $level),lv then there must not be a Clevelp
declaration-contained in any of the Cdeclarationqs immediately contained in
dl, and attach a copy of Iv to each $.declarationq immediate component of dl.
Step 3.2. If d immediately contains an (attribute-list),al then append a copy of each
<attribute) of al to each qdeclaration? immediate component of dl.
Step 3.3. Replace d by the immediate components of dl in the same order.
step 4. Return ds.
4.3.2 CONSTRUCT-EXPLICIT-DECLARATIONS
The occurrence of an $identifier) in the {concrete-external-procedure* as an immediate
component of a {declaration) explicitly specifies that qidentifier? as the name of some
data item. Other contexts may also constitute explicit declarations of an qidentifier?,
in which case a {declaration> is created for it.
{declared-statement-names*::= [<name-list)1 I$procedureT 1 *begin-block*3
*name>::= (statement-namef C+entry-information)3
Operation: construct-explicit-declarations
Step 1. For each $procedurep,p contained in the {concrete-external-procedure? perform
declare-parameters(p).
step 2. Let p be the *procedure* immediate component of the Cconcrete-external-
procedure*. Perform declare-statement-names(p) to obtain a Cdeclared-statement-
names+,dsn. Replace p by the (procedure? of dsn. The Qame-listq,nl of dsn
must not contain a Csigned-integer-commalistp.
Step 3. Perform construct-statement-name-declarations(nl,eattrJ where eattr is an
{attribute+:
{data-attribute?:
ENTRYO;;
to obtain a *unit* containing a {declaration-commalist)rdc. Replace all
occurrences of <attribute?: INTERNAL; in dc by Cattribute+: EXTERNAL.
Step 4. Attach dc to the *concrete-external-procedure).
Chapter 4: The Translator 71
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 90

4.3,.2-l Declare-parameters,
Each Cidentifier) occurring in a (parameter-name-commalist) represents an explicit
declaration of a parameter. Unless it is already declared, a declaration is introduced
for it. If it has been declared erroneously then a conflict will occur when this
declaration is being transformed to its abstract equivalent.
Operation: declare-parameters(p)
where p is a $procedure).
Step 1. For each <parameter-name-commalist+,pl block-component of p perform Step 1.1.
Step 1.1. pl must be such that no two {identifier) components of pl are equal. For
each {identifier),id component of pl perform Steps 1.1.1 through 1.1.3.
Step 1.1.1. Perform find-applicable-declaration(id) to obtain d.
Step 1.1.2,. If d is <absent+, or d is not a concrete-block-component of p, or d is a
concrete-block-component of p but declaration-contains a Clevel) whose
value is not 1, then let d be a
{unit):
{declare-statement?:
DECLARE
{declaration-commalist+:
$declarationp,d:
id;;
C;?;;
and append d to the #unit-list* of p.
Step 1.1.3. If d does not contain qattributet: PARAMETER; then attach Cattribute):
PARAMETER; to d.
4.3.2.2 Declare-statement-names
A $statement-name) may occur as a component of a Cunit.) which contains an $entry-
statement*, {format-statement*, or some other {executable-unit), or as a component of a
Cprocedure). These contexts are used to determine the $attribute)s to be attached to the
explicit declarations for the Cidentifier) of the {statement-name).
Operation: declare-statement-names(p)
where p is a {procedure+ or {begin-block+.
result: a {declared-statement-names?.
Step 1. Let pc be a copy of p and let lnl, pnl, fnl, and en1 each be a {name-list) with
no components.
Step 2. For each {statement-namef,sn concrete-block-component of pc perform Step 2.1.
Step 2.1. One of the following Cases must apply:
Case 2.1.1. sn is simply contained in an {executable-unit),eu.
Append sn to lnl.
Case 2.1.2. sn is simply contained in a $uniY that has a {format-statement?
immediate component.
Append sn to fnl.
Case 2.1.3. sn is contained either in a {prefix-list? immediate component of pc
where pc has qprocedure-statement+,ep, or in a {unit? with an Gentry-
statement),ep as an immediate component.
72
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 91

Case 2.1.3.1. ep has an Gentry-information),ei.
If ep is an {entry-statement), then ei must not contain
RECURSIVE. Append {name*: sn ei; to enl.
Case 2.1.3.2. ep has no {entry-information+.
Append {name): sn: to enl.
Step 3. For each {procedure+ or wegin-block), pn concrete-block-component of pc perform
Step 3.1.
Step 3.1. Perform declare-statement-names(pn) to obtain a {declared-statement-
names*,dsn. Append the elements of the {name-list? of dsn to pnl. Replace
pn by the $procedureq or qbegin-block? of dsn. If pn is a {begin-block),
then the {name-list+ component of dsn must be empty.
Step 4. If In1 - not
declarations(lZ,lattr) empty then perform construct-statement-name-
, where lattr is an
{attribute?:
fdata-attribute?:
LABEL;;
to obtain a +nit),d, and append d to the $unit-list) of pc.
Step 5. If fnl is not empty then perform construct-statement-name-
declarations(fnl,fattrl, where fattr is an
{attribute?:
{data-attribute):
FORMAT:;
to obtain a $unit),d and append d to the {unit-list* of pc.
Step 6. If pnl - not
declarations(pis,pattr), empty then perform construct-statement-name-
where pattr is an
qattribute):
{data-attribute*:
ENTRYO;;
to obtain a {unitP,d and append d to the funit-list? of pc.
Step 7. Return the cdeclared-statement-names? consisting of en1 and pc.
4.3.2.3 Construct-statement-name-declarations
This operation takes a Cname-list) containing fstatement-nameps and possiblly {entry-
informationfs and constructs a *declare-statement) for them. The type of the fstatement-
names is given by a supplied *attribute). Any type of Cstatement-name) may contain one
or more $integer*s, signifying that it is one element of an array of Cstatement-name)s.
The explicit declaration for this array is constructed with a Cdimension-attribute)
component.
Operation: construct-statement-name-declarations(nl,attl
where nl is a {name-list),
att is an {attribute).
result: a {unit+.
Step 1. Let nlc be a copy of the @ame-list),nl. Let un be a
{unit):
Tdeclare-statement*:
DECLARE
{declaration-commalistt,dcl
ii**
Chapter 4: The Translator 73
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 92

step 2. While nlc contains any element, perform Steps 2.1 through 2.7.
Step 2.1. Let id be the *identifier) component of the first element of nlc.
Step 2.2. Let tnl be a Qname-list* containing a copy of all elements of nlc whose
$statement-name* immediately contains an fidentifier) equal to id.
step 2.3. Let a be a
#declaration*:
$ttribute-list),al:
Cattribute?:
INTERNAL;
{attribute?:
CONSTANT;
att.
Step 2.4. If tnl contains more than one element or if tnl contains only one element
and this element has a Csigned-integer-commalist), then perform Steps 2.4.1
through 2.4.6.
Step 2.4.1. Let m be the number of elements of tnl. Each element of tnl must
contain a {signed-integer-commalist? with the same number of elements,
n. tnl must be such that no two Isigned-integer-commalist+ components of
tnl represent the same ordered sequence of numerical values.
Step 2.4.2. Let siCi,jl be the j'th Tsigned-integer* of the Csigned-integer-
commalist) of the i'th element of tnl.
Step 2.4.3. Let ub[kl and lbikl, k=l,...,n be, respectively, the largest and least
value of si[l,kl,..., si[m,kl.
Step 2.4.4. Let bpl be the qbound-pair-list+ containing n Cbound-pair) elements such
that the j*th element is a
<bound-pair):
{lower-bound):
el;
C:?
{upper-bound,):
e2; :
where el and e2 are {extent-expressionqs representing lbl jl and ubC jl
respectively.
Step 2.4.5. Let bpcl be the fbound-pair-conunalistP produced by inserting commas as
appropriate in bpl.
step 2.4.6. Append an
{attribute?:
{data-attribute?:
{dimension-attribute):
DIMENSION
Cdimension-suffix?:
(
bpcl
);;;;
to al, the (attribute-list) of d.
Step 2.5. If al contains ENTRY then perform Steps 2.5.1 through 2.5.2.
Step 2.5.1. If any Gentry-information* of tnl contains a (returns-descriptor),rd
then perform Steps 2.5.1.1 and 2.5.1.2.
Step 2.5.1.1. Each element of tnl must contain a {returns-descriptor).
Note: a check for consistency is made in copy-descriptors.
74
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 93

Step 2.5.1.2. Append to al an
<attribute*:
{data-attribute?:
rd.
Step 2.5.2. If any {entry-information) of tnl contains an $options),op then perform
steps 2.5.2.1 and 2.5.2.2.
Step 2.5.2.1. Each element of tnl must contain an foptions?.
Note : a check for consistency is made in copy-descriptors.
Step 2.5.2.2. Append to al an
fattribute+:
OP.
Step 2.6. If dcl has a subcomponent, append C;,p to dcl. Append d to dcl.
Step 2.7. Delete from nlc all those elements of which there is a copy in tnl.
Step 3. Return un.
4.3.3 COMPLETE-STRUCTURE-DECLARATIONS
A structure is specified by a hierarchical set of names that refers to a group of
individual items each of which may have a different data type. Conversely, an array is
specified by a single name referring to a group of items all of the same data type. The
component items of a structure may themselves be structures or arrays.
Operation: complete-structure-declarations
Step 1. F0r each fdeclaration-commalist)r (description-commalist?, or *generic-
description-commalistq, dl component of the {concrete-external-procedure+
perform determine-structure(d1) to obtain a Cdeclaration-commalist),
+description-commalist), or {generic-description-commalist). dlm. Replace dl by
dlm.
Step 2. Let dijl be the j'th {declaration? component of the Cconcrete-external-
procedure) that declaration-contains LIKE. For each dCj1 perform Step 2.1.
Step 2.1. Perform expand-like-attribute(dCj1) to obtain a {declaration-commalist),
dcmlijl.
Step 3. For each dCj1 perform Step 3.1.
Step 3.1. Let dl be the {declaration-commalist+ that immediately contains d[jl. Attach
a $,q followed by the elements of dcml[jl in sequence so that they
immediately follow d[jl.
Step 4. Delete all {attribute?: LIKE $unsubscripted-reference?; components of
(declaration)s in the +oncrete-external-procedure+-
Step 5. For each {declaration-commalist?, Fdescription-commalist), or Cqeneric-
description-commalist), dl component of the fconcrete-external-procedure? that
declaration-contains STRUCTURE, perform convert-to-logical-levels(dl) to obtain
all, a node of the same type as dl. Perform propagate-alignment(dl1) to obtain
d12. a node of the same type as dll. Replace dl by d12.
Chapter 4: The Translator 75
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 94

4.3.3.1 Determine-structure
Operation: determine-structure(cm1)
where cm1 is a Tdeclaration-commalist%, *description-commalist), or
+generic-description-commalist).
result: a {declaration-commalist)r a (description-commalist), or Cgeneric-
description-commalist).
Step 1. Let cmlc be a copy of cml. Let eCj1 be the j"th immediate component of cmlc that
is not C,). Let n be the number of such components.
Step 2. For each etjl that immediately contains a (level),lv, perform Steps 2.1 through
2.4.
step 2.1.
step 2.2.
step 2.3.
Step 2.4.
Step 3. For
(1)
(2)
(3)
(4)
(5)
(6)
(7)
Iv must not be 0.
If j is less than n and e[j+ll immediately contains a Clevela whose numeric
value is greater than that of Iv, then attach STRUCTURE to e[jl.
If eCj1 declaration-contains LIKE then attach STRUCTURE to eijl. In this
case, eCj1 must not contain more than one instance of LIKE and cml must be a
<declaration-commalist).
If the numeric value of Iv is greater than one, attach MEMBER to eijl.
each eCj1, all of the following must be false:
eCj1 immediately contains $level~ and does not declaration-contain STRUCTURE
or MEMBER.
eCj1 immediately contains {level? whose value is 1 and eij3 declaration-
contains MEMBER.
eljl declaration-contains STRUCTURE or MEMBER and does not immediately
contain <level).
eCj1 declaration-contains STRUCTURE, does not declaration-contain LIKE and
either j is equal to n or eCj+ll does not declaration-contain a {level)
whose value is greater than the {level) declaration-component of e[jl.
eCj1 declaration-contains MEMBER and either j is equal to one or eCj-11 does
not declaration-contain MEMBER or STRUCTURE.
eCj1 declaration-contains STRUCTURE and LIKE, j is less than n, and e[j+ll
declaration-contains MEMBER and a {level? whose numeric value is greater
than the numeric value of the (level) declaration-contained in eCj1.
eCj1 declaration-contains LIKE but not a Qevel).
Step 4. Return cmlc.
4.3.3.2 Expand-like-attribute
Operation: expand-like-attribute(d)
where d is a $declaration).
result: a {declaration-commalist).
step 1. d declaration-contains an
{attribute):
LIKE
{unsubscripted-reference),r.
76
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 95

Step 2. Perform find-applicable-declaration(r) to obtain Id. Id must be a qdeclarationq
that declaration-contains STRUCTURE and must not declaration-contain LIKE.
Step 3. Let la be immediately contained in the {declaration-commalist),dcml. Let eCj1
be the j'th element of dcml that is not C.). Let k be such that e[kl is
identical to Id and let n be the number of elements of dcml that are not C,).
Step 4. Let m be the numeric value of the immediately contained {level) of d. Let Iv be
the numeric value of the immediately contained <level+ of eCk1 and let cf be the
numeric value (m-lv).
Step 5. Let cl be a $.declaration-commalist) with no elements. Let i be k+l. Let t be the
smallest integer greater than k such that one of the following is true: t is
equal to n, eCt+ll does not declaration-contain MEMBER, or eCt+ll immediately
contains a *level? whose numeric value is less than or equal to Iv. Perform
Steps 5.1 through 5.3 while i is less than or equal to t.
Step 5.1. Let ec be a copy of elil. Let the numeric value of the immediately contained
Clevell) of ec be lvc. Replace this Qlevel) by one who se numeric value is
lvc+cf.
Step 5.2. Append ec to cl, appending commas where necessary.
Step 5.3. Let i be i+l.
Step 6. cl must not contain any instance of LIKE.
Step 7. Return cl.
4.3..3.3 Convert-to-loqical-levels
Operation: convert-to-loqical-levels (cm11
where cm1 is a {declaration-commalist$-, (description-commalist*,
or Tqeneric-description-commalist*.
result: a *declaration-commalist?, {description-commalist+,
or {generic-description-commalist).
Step 1. Let cmlc be a copy of cml. Let e[jl be the j'th component of cmlc that is not
4,) and let m be the number of such components. Let lCj1 be the {level)
declaration-contained in e[jl, if it exists, and let n[jl be the numeric value
of lrjl.
Step 2.
Case 2.1. There exists an integer t less than m such that eCt3 and eCt+ll each
declaration-contains a $level+, and n[t+ll is greater than nCt3+1.
Let j be the least such t.
Case 2.2. (Otherwise).
Return cmlc.
Step 3. Let jl be the least integer greater than j such that either jl is equal to m, or
e[jl+ll does not declaration-contain MEMBER, or n[jl+ll is less than or equal to
nIjl+l. For i=j+l,... ,jl, perform Step 3.1.
Step 3.1. Replace l[il by a {level+ whose numeric value is n[il-1.
Step 4. Go to Step 2-
Chapter 4: The Translator 77
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 96

4.3.3.4 Propaqate-aliqnment
Operation: propaqate-aliqnment(cmI)
where cm1 is a Cdeclaration-commalist?, {description-commalist*,
or Tgeneric-description-commalistf.
result: a sdeclaration-commalistJ+, <description-commalist)r
or {generic-description-commalist).
step 1. Let cmlc be a copy of cml. Let eCj1 be the j'th component of cmlc that is not
$.) and let m be the number of such components. Let lljl be the <level)
declaration-contained in eCj1, if it exists, and let n[jl be the numeric value
of lCj1.
Step 2. For i=l,...,m, perform Steps 2.1 and 2.2.
step 2.1. eIi1 must not declaration-contain both ALIGNED and UNALIGNED.
Step 2.2.
Case 2.2.1. etil declaration-contains MEMBER, and eCi1 declaration-contains neither
ALIGNED nor UNALIGNED.
Let k be the greatest integer such that k is less than i and n[kl is
equal to nCil-1. If eCk1 declaration-contains ALIGNED or UNALIGNED,
then attach ALIGNED or UNALIGNED, respectively, to eCi1.
Case 2.2.2. (Otherwise).
No action.
Step 3. For i=l,...,m, perform Step 3.1.
step 3.1. If eCil contains STRUCTURE and ALIGNED, or STRUCTURE and UNALIGNED, delete
ALIGNED or UNALIGNED, respectively, from eiil.
Step 4. Return cmlc.
4.3.3.5 Find-applicable-declaration
Operation: find-applicable-declaration(r)
where r is an #identifier), {unsubscripted-reference?, {basic-reference),
or {reference).
result: a <declaration) or <absent+.
Step 1.
Case 1.1. r is an <identifier?.
Let id1 be an {identifier-list): r.
Case 1.2. 1: is an Cunsubscripted-reference). Append, in order, a copy of each
{identifier) component of r to an (identifier-list),idl.
Case 1.3. r is a <basic-reference).
Append, in order, a copy of each <identifier) component of r which is not a
component of an <arguments?, in order, to an sidentifier-lisQ,idl.
case 1.4. 1: is a <reference).
Let br be the -sic-reference) immediate component of r. Perform find-
applicable-declaration (br) to obtain d. Return d.
78
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 97

Step 2. Let b be the concrete-block that block-contains r. If r is contained in a
{statement-name),sn then perform Step 2.1.
Step 2.1. If sn is contained in a {unit+ which simply contains Fentry-statement2 or sn
is contained in a {prefix-list) immediate component of a {procedure? then
let b be the concrete-block which block-contains b.
Step 3. Let id be the rightmost $identifier* component of idl.
Step 4. Let dl be a $declaration-designator-list) each component of which designates a
{declaration) that is a concrete-block-component of b and that immediately
contains an {identifier) that is equal to id.
Step 5. Delete from dl any component that designates a $declaration),d such that find-
fully-qualified-name(d) returns an {identifier-list),didl such that id1 does not
contain an ordered sublist of the $identifier+s contained in didl.
Step 6.
Case 6.1. dl is empty.
Let b be the concrete-block that block-contains b. If there is no such block
then return <absent+; otherwise, go to Step 4.
Case 6.2. dl contains a single component.
Let d be the {declaration? designated by the single component of dl. Return
d.
Case 6.3. dl contains more than one component.
dl must contain exactly one component that designates a /declaration),d such
that find-fully-qualified-name(d) returns an {identifier-list) equal to idl.
Return d.
4.3.3.6 Find-fully-qualified-name
Each {declaration) has a fully qualified name associated with it. If it is an array or
item declaration then this is a single Cidentifier). If the {declaration? is a member of
a structure then it may have as many $identifier$s as its logical level-number indicates
its depth of embedding to be.
Operation: find-fully-qualified-name(d)
where d is a {declaration+.
result: an {identifier-list).
Case 1. d declaration-contains MEMBER.
step 1.1. Let dcml be the {declaration-commalistg that immediately contains d. Let
dl be the rightmost preceding Tdeclaration? of dcml that declaration-
contains a Clevelq whose numeric value is less than the numeric value of the
Qevelp of d.
step 1.2. Perform find-fully-qualified-name(d1) to obtain an {identifier-list?,idl.
step 1.3. Append the {identifier) immediate component of d to idl.
step 1.4. Return idl.
Case 2. d does not declaration-contain MEMBER.
Step 2.1. Let id be the (identifier) immediate component of d.
step 2.2. Return {identifier-list): id.
Chap&r 4: The Translator 79
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 98

4.3.4 CONSTRUCT-CONTEXTUAL-DECLARATIONS
Certain contexts in a procedure specify the attributes of an identifier appearing in
those contexts. If an identifier is not explicitly declared, but is used in such a
context, then it is contextually declared and a declaration for it is introduced into the
{concrete-external-procedure). All the attributes implied by the context are added onto
this generated declaration.
Operation: construct-contextual-declarations
Step I. Let cep be the {concrete-external-procedure). Let u be a
$unit):
(declare-statement):
DECLARE
{declaration-commalistP,dcml
C;?;;
where dcml contains no elements.
Step 2. For each $identifier),id in cep perform Steps 2.1 through 2.3.
Step 2.1. Let attrs be *absent>.
Step 2.2.
Case 2.2.1. id is a component of a
$reference),r:
Cbasic-reference):
$identifier),id;;
where r has no other components.
Step 2.2.1.1.
Case 2.2.1.1.1. r is an immediate component of an sin-option) or r is an
immediate component of a {data-attributeT,da and da contains
OFFSET.
Let attrs be an
{attribute-list):
{attril utep:
Fdata-attribute):
AREA;;
Cattribute?:
VARIABLE.
Case 2.2.1.1.2. r is an immediate component of a {call-statement).
This case must not occur.
Case 2.2.1.1.3. r is an component of a {file-option+, {copy-option), or
Tnamed-io-condition).
Let attrs be an
fattribute-list):
(attribute?:
$data-attribute):
FILE;;
Cattribute):
CONSTANT.
80
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 99

Case 2.2.1.1.4. r is an immediate component of a {set-option) or a Clocator-
qualifier?, or r is an immediate component of an Cattribute?
that contains BASED.
Let attrs be an
{attribute-list?:
Cattribute?:
Cdata-attribute):
POINTER;;
{attribute?:
VARIABLE.
Case 2.2.1.1.5. (Otherwise).
No action.
Case 2.2.2. id is an immediate component of a {programmer-named-conditions).
Let attrs be an
<attribute-list+:
Cattribute?:
CONDITION.
Case 2.2.3. id is the only component of the sbasic-reference) of a
{reference):
{basic-reference):
$identifierp,id;
<arguments-list+;
where the {reference) has no other immediate components.
Let attrs be an
{attribute-list?:
Cattribute>:
BUILTIN.
Case 2.2.4. (Otherwise).
No action.
Step 2.3. If attrs is not <absent+ then perform find-applicable-declaration(id) to
obtain d. If d is <absent+ then perform Steps 2.3.1 through 2.3.3.
Step 2.3-l. Let d be a
{declaration?:
id
attrs.
Step 2.3.2. If an element of dcml immediately contains an fidentifier? equal to id
then the {attribute-list+ of this {declaration) must equal attrs.
Step 2.3.3. If no element of dcml immediately contains an Cidentifier) equal to id
then append d to dcml.
Step 3. If dcml contains any elements, append u to the {unit-list) of the {procedure)
immediately contained in cep.
Chapter 4: The Translator 81
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 100

4.3.5 CONSTRUCT-IMPLICIT-DECLARATIONS
Identifiers that do not resolve to any declaration or that have not had declarations
constructed because of the context in which they appear are implicitly declared.
Operation: construct-implicit-declarations
Step 1. For each tree of the form
Freference):
Cbasic-reference):
{identifier#,id;;
where the {reference) has no other components, or that is the only Cidentifiert
of an $unsubscripted-reference?, or that is an immediate component of an
<allocation), {freeing), or {locate-statement), perform Step 1.1.
Step 1.1. Perform find-applicable-declarationtid) to obtain d. If d is <absent+ then
perform Step 1.1-l.
Step l-1.1- Let u be a
{unit):
{declare-statement?:
DECLARE
{declaration-commalist):
{declaration?:
id;;
tit-
Append u to the {unit-list? of the {procedure) immediately contained in
the {concrete-external-procedure+.
4.3.6 COMPLETE-DECLARATIONS
Operation: complete-declarations
Step 1. For each {default-attributes),das component of the {concrete-external-
procedure), perform Steps 1.1 and 1.2.
Step 1.1. Let d be a Tdeclaration): x;, where x is a copy of the {attribute-list) of
das (d is a partial $declaration*l.
Step 1.2. Perform test-attribute-consistency(d,fabsent,) to obtain tv. tv must not be
<false>.
Step 2. For each $declarationq, {description), or <generic-description)rd component oi
the {concrete-external-procedure?, perform step 2.1.
Step 2.1. Perform test-attribute-consistency(d) to obtain tv. tv must not be ffalse9.
Step 3. Perform append-system-defaults.
Step 4. Each *default-attributes? component of the Cconcrete-external-procedure) must
not declaration-contain LIKE, MEMBER, STRUCTURE, or PARAMETER.
Step 5. For each fdeclaration*,d component of the {concrete-external-procedure? perform
Steps 5.1 and 5.2.
Step 5.1. Perform apply-defaults(d).
Step 5.2. Perform check-attribute-completeness-and-delete-attributes(d).
Step 6. For each $description),d component of the {concrete-external-procedure) that
satisfies the following conditions, perform steps 6.1 and 6.2.
(1) d is contained in a <declaration+ but not in a {generic-attribute).
(2) d was not produced by Step 6.1.
82
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 101

Step 6.1. Perform apply-defaults(d).
Step 6.2. Perform check-attribute-completeness-and-delete-attributes(d).
Step 7. For each *returns-descriptor),d component of the {concrete-external-procedure)
which does not contain a {description) and which satisfies the following
conditions, perform Step 7.1.
(1) d is contained in a $declaration) but not in a {generic-attribute).
(2) d was not produced by Step 6.1 or Step 7.1.
Step 7.1. Perform apply-defaults(d).
Step 8. For each $descriptiont,d component of the {concrete-external-procedure) which is
contained in a (declaration$ but not in a {generic-attribute), perform check-
attribute-completeness-and-delete-attributes(d).
Step 9. For each tree of the form
{procedure):
Cprefix-list):
{prefix?:
Cstatement-narne),sn;;
{procedure-statement)
[Cunit-list21
Cending;);
or of the form
{unit):
$statement-name-list?:
{statement-name*,sn;
{entry-statement);
perform Steps 9.1 through 9.3.
Step 9.1. Let id be the {identifier? immediately contained in sn.
Step 9.2. Perform find-applicable-declaration(id) to obtain d. d must be a
$declaration).
step 9.3. Perform copy-descriptors(d).
4.3.6.1 Test-attribute-consistency
Operation: test-attribute-consistency(d,dasI
where d is a {declaration+, CdescriptionP, or {generic-description+,
das is a [{default-attributes?].
result: *true> or <false?-.
step 1. If das is tabsent, then :
Case 1.1. d is a $declaration*.
For each pair of CattributePs, al and a2, which are declaration-components
of d, perform test-invalid-duplicates(al,a2> to obtain tv. tvmust not be
*true*.
Case 1.2. d is a {description).
For each pair of {data-attributebs, al and a2, which are declaration-
components of d, perform test-invalid-duplicates(alra2) to obtain tv. tv
must not be *trues.
Chapter 4: The Translator 83
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 102

Case 1.3. a is a {generic-description+.
For each pair of {generic-data-attributegs. al and a2. which are
declaration-components of a, perform test-invalid-duplicates(al,a2) to
obtain tv. tv must not be <true>.
Step 2.
Case 2.1. das is fabsent,.
Let akl be an {attribute-keyword-list) consisting of a copy of each keyword
declaration-component of d.
Case 2.2. das is not <absent,.
step 2.2.1. Let al be an {attribute-list? consisting of copies of each fattribute?
declaration-component of das, and each {attribute) or {data-attribute)
declaration-component of d where d is a {declaration) or Tdescription?
respectively.
Step 2.2.2. For each pair of {attribute? components of al, al and a2, perform test-
invali+duplicates(al,a2) to obtain tv. If tv is <true,, return
<false+.
Step 2.2.3. Let akl be an <attribute-keyword-list+ consisting of a copy of each
keyword declaration-component of al.
Step 3. Replace all multiple occurrences of a given keyword in akl by a single
occurrence of that keyword.
Step 4. If akl contains GENERIC or MEMBER and also contains EXTERNAL, return <false+.
Step 5. If akl contains STRUCTURE and ALIGNED, or contains STRUCTURE and UNALIGNED,
return <false>.
Step 6.
Case 6.1. d is a {description? or {generic-description).
If the set of all keywords in akl is not a subset of the keywords that form
the concrete-representation of some tree whose root-node is {consistent-
description), return <false+.
Case 6.2. d is a partial {declaration) produced by the operation create-constant.
If the set of all keywords in akl is not a subset of the keywords that form
the concrete-representation of some tree whose root-node is fconsistent-
literal-constant), return ffalse,.
Case 6.3. d is a $declarationZ#.
If the set of all keywords in akl is not a subset of the keywords that form
the concrete-representation of some tree whose root-node is {consistent-
declaration*, return <false>.
Step 7. Return <true>.
<consistent-declaration)::= {scope) (declaration-type?
{scope+::= EXTERNAL 1 INTERNAL
{declaration-type)::= {variable? 1 Tnamed-constant? 1 BUILTIN 1 CONDITION 1 GENERIC
{variable)::= VARIABLE {storage-type? Cdata-descriptiong
*storage-type)::= $storage-class) I DEFINED [POSITION1 1 PARAMETER I MEMBER
{storage-class*::= AUTOYATIC I BASED I CONTROLLED I STATIC
{data-description*::= CDIMENSIONI Calignment) {{data-type? [INITIAL] I STRUCTURE)
84
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 103

{alignment)::= ALIGNED 1 UNALIGNED
{data-type)::= <computational-type) 1 {non-computational-type)
$computational-type)::= {arithmetic) 1 {string) 1 qpictured)
{non-computational-type)::= AREA 1 sentry) 1 FILE 1 FORMAT [LOCAL) 1 LABEL [LOCAL3 1
{locator)
{arithmetic)::= fREAL 1 COMPLEX) {BINARY I DECIMAL3 {FIXED 1 FLOAT1 PRECISION
+string)::= 1CHARACTER 1 BIT) {VARYING 1 NONVARYING)
{pictured+::= PICTURE [REAL I COMPLEX1
#entry)::= ENTRY [RETURNS] [OPTIONS]
{locator)::= POINTER 1 OFFSET
{named-constant)::= CONSTANT {ENTRY 1 FILE Cfile-description-set) I FORKAT I LABEL)
IDIMENSION
{file-description-set)::= [ENVIRONMENT] {{stream-set) I {record-set*3
+stream-set)::= STREAM iINPUT I OUTPUT [PRINTI)
(record-set)::= RECORD {INPUT I OUTPUT I UPDATE) {{sequential-set* I <direct-set*)
+.equential-set)::= SEQUENTIAL CREYEDI
{direct-set)::= DIRECT KEYED
{consistent-literal-constant)::= CONSTANT {{arithmetic) 1 BIT I CHARACTER3
{consistent-description)::= {data-description+ [MEMBER]
4.3.6.2 Test-invaliddupLicates
Operation: test-invalid-duplicates(al,aZ)
where al and a2 are $attribute)s or {data-attribute?s or are both
{generic-data-attributeas.
result: <true, or <false+.
Step 1. Let kl and k2 be the leftmost keyword components of al and a2 respectively.
Step 2.
Case 2.1. kl and k2 are equal.
Case 2.1.1. al and a2 have no terminal components other than kl and k2.
Return *false>.
Case 2.1.2. kl (respectively. k2) is the only terminal component of al (respectively
a21 but k2 (respectively, kl) is not the only terminal component
of a2 (respectively, al).
Return *false>.
Case 2.1.3. (Otherwise).
Return *true>.
Case 2.2. kl and k2 are not equal.
Return <false>.
Chapter 4: The Translator 85
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 104

4.3.6.3 Append-system-defaults
Operation: append-system-defaults
Step 1. If the Cprocedure2.p immediately contained in the {concrete-external-procedure?
does not have as a concrete-block-component a (default-statement+ that contains
SYSTEM, append a
{unit):
{default-statement?:
DEFACJLT SYSTEM C;?;;
to the <unit-list+ of p-
Step 2. Modify the {symbol-list) below by replacing dl, d2, d3, d4, and d5 by
implementation-defined integers to obtain Id.
"/* ENTRY DEFAULTS */
DEFAULT (RETURNS) ENTRY;
/* FILE DEFAULTS */
DEFAULT (DIRECTIINPUTIKEYEDI
OUTPUT[PRINTlRECORDlSEQUENTIALlSTREAMlUPDATE) FILE;
/* ARITHMETIC DEFAULTS */
DEFAULT (-CONSTANT & -PICTURE) FIXED, BINARY, REAL;
DEFAULT (FIXED & BINARY & --CONSTANT) PRECISION(dl,O);
DEFAULT (FIXED & DECIMAL & +ZONSTANT) PRECISION(d2,O);
DEFAULT (FLOAT & BINARY & -CONSTANT) PRECISION(d3);
DEFAULT (FLOAT & DECIMAL & +ZONSTANT) PREcIsIoN(d4);
/* STRING AND AREA DEFAULTS */
DEFAULT (CHARACTER) CHARACTER(l), NONVARYING;
DEFAULT (BIT) BIT(l), NONVARYING;
DEFAULT (AREA) AREA(d5);
DEFAULT (POSITION) POSITION(l);
/* SCOPE AND STORAGE CLASS DEFAULTS */
DEFAULT ((ENTRYIFILE) &
(AUTOMATICIBASEDIDEFINEDIPARAMETERISTATICICONTROLLED(MEMBERI
ALIGNEDlUNALIGNEDlINITIAL)) VARIABLE;
DEFAULT ((ENTRYIFILE) & RANGE(*)) CONSTANT;
DEFAULT (RANGE(*) & +CONSTANT) VARIABLE;
DEFAULT (CONDITIONI ((FILEIENTRY) & CONSTANT)) EXTERNAL;
DEFAULT (RANGE(*)) INTERNAL;
DEFAULT (VARIABLE & EXTERNAL) STATIC;
DEFAULT (VARIABLE) AUTOMATIC;
/* ALIGNMENT DEFAULTS */
DEFAULT (tCRARACTERIBITIPICTURE) & -.CONSTANT) UNALIGNED;
DEFAULT (+ZONSTANT) ALIGNED;"
86

86
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 105

Step 3. For each *unit), u component of the $concrete-external-procedure) where u
immediately contains a {default-statement) that contains SYSTEM, perform Steps
3.1 and 3.2.
Step 3.1. Perform parse(ld,$unit-list?) to obtain ul.
step 3.2. Replace the single $unitq,u by the sunit? immediate components of ul such
that all of the immediate components of ul are effectively inserted, in
order, in place of u.
4.3.6.4 Apply-defaults
Operation: apply-defaults(d)
where d is a {declaration), or a fdescriptionq, or a Treturns-descriptor)
with no {description> component.
Step 1. If d is a <returns-descriptor?, then attach to d a {description),decl with no
components, and the surrounding parentheses as required; otherwise let decl be
d.
Step 2. For each {default-statement*,dft component of the Cconcrete-external-procedure?
taken in left-to-right order perform Steps 2.1 through 2.3.
Step 2.1. dft must be a block-component of the Cprocedure) immediate component of the
{concrete-external-procedure).
step 2.2. If dft immediately contains NONE then go to Step 3.
Step 2.3. Let dpe be the (predicate-expressionp component of dft. Perform test-
default-applicability(dpe,declJ to obtain tv. If tv is <true+ perform steps
2.3.1 and 2.3.2.
Step 2.3.1. dft must not contain ERROR.
Step 2.3.2. For each <default-attributes),das component of dft in left-to-right
order perform Steps 2.3.2.1 through 2.3.2.3.
Step 2.3.2.1. Perform test-attribute-consistency(decl,das) to obtain tv.
Step 2.3.2.2. If tv is iB+ and decl is a {declaration), then append to the
{attribute-list) in decl copies of the {attribute) simple
components of das.
Step 2.3.2.3. If tv is t-9 and decl is a {description?, then append to the
<data-attribute-list) in decl copies of the (data-attribute)
simple components of das.
step 3. If d is a Creturns-descriptor?, then decl must contain at least one subnode.
4.3.6.5 Test-default-applicability
Operation: test-default-applicability(dpe,decl)
where dpe is a Cpredicate-expression+, {predicate-expression-three),
{predicate-expression-two?, {predicate-expression-one*, {range-
specificationp, or fattribute-keyword).
decl is a {declaration+, or a (description*.
result: <true, or ifalse,.
Case 1. dpe immediately contains a $1).
Let pe be the Cpredicate-expression) simple component of dpe, and p3 be the
Cpredicate-expression-three? simple component of dpe. If either, or both, test-
default-applicability(pe,decl) or test-default-applicability(p3,decl) yield
Ctrue+ then return <true>; otherwise return <f-9.
Chapter 4: The Translator 87
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 106

Case 2. dpe immediately contains an &.
Let p3 be the {predicate-expression-three+ simple component of dpe, and p2 be
the Cpredicate-expression-two? simple component of dpe. If both test-default-
applicability(p2,decl) and test-default-applicability(p3,decl) yield - <true,,
then return <true>; otherwise return <false+.
ease 3. dpe immediately contains a 7.
Let pe be the other simple component of dpe. Perform test-default-
applicability(pe,decl) to obtain tv. If tv is <true> then return <false,;
otherwise return <true,.
Case 4. dpe is a {range-specification+-,rs.
Case 4.1. rs contains an +.
If decl is a {declaration? which immediately contains an {identifier? return
*true+, otherwise return <false+.
Case 4.2. rs contains an +identifierp,id.
Step 4.2.1. If decl is a Cdescription) or a $declaratiori+ that does not immediately
contain an Cidentifier), then return <false,.
Step 4.2.2. Let idd be the {identifier* immediate component of decl. Compare the
terminal nodes of id and idd, taken in order, until the terminal nodes
of id have been exhausted. If all the comparisons are equal then return
*true>; otherwise return <false>.
Case 4.3. rs contains {letter),11 $:) $letterg,l2.
Step 4.3.1. If decl is a {description? or a {declaration) that does not immediately
contain an <identifier), then return <false+.
Step 4.3.2. Let dl be the first {letter? of the {identifier) simple component of
decl. If dl is, or is after 11 and is, or is before 12 in the English
alphabet then return t-3; otherwise return <false+.
Case 5. dpe is an $attribute-keywordp,ak.
If decl declaration-contains an $attribute),atr or a {data-attributep,atr such
that the leftmost keyword, k of atr is equal to ak then return <true+; otherwise
return *false,.
Case 6. (Otherwise).
Let pe be the {predicate-expression) component of dpe. Perform test-default-
applicability(pe,declJ to obtain tr. Return tr.
4.3.6.6 Copy-descriptors
Operation: copy-descriptors(d)
where d is a (declaration+.
Step 1. Let e be the Cdata-attribute) declaration-component of d that immediately
contains ENTRY.
Step 2.
Case 2.1. d declaration-contains a {dimension-attribute).
Let ep [il, i=l,...,n, be the n occurrences of
{statement-name):
$identifier?,id
({signed-integer-commalist)) C:);
such that d is obtained by performing find-applicable-declaration(id).
88
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 107

Case 2.2. (Otherwise).
Let ep[ll be the single occurrence of a
{statement-name*:
$identifier),id $I:$-;
such that d is obtained by performing find-applicable-declaration0. Let
n be 1.
Step 3. For each ep[il, i=l,...,n, perform Steps 3.1 through 3.4.
Step 3.1.
Case 3.1.1. ep[il is contained in a {prefix-list+ immediate component of a
$procedure+,p.
Let es be the immediately contained sprocedure-statement) of p-
Case 3.1.2. ep[il is contained in a {statement-name-list) immediate component of a
$unit),u.
Let es be the immediately contained Tentry-statement? of u.
Step 3.2.
Case 3.2.1. es contains a Cparameter-name-commalisyrpl.
Step 3.2.1.1. Let pnCj1 be the {identifier) of the j'th immediately contained
{parameter-name? component of pl. Let m be the number of such
components.
For 3 2 1 ;a;hthEE;;;r3 $=',,;- irmr taken in order, perform Steps
. . . . . . . . .
Step 3.2.1.1-1. Perform find-applicable-declaration(pnIjl1 to obtain pd. pd
must be a {declaration).
Step 3.2.1.1.2. Let dcml be the {declaration-commalist) that immediately
contains pd- Let dcm[kl be the k'th {declaration? component
of dcml. Let there be kmx such components and let k be such
that dcm[kl is identical to pd.
step 3.2.1.1-3. Let knd be the minimum integer greater than or equal to k
such that one of the following is true:
(1) knd is equal to kmx;
(2) dcmIknd+lI does not contain {level);
(3) dcm[knd+ll contains a {level? whose {integer? has the
value 1.
step 3-2.1.1.4. For each dcmtinxl, inx=k,...,knd, taken in order, perform
Steps 3.2.1.1.4.1 through 3.2-1-1-4-5.
step 3.2.1.1.4.1. Let pnd be a copy of dcmlinxl. Delete from pnd
any occurrence of an
rattribute):
PARAMETER.
Chapter 4: The Translator 89
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 108

Step 3.2.1.1.4.2. If dcmiinxl contains an
{attribute):
Cdata-attribute),da:
OFFSET
(
$reference),r
1;;
then perform test-offset-in-description(da3 to
obtain tv. If tv is *trim+, then replace the
sdata-attribute) in pnd corresponding to da by
$data-attribute): OFFSRT.
Note: This language feature allows pointer to offset
conversion to be performed on calls to internal
procedures only if the area to which the offset is
relative is known to the calling block.
Step 3.2.1.1.4.3. If pnd contains a
sdata-attribute),da:
ENTRY
idescription-commalist)
1;
then replace da with a
Fdata-attribute):
ENTRY.
Step 3.2.1.1.4.4. If pnd contains an
$attribute+,atr:
fdata-attribute):
<returns-descriptor+;;
then delete atr.
Step 3.2.1.1.4.5. Append, with intermediate $,)s as required, to a
{description-commalist),desCilr a {description)
simply containing copies of the {level) and {data-
attribute) simple components of pnd. pnd must not
contain any {extent-expression@ containing an
{identifier).
Case 3.2.2. es does not contain a {parameter-name-commalist).
Let desli.1 be .?absent,.
Step 3.3. If es contains a <returns-descriptor), let rdCi1 be that {returns-
descriptor); otherwise, let rdiil be <absent+.
Step 3.4. If es contains an Captions) then let opsCi1 be a copy of that $options);
otherwise let opsCi1 be <absent+.
Step 4.
Step 4.1. Let xCi1 be desiil for i=l ,...,n. Perform Steps 4.5 and 4.6.
Step 4.2. Let xCi1 be rd[il for i=l,...,n. Perform Steps 4.5 and 4.6.
Step 4.3. Let xCi.1 be ops[il for i=l,...,n. Perform Steps 4.5 and 4.6.
Step 4.4. Go to Step 5.
90
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 109

Step 4.5. It must be possible to make copies cxtil of all the xIi1 in which the
ordering of the immediate components of {data-attribute-list+ and soptions-
specification@ has been altered such that all the cxCi1 are equal, except
for the subnodes of any {extent-expression)s.
Step 4.6. For each pair of corresponding {extent-expression+s, extl and ext2, in each
pair of members of the set cxCi1, perform test-descriptor-extent-
expressions(extl.ext2) to obtain tv , which must be *true+.
Step 5. If desfll is not <absent,, replace e with a
$data-attribute):
ENTRY
(
desCl1
1.
Step 6. If d declaration-contains a {returns-descriptorp,drd, then for i=l,...,n,
replace rdlil (which must not be <absent,) by drd.
Case 6.1. d is not block-contained in the fconcrete-external-procedure?.
For each {data-attribute),da: OFFSET($reference+); in rdCi1, Perform Step
6.1.1.
Step 6.1.1. Perform test-offset-in-description(da7 to obtain tv, which must be
<notrim+.
Case 6.2. (Otherwise).
For each Cdata-attribute?: OFFSET($referenceq); in drd, delete the
({reference*).
Step 7. If d declaration-contains an $optionsp,dops, then for i=l,...,n, replace ops[il
(which must not be <absent+) by dops.
4.3.6.7 Test-offset-in-description
This operation tests whether the (freference?) from an OFFSET attribute has to be trimmed
because it would fall within the scope of a different declaration if copied into an ENTRY
declaration in the surrounding block.
Operation: test-offset-in-description(da)
where da is a {data-attribute): OFFSET(jXeference)).
result: *trim+ or <notrim+.
Step 1. For each +referencet,r contained in da, perform Step 1.1.
Step 1.1. Perform find-applicable-declaration(r) to obtain rd. rd must be a
{declaration*. If rd is block-contained in the same block as da, return
ftriti.
Step 2. Return <notrim%.
Chapter 4: The Translator 91
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 110

U.3.6.8 Test-descriptor-extent-expressions
Operation: test-descriptor-extent-expressions(extl,extZ)
where extl and ext2 are sextent-expression)s.
result: <true> or *false,.
Step 1. If extl or ext2 contains sidentifierg, return <false>.
Step 2. Let ccl and ce2 be the {expression) immediate components of extl and ext2.
Perform create-expression(ce1) to obtain an <expression>,el, and create-
expression(ce2) to obtain an <expression>,eZ.
Step 3. Perform evaluate-restricted-expression and evaluate-restricted-
expression(e2) to obtain vl and v2 respectively. If vl or v2 is not a
<constant> having <computational-type>, return <false,. Otherwise, perform
evaluate-expression-to-integertell and evaluate-ex$ZGZion-to-integer(e2) to
obtain <integer-value+s il and i2 respectively. If il and i2 are equal, return
t-s; otherwise return <false+.
4.3.7 VALIDATE-CONCRETE-DECLARATIONS
Various checks are applied to the Cconcrete-external-procedure+ to ensure that a valid
set of declarations has been generated. Each declaration must be unique within its own
scope. A check is also made to ensure that $declarationZ+s of INTERNAL CONSTANT ENTRY,
CONSTANT FORMAT, and CONSTANT LABEL were constructed by the operation construct-
statement-name-declarations.
Operation: validate-concrete-declarations
step 1.
Step 2.
Step 3.
The {concrete-external-procedure) must not contain two $declaration)s, dl and
d2, such that dl and d2 are block-components of the same block, and find-fully-
qualified-name(dl1 yields the same result as find-fully-qualified-name(d2).
(dl and d2 are duplicate declarations).
The {concrete-external-procedure) must not contain a $referenceq,r, fbasic-
reference),r, or $unsubscripted-referencet,r such that find-applicable-
declaration(r) yields <absent>.
(There is no {declaration) for r).
The <concrete-external-procedure> must not contain a $declaration),d that
declaration-contains LABEL and CONSTANT, or FORMAT and CONSTANT, or INTERNAL and
ENTRY and CONSTANT, unless d was created by the operation construct-statement-
name-declarations.
92
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 111

4.3.7.1 Check-attribute-completeness-and-delete-attributes
Operation: check-attribute-completeness-and-delete-attributes(d)
where d is a {declaration) or a {description).
Step 1. For each distinct keyword. k which is a declaration-component of d perform Step
1.1.
Step 1.1. If there is an $attribute),att or a {data-attribute),att declaration-
component of a, such that att declaration-contains k, but not as its sole
terminal component, then delete all {attribute) or Tdata-attribute?
declaration-components of d which declaration-contain k, except for att;
otherwise, delete all but one of the fattribute? or {data-attribute?
declaration-components of d which declaration-contain k.
Step 2. a must not declaration-contain an {attribute? or Cdata-attribute+ with AREA,
BIT, CHARACTER, DIMENSION, GENERIC, INITIAL, PICTURE, POSITION, PRECISION, or
RETURNS as its sole terminal node.
Step 3. If d declaration-contains EXTERNAL, it must not declaration-contain AUTOMATIC,
BASED, DEFINED, PARAMETER, or BUILTIN.
Step 4. If d declaration-contains EXTERNAL and CONSTANT, it must not declaration-contain
FORMAT or LABEL.
Step 5. If a declaration-contains EXTERNAL and CONSTANT and ENTRY, it must not
declaration-contain DIMENSION.
Step 6. If a is a $description), d must not declaration-contain INITIAL.
Step 7. If a is a Cdeclarationt which declaration-contains DEFINED or PARAMETER (or
declaration-contains MEMBER and the rightmost preceding $declaration+ whose
{level) has the value one declaration-contains DEFINED or PARAMETER), then d
must not declaration-contain INITIAL.
Step 8. If a is a $declaration) which declaration-contains STATIC and either ENTRY,
FORMAT, or LABEL (or declaration-contains MEMBER and either ENTRY, FORMAT, or
LABEL, and the rightmost preceding {declaration+ whose Clevel) has the value one
declaration-contains STATIC), then d must not declaration-contain INITIAL.
Step 9. Let akl be an fattribute-keyword-list$ consisting of copies of the keyword
declaration-components of d.
Step 10. Delete from akl any keyword that can be contained in a tree whose root-node is
{Eile-description-set).
Step 11.
Case 11.1. d is a fdescription).
There must exist a tree whose root-node is {consistent-description) and
whose concrete-representation consists of the same set of keywords as
are contained in akl.
Case 11.2. d is a {declaration?.
There must exist a tree whose root-node is {consistent-declaration) and
whose concrete representation consists of the same set of keywords as
are contained in akl.
Chap'ter 4: The Translator 33
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 112

4.4 Create-abstract-equivalent-tree
Where the Concrete and Abstract Syntaxes are similar a simple transformation generates a
specific abstract tree from the given concrete one. Essentially this consists of
ignoring those concrete terminals which represent the 'punctuation" of the concrete
program, and transforming each concrete node into its abstract equivalent. Where the
syntaxes are not similar an operation specifies the translation.
Operation: create-abstract-equivalent-tree(ct)
where ct is a tree belonging to a category of the Concrete Syntax.
result: a tree belonging to a related category of the Abstract Syntax.
or <none>.
Case 1. ct is an $allocation), (assignment-statement?, fbegin-block), {constant),
$expressio*, Cformat-iteration+, {freeing*, (identifier?, (initial-element$l
<if-statement), {group), *locate-statement), {on-statement), or rpicture).
Perform create-f(&) to obtain abt, where "$f)" is the name of the category of
ct. Return abt.
Case 2. ct belongs to a terminal category.
Case 2.1. ct is a keyword and there is a terminal category whose name is of the form
"<cn>" where cn is the lowercase equivalent of category name of ct (e.g., if
ct% FIXED, cn is "fixed").
Return a <cn>. -
Case 2.2. ct is an +.
Return an <asterisk>.
Case 2.3. (Otherwise).
Return <none>.
Case 3. The category-name of ct is of the form "$cn-lisQn or l $cn-commalist+", where
"cn" is some name.
Let x be a <cn-list>. For each $cn),y, in ct, taken in order, perform rlreate-
abstract-equivalent-tree(y) to obtain Z, and append z to x- Return x-
Case 4. ct is a {radix-factor*.
Let d be the digit in the denotation of the immediate component of ct. Return
(radix-factor>: d.
Case 5. ct is a <condition-name.
Perform create-condition(ct) to obtain abt. Return abt.
Case 6. (Otherwise).
step 6.1. Let "cn' be the name such that '$cn)' is the category-name of ct. Let x be
a <cn>. If <cn> is a terminal category, return x.
Step 6.2. For each imediate subnode, y , of ct, taken in order, perform Step 6.2.1.
Step 6.2.1.
Case 6.2.1.1. y is a $reference).
Let ref be a <variable-reference> , or a <target-reference>, or a
<subroutine-reference>, choosing the alternative that permits ref
to be attached to x as an immediate subnode. Perform create-
reference(y,ref) to obtain z. Attach z to x.
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 113

Case 6.2.1.2. y is an {unsubscripted-reference).
Perform create-reference(y,<variable-reference>) to obtain z.
Attach z to x.
Case 6.2.1.3. (Otherwise).
Perform create-abstract-equivalent-tree(y) to obtain z. If z is
not equal to *none>, attach z to x.
Step 6.3. Return x.
4.4.1 CREATION OF BLOCKS AND GROUPS
4.4.1.1 Create-procedure
Operation: create-procedure(cp3
where cp is a {procedure).
result: a <procedure>.
Step 1. cp immediately contains a Cprocedure-statement+,cps. Perform create-entry-
point(cps1 to obtain an <entry-point>,ep. Let ap be a
<procedure>:
<entry-or-executable-unit-list>,eeul:
<entry-or-executable-unit>: ep.
step 2. Perform create-block(cp,ap).
Step 3. Let pl be the {prefix-list) immediately contained in cp. Perform create-
condition-prefix-list(pl1 to obtain a <condition-prefix-list>,cpl or
fabsent+,cpl. If cpl is not fabsent,, attach cpl to ap.
Step 4. If cps simply contains RECURSIVE, then attach <recursive> to ap.
Step 5. Return ap.
4.4.1.2 Create-beqin-block
A <begin-block> is constructed in much the same way as a <procedure>, which it resembles
except for the presence of any entry or return information.
Operation: create-beqin-block (ebb)
where ebb is a {begin-block).
result: a <begin-block>.
step 1. Let abb be a <begin-block>. If the {begin-statement) in ebb contains an
(options), attach an implementation-dependent tree of type <options> to abb.
Step 2. Perform create-block(cbb,abb).
Step 3. For each <entry-point>,ep, contained in abb, abb must contain a <procedure> that
contains ep.
step 4. Return abb.
Chapter 4: The Translator 95
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 114

4.4.1.3 Create-block
Operation: create-block(cb,ab)
where cb is a {procedure) or {begin-block),
ab is a <procedure> or <begin-block>.
Step 1. For each $declaration),d that is a block-component of cb and that does not
declaration-contain MEMBER or GENERIC, perform create-declaration(d) to obtain a
<declaration>,ad, and append ad to the <declaration-list> immediately contained
in ab.
Step 2. For each <declaration>,ad component of ab such that ad contains at least one
{expression-designator?, freference-designator?, or <initial-designator> perform
replace-concrete-designators(ad).
Step 3. For each fprocedure),p that is a block-component of cb perform create-
procedure(p) to obtain a <procedure>,ap and append ap to the <procedure-list> in
ab.
Step 4. For each *format-statement),fs that is a block-component of cb, perform create-
format-statement(fs) to obtain a <format-statement>,afs and append afs to the
(format-statement-list> in ab.
Step 5. If cb immediately contains a {unit-list?, let ul be that {unit-list). Otherwise
let ul be *absent+.
Case 5.1. cb is a {procedure).
Perform create-entry-or-executable-unit-list(ul) to obtain an <entry-or-
executable-unit-list>*eul and attach eul to ab.
Case 5.2. cb is a {begin-block?.
Perform create-executable-unit-list(u1) to obtain an <executable-unit-
list>,eul and attach eul to ab.
4.4.1.4 Replace-concrete-desiqnators
Operation: replace-concrete-desiqnatorsfad)
where ad is a <declaration> or a <data-description>.
Step 1. For each <expression-designator)red component of ad, perform Steps 1.1 and 1.2.
step 1.1. Let e be the {expression? designated by ed.
Step 1.2. Perform create-abstract-equivalent-tree(e) to obtain an <expression>,ae and
replace ed by ae.
Step 2. For each Creference-designator)rrd component of ad, perform Steps 2.1 and 2.2.
step 2.1. Let r be the sreference) designated by rd.
Case 2.1.1. rd is an immediate component of (based>.
Perform create-reference(r,<value-reference71 to obtain a <value-
reference>,vr.
Case 2.1.2. rd is an immediate component of <base-item> or <offset>.
Perform create-reference(r,<variable-reference>) to obtain a <variable-
reference>,vr.
Step 2.2. Replace rd by vr.
96
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 115

step 3. For each {initial-designatorp,ides component of ad, perform Step 3.1.
Step 3.1. Let int be the {initial) designated by ides. Perform create-abstract-
equivalent-treetint) to obtain an <initial>,i and replace int by i.
4.4.1.5 Create-qroup
Operation: create-qroup(g1
where g is a {group*.
result: a <group>.
Step 1. Let ds be the Cdo-statement) in g. If g immediately contains a funit-list+, let
ul be that @nit-list+. Otherwise let ul be <absent+.
Case 1.1. ds immediately contains a $do-spec),dsp.
Perform create-abstract-equivalent-treecdsp) to obtain a <do-spec>,adsp.
Perform create-executable-unit-list(U) to obtain an cexecutable-unit-
list>,eul. For each <entry-point>,ep contained in eul, eul must contain a
<procedure> that contains ep. Return a <group>: <iterative-group>:
<controlled-group>: adsp eul.
Case 1.2. ds immediately contains a fwhile-optionp.wo.
Perform create-abstract-equivalent-tree(woI to obtain a <while-option>,awo.
Perform create-executable-unit-list(u1) to obtain an <executable-unit-
list>,eul. For each <entry-point>,ep contained in eul, eul must contain a
<procedure> that contains ep. Return a <group>: <iterative-group>: <while-
only-group>: awo eul.
Case 1.3. (Otherwise).
Perform create-entry-or-executable-unit-list(U) to obtain an <entry-or-
executable-unit-list>,eul. Return a <group>: <non-iterative-group>: eul.
4.4.1.6 Create-entry-or-executable-unit-list
Operation: create-entry-or-executable-unit-list(ul1
where ul is a [{unit-list)].
result: an <entry-or-executable-unit-list>.
Step 1. Let eul be an <entry-or-executable-unit-list>. If ul is not <absent+ then for
each $unit),u in ul, taken in order, perform Step 1.1.
Step 1.1. If u immediately contains an gentry-statementf,es then perform create-entry-
point(es) to obtain an <entry-point>,ep, and append an <entry-or-executable-
unit>: ep; to eul; otherwise, if u immediately contains an Cexecutable-
unit),eu then perform create-executable-unit(eu1 to obtain an <executable-
unit>,aeu and append an <entry-or-executable-unit>: aeu: to eul.
step 2. Append an <entry-or-executable-unit>: <executable-unit>: <end-statement>;; to
eul, and return eul.
Chapter 4: The Translator 97
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 116

4.4.1.7 Create-executable-unit-list
Operation: create-executable-unit-list(u1)
where ul is a [{unit-listgl.
result: an <executable-unit-list>.
Step 1. Let eul be an <executable-unit-list>. If ul is not *absent+ then for each
{unit),u in ul, taken in order, perform Step 1.1.
Step 1.1. u must not immediately contain an {entry-statement). If u immediately
contains an {executable-unit),eu, perform create-executable-unit(eu) to
obtain an <executable-unit>,aeu and append aeu to eul.
Step 2. Append an <executable-unit>: <end-statement>; to eul, and return eul.
4.4.1.8 Create-executable-unit
Operation: create-executable-unit(eu)
where eu is {executable-unit).
result: an <executable-unit>.
Step 1. If eu immediately contains an {executable-single-statement),esr let st be the
immediate component of es; otherwise let st be the immediate component of eu
that is not a {prefix-list?.
Step 2. Perform create-abstract-equivalent-tree(st) to obtain ast. Let aeu be an
<executable-unit>: ast.
Step 3. If eu immediately contains a <prefix-list),pl, perform Steps 3.1 and 3.2.
Step 3.1. Perform create-condition-prefix-list(p1) to obtain a <condition-prefix-
list>,cpl or fabsent>,cpl. If cpl is not tabsent, then attach cpl to
aeu.
Step 3.2. Perform create-statement-name-list(p1) to obtain a <statement-name-
list>,snl or fabsent>,snl. If snl is not <absent+, then attach snl to
aeu.
Step 4. Return aeu.
4.4.1.9 Create-entry-point
An <entry-point> may be the primary <entry-point> of a <procedure>, or a secondary
<entry-point> specified in the concrete {procedure+ by an sentry-statement). Each <entry-
point> has its own information for entry and return which becomes the <entry-information>
and <returns-descriptor> respectively.
Operation: create-entry-point(es)
where es is an {entry-statement) or a {procedure-statement).
result: an <entry-point>.
Step 1. Let ep be an <entry-point>: <entry-information>,ei.
Case 1.1. es is an <entry-statement).
Let pl be the {statement-name-list) of the {unit) immediately containing es.
Case 1.2. es is a <procedure-statement).
Let pl be the {prefix-list) of the Iprocedure) immediately containing es.
98
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 117

Step 2. Perform create-statement-name-list(pll to obtain a <statement-name-list>,snl.
Let asn be a copy of the <statement-name> in snl, and attach asn to ep.
step 3.. If es has a Cparameter-name-commalistt,pnl, perform create-abstract-equivalent-
tree(pn1) to obtain a <parameter-name-list>,apnl and attach apnl to ei.
Step 4. If es has a veturns-descriptor),rd, then perform step 4.1.
Step 4.1. Let d be the first $descriptiori# in rd. Perform create-data-description(d)
to obtain a <data-description>,dd. Perform replace-concrete-
designators(dd). Let rdd be a <returns-descriptor>: dd: and attach rdd to
ei.
Step 5. If es has an $options*, attach an implementation-dependent tree of type
<options> to ei.
Step 6. Return ep.
4.4.1.10 Create-statement-name-list
Operation: create-statement-name-list(p1)
where pl is a Cprefix-list? or a {statement-name-list).
result: a <statement-name-list>.
Step 1. Let snl be a <statement-name-list>. For each Cstatement-name),sn in pl, perform
Steps 1.1 and 1.2.
Step 1.1. Let id be the Cidentifiera in sn. Perform create-identifiertid) to obtain an
<identifier>,ad. Let asn be a <statement-name>: ad. For each Csigned-
integer),si in sn, taken in order, perform Step 1.1.1.
Step 1.1.1. Let asi be a <signed-integer> whose concrete-representation is the same
as that of si. Append asi to the <signed-integer-list> in asn.
Step 1.2. Append asn to snl.
Step 2. If snl has any subnodes, return snl; otherwise return <absent%.
4.4.1.11 Create-condition-prefix-list
Operation: create-condition-prefix-list(p1)
where pl is a sprefix-list.) or a {condition-prefix-commalist).
result: a <condition-prefix-list>.
step 1. Let cpl be a <condition-prefix-list>.
Step 2. For each {computational-condition*,cc in pl, perform Step 2.1.
Step 2.1. Perform create-condition(cc) to obtain a <computational-condition>,acc.
Append a <condition-prefix>: act <enabled>; to cpl.
step 3. For each *disabled-computational-conditiongrdcc, in pl, perform Step 3.1.
step 3.1. Perform create-condition(dcc1 to obtain a <computational-condition>,acc.
cpl must not contain <condition-prefix>: act <enabled>. Append a
<condition-prefix>: act <disable&; to cpl.
step 4. If cpl has any subnodes then return cpl; otherwise return <absents.
Chapter 4: The Translator 99
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 118

4.4.1.12 Create-condition
Operation: create-condition(c)
Case 1.
Case 2.
Case 3.
Case 4.
Case 5.
Case 6.
Case 7.
where c is a Ccomputational-condition), $disabled-computational-
condition), or {condition-name), or Cio-condition?.
result: a <computational-condition>, <io-condition>, or <condition-name>.
c is a {computational-conditiori?.
Return a <computational-condition>: <x-condition>; where "x" is the lowercase
equivalent of the concrete-representation of c.
c is a {disabled-computational-condition+.
Return a <computational-condition>: <x-condition>; where x is a name such that
"nox" is the lowercase equivalent of the concrete-representation of c.
c is an {io-condition?.
Return an <io-condition>: <x-condition>; where "x" is the lowercase equivalent
of the concrete-representation of c.
c is a bcondition-name) that contains AREA, ERROR, FINISH, or STORAGE.
Return a <condition-name>: <x-condition>; where "x" is the lowercase equivalent
of the concrete-representation of c.
c is a {condition-name?: Ccomputational-condition),cc.
Perform create-conditionkc) to obtain a <computational-condition>,acc, and
return a <condition-name>: act.
c - {condition-name):
(+XefZenceT,ref).
<named-io-condition*: {io-condition),ioc
Perform create-condition(ioc) to obtain an <io-condition>.aioc, and perform
create-reference(ref,<value-reference>) to obtain a <value-reference>,vr.
Return a <condition-name>: <named-io-condition>: aioc vr.
c is a Ccondition-name): {programmer-named-condition).
Let id be the {identifier) in c. Perform find-applicable-declarationcid) to
obtain a $declaration),dd. Let des be a <declaration-designator> designating the
<declaration> whose {declaration-designator) designates dd. Return a <condition-
name>: <programmer-named-condition>: aes.
100
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 119

4.4.2 CREATION OF STATEMENTS
4.4.2.1 Create-assiqnment-statement
Operation: create-assiqnment-statement(ast)
where ast is an {assignment-statement+.
result: an <assignment-statement>.
step 1.
Case 1.1. ast immediately contains $,p, BY, and NAME.
Perform create-by-name-assignment(ast) to obtain an <assignment-
statement>,aast or a <null-statement>,aast. If aast is a (null-statement>
then return aast.
Case 1.2. (Otherwise).
Let aast be an <assignment-statement>: <target-reference-list>,trl. For
each $reference),r, immediately contained in the {reference-commalist+ in
ast, taken in left-to-right order, perform create-reference(r,<target-
reference>) to obtain a <target-reference>,tr, and append tr to trl. Let e
be the {expression) immediately contained in ast. Perform create-
expression(e) to obtain an <expression>,ae, and attach ae to asst.
step 2. The <data-description> of ae must be proper for assignment to the <data-
description> of each <target-reference> in aast.
Step 3. Return aast.
4.4.2.2 Create-by-name-assiqnment
Operation: create-by-name-assiqnment(ast)
where ast is an Tassignment-statement+.
result: an <assignment-statement>
or <null-statement>.
Step 1. Perform create-by-name-parts-list(ast) to obtain a <by-name-parts-list>,bnpl or
tabsent+,bnpl. If bnpl is <absent>, return a <null-statement>.
Step 2. Let aast be an <assignment-statement>: <target-reference-list>.trl. For each
+.reference),r, immediately contained in the {reference-commalist) in ast, taken
in left-to-right order, perform Step 2.1.
Step 2.1. Perform create-reference(r,<target-reference>,bnpl) to obtain a <target-
reference>,tr. Append tr to trl.
step 3. Let e be the $expressiong immediately contained in ast. Perform create-
expression(e,bnpl) to obtain an <expression>,ae and attach ae to aast.
step 4. Each <builtin-function-reference> or <procedure-function-reference> simple
component of aast, which is not contained in a <locator-qualifier>, in a
<subscript>, or in a <builtin-function-reference> whose result has faggregate-
type+: <scalar+; (see "Attributes" for each builtin function in Section 9.4.4).
must immediately contain a <data-description> which simply contains an <item-
data-description>.
Step 5. Return aast.
Chapter 4: The Translator 101
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 120

4.4.2.3 Data-descriptions Proper for Assiqnment
The <data-description>,dds is proper for assiqnment to the <data-description>,ddt if and
only if the following conditions exist?-
(1) The iaggregate-type, of dds is promotable (see Section 7.5.3.11 to the faggregate-
type+ of ddt.
(2) Corresponding <data-type>s of dds and ddt:
(2.1) both have <computational-type>, or
(2.2) both have <locator>, or
(2.3) both have <non-computational-type>, with the immediate subnodes of the <non-
computational-type>s belonging to the same category other than <locator>.
Further, if one <data-type> has <offset> and the other has <pointer>, then the
<offset> must contain a <variable-reference>.
4.4.2.4 Create-by-name-parts-list
Operation: create-by-name-parts-list(asr)
where asr is an Cassignment-statement?
or an $argumentsp.
result: a <by-name-parts-list> or <absent>.
Step 1. Let bnpl be a <by-name-parts-list-list> with no components.
Step 2. For each {reference).lr immediately contained in the {reference-commalist> of
asr perform Step 2.1.
Step 2.1. Perform find-applicable-declaration(lr) to obtain a $declaration),cd, which
must declaration-contain STRUCTURE. Perform find-by-name-parts(cd) to
obtain a <by-name-parts-list>,bnp and append bnp to bnpl.
Step 3. For each $referenceq,r contained in the Cexpressiont immediate component of asr,
but not contained in a {locator-qualifier? or {arguments*, perform Steps 3.1 and
3.2.
step 3.1. Perform find-applicable-declaration(r) to obtain a $declaration),cd.
step 3.2. If cd declaration-contains STRUCTURE then perform find-by-name-partsfcd) to
obtain a <by-name-parts-list>.bnp and append bnp to bnpl.
Step 4. Let rbnpl be a <by-name-parts-list> consisting of those <by-name-parts> which
are common to every <by-name-parts-list> of bnpl.
Step 5. If rbnpl is not empty then return rbnpl; otherwise return <absent%.
102
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 121

4.4.2.5 Find-by-name-parts
Operation: find-by-name-parts(d)
where d is a Cdeclarationp.
result: a <by-name-parts-list>.
Step 1. Let dl be the Cdeclaration-commalist) node that immediately contains d. Let
eIj1 be the j'th immediate component of dl that is not a C,). Let n be the
number of such components and let k be such that eCk1 is d. Let Id be the
numeric value of the {level) of d. Let i be k+l.
Step 2. Let bnpl be a <by-name-parts-list> with no components.
step 3. While isn perform Steps 3.1 through 3.4.
step 3.1. If eCi1 does not declaration-contain MEMBER then go to Step 4.
step 3.2. Let le be the numeric value of the {level) of eCi1. If lelld, go to Step 4.
step 3.3. If le = Id+1 then perform Steps 3.3.1 and 3.3.2.
Step 3.3.1. Let cid be the {identifier) of eliI. Perform create-identifiertcid) to
obtain an <identifier>,id.
Step 3.3.2.
Case 3.3.2.1. efil declaration-contains STRUCTURE.
Perform find-by-name-parts(e0 to obtain a <by-name-parts-
list>,rbnp. Attach a copy of id to each <identifier-list>
component of rbnp as the initial element. Append a copy of each
immediate component of rbnp to bnpl.
case 3.3.2.2. (Otherwise).
Append a <by-name-parts> containing id to bnpl.
step 3.4. Let i = i+l.
step 4. Return bnpl.
4.4.2.6 Create-allocation
Operation: create-allocation
where al is an {allocation).
result: an <allocation>.
Step 1. Let id be the fidentifier) immediately contained in al. Perform find-
applicable-declarationtid) to obtain a $declaration*,cdcl. cdcl must not
declaration-contain MEMBER.
Step 2. Let adcl be the <declaration> whose {declaration-designator) designates cdcl.
If adcl contains <controlled>, then al must not contain a Cset-option> or an
{in-option). If adcl contains a <based>,b and al does not contain a {set-
option), then b must immediately contain a <value-reference> that immediately
contains a <variable-reference>.
Step 3. Let des be a <declaration-designator> designating adcl. Let aal be an
<allocation>: des. If al contains a Cset-option$,sp, then perform create-
abstract-equivalent-tree(sp) to obtain a <set-option>,asp, and attach asp to
aal. If al contains an fin-option),io, then perform create-abstract-equivalent-
tree(io1 to obtain an <in-option>,aio, and attach aio to aal.
Step 4,. Return aal.
Chapter 4: The Translator 103
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 122

4.4.2.7 Create-format-statement
Operation: create-format-statement(fs)
where fs is a (format-statement).
result: a <format-statement>.
Step 1. Let fsl be the {format-specification-commalist+ component of fs. Perform
create-abstract-equivalent-tree(fs1) to obtain a <format-specification-
list>,afsl.
Step 2. The {unit) that immediately contains fs must immediately contain a Cprefix-
list*,pl. Perform create-statement-name-list(p1) to obtain a <statement-name-
list>,snl, which must not be <absent+. Let afs be a <format-statement>: snl
afsl.
Step 3. Perform create-condition-prefix-list(pl) to obtain a Gcondition-prefix-
list>,cpl. or fabsent9,cpl. If cpl is not <absent+, attach cpl to afs.
Step 4. Return afs.
4.4.2.8 Create-format-iteration
Operation: create-format-iterationcfil
where fi is a <format-iteration).
result: a <format-iteration>.
Step 1. Let afi be a <format-iteration>.
Step 2. Let ff be the {format-iteration-factor? immediately contained in fi. If ff
immediately contains an {expression+, let e be that $expression); otherwise let
e be an {expression) to which the immediately contained {integer) of ff has been
attached. Perform create-expression(e) to obtain an <expression>,ae and attach
a <format-iteration-factor>: ae; to afi.
Step 3.
Case 3.1. fi immediately contains a Cformat-specification-commalist),fsc.
Perform create-abstract-equivalent-treetfsc) to obtain a <format-
specification-list>,fl, and attach fl to afi.
Case 3.2. fi immediately contains a {format-item),ft.
Perform create-abstract-equivalent-tree(ft) to obtain a <format-item>,ft and
attach a <format-specification-list>: <format-specification>: ft;; to afi.
Step 4. Return afi.
104
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 123

4.4.2.9 Create-freeinq
Operation: create-freeinqcfr)
step 1.
Step 2.
Step 3.
where fr is a Cfreeing).
result: a <freeing>.
Let id be the {identifier) immediately contained in fr. Perform find-
applicable-declaration(id) to obtain a $declaratio@,cdcl. cdcl must not
declaration-contain MEMBER.
Let adcl be the <declaration> whose {declaration-designator) designates cdcl.
If adcl contains <controlled>, then fr must not contain an {in-option) or a
Qocator-qualifier).
Let des be a <declaration-designator> designating adcl. Let afr be a <freeing>:
des. If fr immediately contains a {locator-qualifier) with +Xeference),r,
perform create-referencetr,<value-reference>) to obtain a <value-reference>,vr
and attach a <locator-qualifier>: vr; to afr. If afr contains an fin-
option),io, perform create-abstract-equivalent-tree(io) to obtain an <in-
option>.aio. and attach aio to afr. Return afr.
4.4.2.10 Create-if-statement
Operation: create-if-statementtifs)
where ifs is an {if-statement).
result: an <if-statement>.
Step 1. ifs immediately contains an {if-clause) which contains an $expressionp,e.
Perform create-expression(e) to obtain an <expression>,ae.
Step 2.
Case 2.1. ifs immediately contains an {executable-unit),eu, but not an ELSE.
Perform create-executable-unit(eu1 to obtain an <executable-unit>,aeu.
Return an
<if-statement>:
<test>: ae;
<then-unit>: aeu.
Case 2.2. ifs immediately contains a sbalanced-unit),bu, an ELSE, and an Cexecutable-
unit).eu.
Perform create-balanced-unit(bu) to obtain an <executable-unit>,eul.
Perform create-executable-unit(eu) to obtain an <executable-unit>,eu2.
Return an
<if-statement>:
<test>: ae;
<then-unit>: eul;
<else-unit>: eu2.
chapier 4: The Translator 105
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 124

4.4.2.11 Create-balanced-unit
Operation: create-balanced-unit(bu)
where bu is a {balanced-unit),.
result: an <executable-unit>.
step 1.
Case 1.1. bu immediately contains an sexecutable-single-statemenyless.
Let s be the immediate component of ess. Perform create-abstract-
equivalent-tree(s) to obtain stat.
Case 1.2. bu immediately contains a {group), Cbegin-block), or {on-statement).
Let s be that {group), {begin-block), or (on-statement). Perform create-
abstract-equivalent-tree(s) to obtain stat.
Case 1.3. (Otherwise).
Let e be the {expression) immediately contained in the $if-clause)
immediately contained in bu. Perform create-expression(e) to obtain an
<expression>,ae. Let bul and bu2 be, in order, the sbalanced-unit@
immediately contained in bu. Perform create-balanced-unit(bu1) to obtain
eul and create-balanced-unit(bu21 to obtain eu2. Let stat be an
<if-statement>:
<test>: ae;
<then-unit>: eul;
<else-unit>: eu2.
Step 2. Let eu be an <executable-unit>: stat. If bu immediately contains a {prefix-
list),pl, perform Steps 2.1 and 2.2.
Step 2.1. Perform create-statement-name-list(p1) to obtain a <statement-name-list>,snl
or fabsent>,snl. If snl is not <absent,, attach snl to eu.
Step 2.2. Perform create-condition-prefix-list(pl1 to obtain a <condition-prefix-
list>,cpl or fabsent+,cpl. If cpl is not <absent+, attach cpl to eu.
Step 3. Return eu.
4.4.2.12 Create-locate-statement
Operation: create-locate-statement(ls)
where Is is a <locate-statement).
result: a <locate-statement>.
Step 1. Let id be the $identifierg immediately contained in 1s. Perform find-
applicable-declaration(id) to obtain a $declaration+,cdcl. cdcl must not
declaration-contain MEMBER.
Step 2. Let des be a <declaration-designator> designating the <declaration>,ad, whose
<declaration-designator) designates cdcl. ad must contain <based>. Let als be
a <locate-statement>: des. For each immediate subnode, x, of 1s other than id
or LOCATE, perform create-abstract-equivalent-tree(x) to obtain a <file-
option>,y, <pointer-set-option>.y or a <keyfrom-option>,y, and attach y to als.
Step 3. Return als.
106
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 125

4.4.2.13 Create-on-statement
Operation: create-on-statement(os)
where OS is an Con-statementg.
step 1.
result: an <on-statement>.
Case 1.1. OS immediately contains SYSTEM.
Let p be a <system-action>.
Case 1.2. OS immediate contains an {on-unit).ou.
Step 1.2.1.
Case 1.2-L-l. ou immediately contains an {executable-single-statement),ess.
Let s be the immediate component of ess.
Case 1.2.1.2. ou immediately contains a {begin-block?.
Let s be that Cbegin-block?.
Step 1.2.2. Perform create-abstract-equivalent-tree(s) to obtain sa. Let ep be an
<entry-point> whose only subnode is <entry-information>. Let eul be an
<entry-or-executable-unit-list>:
<entry-or-executable-unit>: ep;
<entry-or-executable-unit>:
<executable-unit>,eu: sa;;
<entry-or-executable-unit>:
<executable-unit>:
<end-statement>.
Step 1.2.3. If ou immediately contains a +condition-prefix-commalist*,cpl, perform
create-condition-prefix-list(cp1) to obtain acpl, and attach acpl to eu.
Step 1.2.4. Let p be an <on-unit>: <procedure>: eul.
step 2. Let aos be an <on-statement>: p. If OS immediately contains SNAP attach <->
to aos. For each *condition-nameg,cn in the {condition-name-commalist) in OS,
perform create-conditionfcn) to obtain a <condition-name>,acn, and append acn to
the <condition-name-list> in aos. Return aos.
4.4.3 CREATE-DECLARATION
Operation: create-declaration(d)
where d is a (declaration).
result: a <declaration>.
step 1. Let cid be the Cidentifier? of d. Perform create-identifiertcid) to obtain an
<identifier>,id.
step 2.
Case 2.1. d contains INTERNAL.
Let SC be <scope>: <internal>.
Case 2,.2. d contains EXTERNAL.
Let SC be <scope>: <external>.
Chapter 4: The Translator 107
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 126

step 3.
Case 3.1. d contains VARIABLE.
Perform create-variable(d) to obtain a <variable>,v. Let dt be
<declaration-type>: v.
Case 3.2. d contains CONSTANT.
Perform create-named-constant(d) to obtain a <named-constant>,nc. Let dt be
<declaration-type>: nc.
Case 3.3. d contains BUILTIN.
Let dt be a <declaration-type>: <builtin>.
Case 3.4. d contains CONDITION.
Let dt be a <declaration-type>: <condition>.
Step 4. Let dd be a {declaration-designator) that designates d.
step 5. Return <declaration>: id SC dt dd.
4.4.3.1 Create-named-constant
Operation: create-named-constant(d)
where d is a {declaration).
result: a <named-constant>.
step 1. Let nc be a <named-constant>.
Step 2. If d declaration-contains a {dimension-attribute+,da then perform create-bound-
pair-list(da) to obtain a <bound-pair-list>,bpl and attach bpl to -2.
Step 3.
Case 3.1.
Case 3.2.
Case 3.3.
d declaration-contains ENTRY.
Perform create-entry(d) to obtain an <entry>,ae. Attach ae to nc.
d declaration-contains FILE.
Attach Cm> to nc. Let fd be a <file-description>. For each {attribute)
component of d which declaration-contains STREAM, RECORD, INPUT, OUTPUT,
UPDATE, SEQUENTIAL, DIRECT, PRINT, KEYED, or ENVIRONMENT attach <stream>,
<record>, <m>, <output>, <update>, <sequential>, <direct>, <print>,
<keyed>, or <environment>, respectively, to fd. Attach fd to nc. If
<environment> was attached then perform some implementation-defined action.
d declaration-contains FORMAT or LABEL.
Attach <format> or <label> respectively, to nc.
Step 4. Return nc.
108
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 127

4.4.3.2 Create-variable
For each distinct variable a <declaration> which contains <variable> is constructed and
completed according to the declared attributes of the item. A <variable> may be referred
to by a <value-reference>, a <target-reference>, or a <subroutine-reference> (see Section
4.4.5).
Operation: create-variable(d)
where d is a {declaration*.
result: a <variable>.
Step 1.
Case 1.1. d contains AUTOMATIC.
Let st be a <storage-type>: <storage-class>: <automatic>.
Case L-2. d contains CONTROLLED.
Let st be a <storage-type>: <storage-class>: <controlled>.
Case 1.3. d contains STATIC.
Let st be a <storage-type>: <storage-class>: <static>.
Case 1.4. d contains an Cattributep,atr that immediately contains BASED.
Let st be a <storage-type>: <storage-class>: <based>. If atr also simply
contains a $reference),r then attach a Creference-designator>: designator of
r; to st.
Note: The translation of r will be completed after the processing of all {declare-
statementts.
Case 1.5. d contains DEFINED ($referenceg,r) or DEFINED $reference),r.
Let st be
<storage-type>:
<defined>:
<base-item>:
qreference-designator?:
designator of r.
If d contains the form $attributeT,atr: POSITION; then atr must have an
Cexpression).
If d contains $attributeg,atr which immediately contains POSITION
($expressionq,e) then attach a <position>: Cexpression-designator?:
designator of e.
Note: The translation of e will be completed after the processing of all {declare-
statement)s.
Case 1.6. d contains Cattribute): PARAMETER.
Let st be a <storage-type>: <parameter>.
Step 2. Perform create-data-description(d) to obtain a <data-description>,dd.
step 3. Return a <variable>: st dd.
Chapter 4: The Translator 109
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 128

4.4.3.3 Create-bound-pair-list
Operation: create-bound-pair-list(da)
where da is a Cdimension-attribute).
result: a <bound-pair-list>.
Step 1. Let bpl be the {bound-pair-commalist$ component of da.
Step 2. For each {bound-pair),bp of bpl such that bp contains an <upper-bound) and does
not contain a <lower-bound2. attach a Qower-bound): un; to bp, where un is a
copy of an sextent-expression2 whose concrete representation is the character
Cl? -
Step 3. Let abpl be a <bound-pair-list>. For each $bound-pair),bp component of bpl in
left-to-right order perform Step 3.1.
Step 3.1.
Case 3.1.1. bp contains l .
Append a <bound-pair>: <asterisk>: to abpl.
Case 3.1.2. (Otherwise).
Step 3.1.2.1. Let abp be a
<bound-pair>:
<lower-bound>,alb
<upper-bound>,aup.
Attach to alb an {expression-designator) designating the
Cexpression? simply contained in the Qower-bound> component of
bp- Attach to aup an Fexpression-designator) designating the
EFession) simply contained in the {upper-bound) component of
.
step 3.1.2.2. If the slower-bound) component of bp contains a {refer-option),ro
then perform create-refer-option(ro1 to obtain a <refer-
option>,aro and attach aro to alb.
Step 3.1.2.3. If the {upper-bound) component of bp contains a {refer-option),ro
perform create-refer-option to obtain a <refer-option>,aro
and attach
Step 3.1.2.4. Append abp
Step 4. Return abpl.
4.4.3.4 Create-data-description
aro to aup.
to abpl.
The <data-description> component of a <declaration> specifies the aggregate properties Of
a variable or the descriptor of a variable , and also the data properties associated with
elements of the variable. This distinction between structures, arrays, and scalars is
made at a high level, and the individual element data properties are attached to each
scalar.
Operation: create-data-description(d)
where d is a {declaratioe, a <description), or a Cgeneric-description)
whose subtree would be a valid subtree of a {description).
result: a <data-description>.
Step 1. Let dd be a <data-description>.
Step 2. If d declaration-contains a (dimension-attribute)rba then perform Steps 2.1 and
2.2.
110
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 129

Step 2.1. Perform create-bound-pair-list(da) to obtain a <bound-pair-list>,abpl.
Step 2.2. Attach <dimensioned-data-description>: <element-data-description> abpl; to
dd.
step 3. If d declaration-contains STRUCTURE then perform Steps 3.1 through 3.4.
Step 3.1. Let dl be the node that immediately contains d. Let eijl be the j'th
immediate component of dl that is not a C.). Let n be the number of such
components and let k be such that eCkl is identical to d. Let i be k+l.
Let Id be the numeric value of the $levelp of d.
Step 3.2. Let id1 be an <identifier-list> and let mdl be a <member-description-list>.
Step 3.3. While iln perform Steps 3.3.1 through 3.3.4.
Step 3.3.1. If efil does not declaration-contain MEMBER then go to Step 3.4.
Step 3.3.2. Let le be the numeric value of the {level? of e[il. If lelld then go to
Step 3.4.
Step 3.3.3. If le=ld+l then perform Steps 3.3.3.1 through 3.3.3.3.
step 3.3.3.1. Perform create-data-description(e[i3) to obtain a <data-
description>,add.
step 3.3.3.2. Append a <member-description>: add; to mdl.
step 3.3.3.3. If d is a $declaration) then let cid be the {identifier) of e[il,
perform create-identifierfcid) to obtain an <identifier>,id, and
append id to idl.
Step 3.3.4. Let i be i+l.
Step 3.4. If id1 has no components then let sdd be a <structure-data-description>:
mdl; otherwise let sdd be a <structure-data-description>: id1 mdl. Attach
sdd to dd.
Step 4. If d does not declaration-contain STRUCTURE then perform Steps 4.1 through 4.4.
Step 4.1. One of the following cases must apply:
Case 4.1.1. d declaration-contains ALIGNED.
Let al be <alignment>: <aliqned>.
Case 4.1.2. d declaration-contains UNALIGNED.
Let al be <alignment>: <unaliqned>.
Step 4.2. Perform create-data-type(d) to obtain a <data-type>,dt. Let idd be an
<item-data-description>: al dt.
Step 4.3. If d declaration-contains finitial),int then let intd be an linitial-
designator) that designates int and attach intd to idd.
Step 4.4. Attach idd to dd.
Step 5. Return dd.
Chapter 4: The Translator 111
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 130

4.4.3.5 Create-data-tvpe
Operation: create-data-type(d)
where d is a Cdeclarationq, a (description), or a {generic-description) that
restricted
;Sdescription)s.
to those forms that are equivalent to
result: a <data-type>.
One and only one of the following cases must apply:
case 1. d declaration-contains AREA($area-size),asz).
Step 1.1. Let ar be an <area>. If asz immediately contains an $*) then let ed be an
<asterisk>; otherwise let ed be an sexpression-designator2 that designates
the <expression) simple component of asz. Attach ed to ar.
Step 1.2. If asz contains a $refer-option),ro, perform create-refer-option(r0) to
obtain a <refer-option7,aro and attach aro to ar.
Step 1.3. Return <data-type7: <non-computational-type>: ar.
Case 2.
Case 3.
Case 4.
Case 5.
Case 6.
Case 7.
d declaration-contains ENTRY.
Perform create-entry(d) to obtain an <entry>,e and return a <data-type>: <non-
computational-type>: e.
d declaration-contains FILE.
Return a <data-type>: <non-computational-type>: <file>.
d declaration-contains FORMAT or LABEL.
Attach <format7 or <label7 respectively to <data-type7,dt: <non-computational-
type>. If d declaration-contains LOCAL then attach <local7 to dt. Return dt.
d declaration-contains POINTER or OFFSETI($reference),r)1.
Let dt be a <data-type>: <non-computational-type,: <locator7,loc. Attach
<pointer7 or <offset>,ofs, respectively to lot. If r exists then attach a
(reference-designator) that designates r to ofs.
d declaration-contains PICTURE $picturs:P,pa.
Perform create-picture(pa) to obtain a <pictured>,p and return a <data-type>:
<computational-type>: p.
d declaration-contains a {data-attribute),sa which declaration-contains
CHARACTER or BIT.
step 7.1. If sa contains CHARACTER then let St be a <string-type>: <character>.
Otherwise, let st be a <string-type,: <m7.
Step 7.2. Let ml be the waximum-length) component of sa. Let am1 be a <maximum-
length>. If ml immediately contains an $+) then let ed be an <asterisk>;
otherwise let ed be an $expression-designator) that designates the
$expression) simple component of ml. Attach ed to aml.
Step 7.3. If ml contains a {refer-option),ro then perform create-refer-option to
obtain a <refer-option7,aro and attach aro to aml.
Step 7.4. If d declaration-contains VARYING let v be <varvinq>; otherwise let v be
<nonvarvinq>.
Step 7.5. Return a <data-type>: <computational-type>: <string>: st am1 v.
112
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 131

Case 8. (Otherwise).
Step 8.1. Let dt be a
<data-type>:
<computational-type>:
<arithmetic>:
<mode>, m
<base>,b
<scale>,s
<precision>,p.
Step 8.2. If d declaration-contains REAL then attach <real> to m; otherwise, attach
<complex>.
Step 8.3. If d declaration-contains BINARY then attach <binary> to b; otherwise,
attach <decimal>.
Step 8.4. If d declaration-contains FIXED then attach <fixed> to s; otherwise, attach
<float>.
Step 8.5. Let cp be the Cprecisiona declaration-component of d. Perform create-
abstract-equivalent-tree(cp) to obtain a <precision>,ap. The <number-of-
digits> in ap must not be greater than the maximum <number-of-digits>
allowed for the <base> and <scale> of dt. Replace p by ap.
Step 8.6. Return dt.
4.4.3.6 Create-entry
Operation: create-entry(d)
where d is a {declaration), a $descriptionP, or a {generic-description) that
declaration-contains an ENTRY [([{description-commalist),dlo3)1.
result: an <entry>.
Step 1. Let ent be an <entry> with no subnodes.
Step 2. If dlo exists, then for each $description),pd in dlo that does not declaration-
contain MEMBER, perform Steps 2.1 and 2.2.
Step 2.1. Perform create-data-description(pd) to obtain a <data-descriptioti,dd.
Step 2.2. Append a <parameter-descriptor>: dd; to the <parameter-descriptor-list> in
ent.
Step 3. If d declaration-contains RETURNS (*description-commalist),dc), perform Steps
3.1 through 3.3.
Step 3.1. Let rd be the {description) immediate component of dc that does not
declaration-contain MEMBER. There must be exactly one such (description).
Step 3.2. Perform create-data-descriptiontrd) to obtain a <data-description>,dd.
Step 3.3. Attach a <returns-descriptor>: dd; to ent.
Step 4. If d contains OPTIONS, attach <options> with some implementation-defined
subnodes, to ent.
Step 5. Return ent.
Chapter 4: The Translator 113
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 132

4.4.3.7 Create-refer-option
Operation: create-refer-option(cr0)
where cro is a srefer-option).
result: a <refer-option>.
step 1. Let ur be the {unsubscripted-reference? of cro. Perform find-applicable-
declaration(ur3 to obtain d.
step 2. Perform find-fully-qualified-name(d) to obtain an Qdentifier-list),idl-
step 3. Perform create-abstract-equivalent-treetidl) to obtain an <identifier-
list>,aidl.
step 4. Return a <refer-option>: aidl.
4.4.3.8 Create-identifier
Operation: create-identifiertid)
where id is an fidentifier).
result: an <identifier>.
step 1. Return an <identifier> whose concrete-representation is the same as that of id.
4.4.3.9 Create-initial-element
Operation: create-initial-elementtine)
where ine is an Cinitial-element).
result: an <initial-element>.
step 1. Let aine be an <initial-element>.
Case 1.1. ine immediately contains * as its only component.
Attach an <asterisk> to aine.
Case 1.2. ine immediately contains a {parenthesized-expression+,cpe.
Perform create-abstract-equivalent-tree(cpe) to obtain a <parenthesized-
expression>,pe and attach pe to aine.
Case 1.3. ine immediately contains an {initial-constant-0ne)rico.
Let e be the {expression) whose concrete-representation is the same as the
concrete-representation of ice. Perform create-expression(e) to obtain an
<expression>.ae, and attach a <parenthesized-expression>: ae; to aine.
Case 1.4. ine immediately contains an {iteration-factor),itf.
Perform create-abstract-equivalent-treetitf) to obtain aif and attach aif to
aine.
Case 1.4.1. ine immediately contains an l .
Attach an <initial-element-list>: <initial-element>: <asterisk>;; to
aine.
114
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 133

Case 1.4.2. ine immediately contains an {initial-constant-twoo),ict-
Let e be the $expression) whose concrete-representation is the same as
the concrete-representation of ict. Perform create-expression(e) to
obtain an <expression>,ae. Attach an <initial-element-list>: <initial-
element>: <parenthesized-expression>: ae;;; to aine.
Case 1.4.3. ine immediately contains an $initial-element-commalist),iec.
Perform create-abstract-equivalent-tree(iec1 to obtain aiec, and attach
aiec to aine.
step 2. Return sine.
4.4.4 CREATE-EXPRESSION
Operation: create-exvression(e,bnpl)
where e is an Cexpression), {expression-seven), {expression-six),
{expression-five), (expression-four*, {expression-three),
{expression-two), {expression-one*, Cprimitive-expression*,
{prefix-expression), or {parenthesized-expression),
bnpl is a [<by-name-parts-list>].
result: an <expression>.
Case 1. e is an Cexpression), {expression-seven?, {expression-six), *expression-five*,
$expression-four), {expression-three), or an <expression-two) and e has only one
component, ec.
Perform create-expression(ec,bnpl) to obtain an <expression>,aec. Return sec.
Case 2. e is an {expression), {expression-seven), *expression-six), {expression-five),
{expression-four), *expression-three?, or an {expression-one) and e has three
components, el, op, and e2.
Step 2.1. Perform create-expression(el,bnpl) to obtain an <expression>,ael. Perform
create-expression(e2,bnpl) to obtain an <expression>,ae2.
step 2.2.
step 2.3.
If op is or has I, &, >, >=, =, <=, <, v>, q=, Y< <it!18 +r -~ *, /, *+ then
let aop be <or>, <and>, <s>, <e>, <a>, <ne>,
<m>, <g><subtract>,
<le>,
(multiply>, <dividG,
<le>, <ge> I
or?pAwer>respeZively.
Let ddl and dd2 be the <data-description% immediately contained in ael and
ae2, respectively.
The associated <aggregate-type+s of ddl and dd2 must be compatible.
Individual <data-type>s in ddl and dd2 and corresponding <data-type>s in ddl
and dd2 must satisfy any constraints specified in the "Constraints'
paragraphs of the section of chapter 9 for the tinfix-operator>,aop.
Let dd be a <data-description> whose associated *aggregate-type+ is the
common *aggregate-type, of ddl and dd2 and whose <data-type>s are defined as
"scalar-result-types" in the "Attributes" paragraphs of the section of
Chapter 9 for the <infix-operator>,aop in terms of the corresponding <data-
type>s in ddl and dd2.
Step 2.4. Return an
<expression>:
<infix-expression>:
ael
<infix-operator>:
sop;
ae2
dd;
dd.
Chapter 4: The Translator 115
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 134

Case 3. e is a <primitive-expression).
Let ec be the immediate component of e.
Case 3.1. ec is a sreferenceq.
Perform create-reference(ec,<value-reference>,bnpU to obtain a Ofalue-
reference>,vr. Let dd be the <data-description> immediately contained in vr.
Return an <expression>: vr dd.
Case 3.2. ec is a Cconstant).
Perform create-constant(ec1 to obtain a <constant>,c. Let dt be the <data-
type> immediately contained in c. Return an
<expression>:
C
<data-description>:
<item-data-description>: dt.
Case 3.3. ec is an <isub+.
ec must be contained in an {attribute? that immediately contains DEFINED.
Let i be an <integer> whose concrete-representation is the same as that of
the iinteger) in ec. Let dt be a <data-type> that is an integer-type. Return
an
<expression>:
<isub>: i;
<data-description>:
<item-data-description>: dt.
Case 4. e is a $prefix-expression): op el.
Step 4.1. Perform create-expression(el,bnpl) to obtain ael.
Step 4.2. If op has q, +, - then let aop be <E>, <>, or Uninus> respectively.
Step 4.3. Let ddl be the <data-description> immediately contained in ael.
The <data-type>s in ddl must satisfy any constraints in the "Constraints'
paragraphs of the section of Chapter 9 for the <prefix-operator>,aop.
Let dd be a <data-description> whose associated faggregate-type, is the same
as that of ddl and whose <data-type>s are defined as "scalar-result-types'
in the -Attributes* paragraphs of the section of Chapter 9 for the <prefix-
operator>,aop in terms of the corresponding <data-type> in ddl.
Step 4.4. Return an
<expression>:
<prefix-expression>:
<prefix-operator>: aop;
ael
dd;
dd.
Case 5. e is a {parenthesized-expression): ($expressioq,el).
Step 5.1. Perform create-expression(el,bnpl) to obtain an <expression>,ae. Let dd be
the <data-description> immediate component of ae.
Step 5.2. Return an
<expression>-
<parenthesized-expression>:
dd.
116
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 135

4.4.5 CREATE-REFERENCE
Operation: create-reference(cr,targ,bnpl)
where cr is a <reference) or an Cunsubscripted-reference?,
targ is a <variable-reference>, <value-reference>, <subroutine-
reference>, or <target-reference>,
bnpl is a [<by-name-parts-list>].
result: a tree of the same type as targ.
Step 1. If cr immediately contains an {arguments-list*,ca, let al be a copy of ca.
Otherwise let al be an *arguments-list). Perform find-applicable-declaration0
to obtain a $declaration),cd.
Case 1.1. cd contains a {generic-attribute).
Let dcl be <absent+.
Case 1.2. cd declaration-contains MEMBER.
Let red be the rightmost preceding +.declaration# that does not declaration-
contain MEMBER. Let dcl be the <declaration> whose Cdeclaration-designator)
designates red.
Case 1.3. (Otherwise).
Let dcl be the <declaration7 whose Cdeclaration-designator) designates cd.
It must not contain <condition>.
step 2.
Case 2.1. dcl
Step 2.1.1.
Step 2.1.2.
is a <declaration> that has <variable>.
Let des be a <declaration-designator> designating dcl. Let dd be a copy
of the <data-description> in dcl. Let ref be a <variable-reference>:
des dd-
Perform find-fully-qualified-name(cd) to obtain an {identifier-list),il.
Perform create-abstract-equivalent-treetill to obtain an <identifier-
list>,idl. Delete the first <identifier> in idl. If id1 still contains
any <identifier>s. attach id1 to ref. and for each <identifier>,id, in
idl, taken in order, perform Steps 2.X.2.1 through 2.1.2.3.
Step 2.1.2.1. Let dd be the <data-description> imoediately contained in ref.
It will have a <structure-data-description7,sdd. If dd has a
<dimensioned-data-description>, let bpl be the <bound-pair-list>
in dd; otherwise let bpl be a <bound-pair-list>. Let il be the
<identifier-list> in sdd, and let mdl be the <member-description-
list7 in sdd. Let i be the integer such that the i*th
<identifier> in il equals id, and then let mdd be the <data-
description> immediately contained in the i'th Unember-
description7 of mdl.
step 2.1.2.2. If mdd has a <dimensioned-data-description>, append copies of the
<bound-pair>s in mdd to bpl, and let tdd be the <item-data-
description> or <structure-data-description> immediately
contained in the <element-data-description7 of mdd. Otherwise,
let tdd be the <item-data-description7 or <structure-data-
description> immediately contained in mdd.
step 2.1.2.3. If bpl contains <bound-pair>s, replace dd by a
<data-description>:
<dimensioned-data-description>:
<element-data-description>:
tdd:
bpl-
Otherwise replace dd by a <data-description>: tdd.
Chapter 4: The Translator 117
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 136

Step 2.1.3. If cr is a {reference), then let br be the <basic-reference) immediately
contained' in cr, and perform collect-subscripts(br1 to obtain a
{subscript-commalist*,sl. otherwise let sl be a (subscript-commalist).
Case 2.1.3.1. The <data-description> immediately contained in ref does not have
a <dimensioned-data-description>.
In this case sl must not contain any $subscript)s.
Case 2.1.3.2. The <data-description>,dd, immediate component of ref has a
<dimensioned-data-description>.
Perform apply-subscripts(cr,ref,sl,al).
Case 2.2. dcl is a <declaration> that has <named-constant>.
Perform create-named-constant-reference(dc1) to obtain a Ulamed-constant-
reference>,ref. If the <data-description> immediately contained in ref has a
<dimensioned-data-description>, let sl be a $subscript-commalist)r and
perform apply-subscripts(cr,ref,sl,al).
Case 2.3. dcl is a <declaration> that has <builtin>.
step 2.3.1.
Case 2.3.1.1. al has an farguments).
In this case, al must have only one (arguments). Let ar be the
{arguments) in ca. Perform create-argument-listtar) to obtain
args. Delete the <arguments) from al.
Case 2.3.1.2. al does not have an {arguments).
Let args be <absent+.
step 2.3.2.
Case 2.3.2.1. targ is a <value-reference>.
Perform create-builtin-function-reference(dc1.arg.s) to obtain a
<builtin-function-reference>,ref.
Case 2.3.2.2. targ is a <target-reference>.
Perform create-pseudo-variable-reference(dcl,args) to obtain a
<pseudo-variable-reference>,ref.
Case 2.3.2.3. targ is a <variable-reference> or a <subroutine-reference>.
This case must not occur.
Case 2.4. cd has a {generic-attribute).ga.
Step 2.4.1.
Case 2.4.1.1. al has an {arguments).
Let ar be the first <arguments) in ca, and perform create-
argument-listtar) to obtain args.
Case 2.4.1.2. al does not have an Targuments).
In this case targ must be a <subroutine-reference>. Let args be
*absent+.
Step 2.4.2. Perform select-generic-alternative(garargs) to obtain a <value-
reference>,vr. Let ref be the first immediate component of vr.
Step 3.
Case 3.1. cr immediately contains a {locator-qualifier),lq.
118
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 137

Let r be the <reference) immediately contained in lq. Perform create-
reference(r,<value-reference>) to obtain a <value-reference>,vr. All the
following conditions must hold:
(1) ref must be a <variable-reference>;
(2) the <declaration7 designated by the <declaration-designator7 immediately
contained in ref must have <based>;
(3) the <data-description7 immediately contained in vr must immediately
contain an <item-data-description> whose <data-type7 must have
<locator>, and if the <data-type7 has an <offset>,os, then OS must have
a <variable-reference> or a *reference-designator).
Attach a <locator-qualifier>: vr; to ref.
Case 3.2. cr does not immediately contain a (locator-qualifier) and dcl is a
<declaration> that has <variable>.
If dcl contains a <based>,b, then b must inunediately contain a <value-
reference> or a {reference-designator).
Case 3.3. (Otherwise)-
No action.
step 4. If al contains an {arguments), perform Steps 4.1 to 4.4.
Step 4.1. ref must not be a <subroutine-reference>. Perform create-value-
referencefref) to obtain evr. The <data-description> immediate component of
evr must immediately contain an <item-data-description7 whose <data-type>
must have <entry>.
step 4.2. Let ar be the first <arguments) in al. Perform create-argument-listcar) to
obtain args. Delete ar from al.
Step 4.3. Perform create-entry-reference(evr,args) to obtain ref.
Step 4.4. Go to Step 4.
Step 5. If bnpl in not fabsent, and ref is a <variable-reference> then perform apply-by-
name-parts(ref,bnpl) to obtain a <variable-reference>,ref. If bnpl is not
*absent+ and ref is not a <variable-reference>, then the <data-description7 of
ref must immediately contain <item-data-description>.
Step 6.
Case 6.1. targ is a <variable-reference>.
In this case ref must be a <variable-reference>. Return ref.
Case 6.2. targ is a <value-reference>.
ref must not be a <subroutine-reference>. Perform create-value-
referencecref) to obtain a <value-reference7,vr. Return vr.
Case 6.3. targ is a <subroutine-reference>.
If ref is a <subroutine-reference7, return ref. Otherwise perform create-
value-referencetrefl to obtain a <value-reference>,vr whose immediately
contained <data-description> must immediately contain an <item-data-
description7 whose <data-type7 must have <entry>. Perform create-entry-
referencefref) to obtain sr, which must be a <subroutine-reference>. Return
sr.
Case 6.4. targ is a <target-reference>.
In this case ref must be a <variable-reference> or <pseudo-variable-
reference>. Let dd be a copy of the <data-description> immediately contained
in ref. If ref is a <variable-reference>, perform trim-dd(dd). Return a
<target-reference>: ref dd.
Chapter 4: The Translator 119
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 138

4.4.5.1 Collect-subscripts
Operation: collect-subscripts
where br is a Cbasic-reference*.
result: a {subscript-commalist+ which may have no components.
Case I. br has a {structure-qualification).sq.
Let br2 be the {basic-reference) immediately contained in sq. Perform collect-
subscripts(br2) to obtain a {subscript-commalistt,sl. If sq has an
{arguments), args, then args must have a {subscript-commalist)rs12, append the
{subscript+ in ~12 to sl, appending $,) as required. Finally, return sl.
Case 2. br does not have a {structure-qualificationp.
Return a {subscript-commalist).
4.4.5.2 Apply-by-name-parts
Operation: apply-by-name-parts(vro,bnp)
where vro is a <variable-reference>,
bnp is a <by-name-parts-list>.
result: a <variable-reference>.
Case 1. The <data-description7,dd immediate component of vro has a <structure-data-
description>.
Step 1.1. Let vr be a copy of vro. Let cdd be a
<data-description>:
<structure-data-description>:
<member-description-list>,mdla.
For each <identifier-list>,idl component of bnp, taken in order, perform
Steps 1.1.1 and 1.1.2.
Step 1.1.1. For each <identifier>,id in idl, taken in order, perform Steps l-1.1.1
through 1.1.1.3.
Step 1.1.1.1. dd will have a <structure-data-description7,sdd. If dd has a
<dimensioned-data-description7, let bpl be the <bound-pair-list>
in dd; otherwise let bpl be a <bound-pair-list> with no
component. Let il be the <identifier-list> in sdd, and let mdl
be the -ember-description-list> in sdd. Let i be the integer
such that the i'th <identifier7 in il equals id, and let mdd be
the <data-description7 in the i'th -ember-description> of mdl.
Step 1.1.1.2. If mdd has a <dimensioned-data-description7, append copies of the
<bound-pair>s in mdd to bpl, and let tdd be the <item-data-
description> or <structure-data-description> immediately
contained in the <element-data-description> of mdd. Otherwise,
let tdd be the <item-data-description7 or <structure-data-
description> immediately contained in mdd.
Step 1.1.1.3. If bpl contains <bound-pair7s then let dd be a
<data-description>:
<dimensioned-data-description>:
<element-data-description>:
tdd:
bpl.
Otherwise let dd be a <data-description>: tdd.
120
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 139

Step 1.1.2. Append dd to mdla.
step 1.2. Replace dd by cdd and append bnp to vr-
step 1.3. Return vr.
Case 2. (Otherwise).
Step 2.1. Return vro.
4.4.5.3 Apply-subscripts
Operation: apply-subscripts(cr,ref,sl,al)
where cr is a $refer&ce),
ref is a <variable-reference> or a <named-constant-reference>,
sl is a {subscript-commalist+,
al is an farguments-list?.
Step 1. Let dd be the <data-description> immediately contained in ref. Let m be the
number of <bound-pair>s in dd, and let n be the number of $subscript+s in sl (n
may be 0). If n < m, and if al has a first immediate component, args, that has a
{subscript-commalist), then perform Step 1.1.
Step 1.1. Append the {subscript+ in args to sl, appending C,) as required. Delete
args from al.
Step 2. Let n be the number of Qaubscriptps in sl which are not in any {expression) also
in sl.
Case 2.1. n = 0.
Attach a <subscript-list> containing m occurrences of <asterisk> to ref.
Case 2.2. n + 0.
step 2.2.1. In this case n must equal m. Attach a <subscript-list>,sl2 to ref. For
i=l ,...,nr perform Step 2.2.1.1.
Step 2.2.1.1. Of those $subscript*s contained in cr which are not contained in
any {expression) contained in cr. let s be the i'th one. If s
immediately contains an $expression),e, perform create-
expression(e) to obtain an <expression>,e2, and append a
<subscript>: e2; to s12. Otherwise append a <subscript>:
<asterisk>; to ~12.
Step 2.2.2. For i=n,..., 1, perform Step 2.2.2.1.
Step 2.2.2.1. If the i'th <subscript> in s12 contains an <expression>, delete
the i'th <bound-pair> in dd.
Step 2.2.3. If the <bound-pair-list> in dd now has no <bound-pair>s, let dd2 be the
<item-data-description> or <structure-data-description> in dd, and then
replace dd by a <data-description>: dd2.
4.4.5.4 Create-value-reference
Operation: create-value-referencetref)
where ref is a <variable-reference>, <procedure-function-reference>,
<builtin-function-reference>, or <named-constant-reference>.
result: a <value-reference>.
Step 1. Let dd be a copy of the <data-description> immediately contained in ref.
Step 2. If ref is a <variable-reference> perform trim-dd(dd). Return a <value-
reference>: ref dd.
Chapter 4: The Translator 121
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 140

4.4.5.4.1 Trim-dd
Operation: trim-dd(dd)
where dd is a <data-description>.
step 1. Delete from dd any occurrences of the following categories that are not
contained in an <entry>:
<alignment>,
<initial>,
<identifier-list>,
<local>,
<G$X&ur>,
<varying>,
<nonvarving>.
Replace by an <asterisk> the immediate component of each <maximum-length>,
<area-size> or <bound-pair> that is not a component of an <entry>.
4.4.5.5 Create-named-constant-reference
Operation: create-named-constant-reference(dc1)
where dcl is a <declaration>.
result: a <named-constant-reference>.
step 1. Let ct be a copy of the leftmost immediate component of the <named-constant>
component of dcl. Let dt be an
<item-data-description>:
<data-type>:
<non-computational-type>:
ct.
step 2.
Case 2.1. dcl contains a <bound-pair-list>,bp.
Let n be the number of <bound-pair>s in bp. Let bpl be a <bound-pair-list>
containing n subnodes <bound-pair>: <asterisk>. Let dd be a
<dimensioned-data-description>:
<element-data-description>:
dt;
bpl.
Case 2.2. (Otherwise).
Let dd be dt-
Step 3. Let ddg be a <declaration-designator> designating dcl. Return a
<named-constant-reference>:
ddg <data-description>:
dd.
122
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 141

4.4.5.6 Create-argument-list
Operation: create-aruument-list(a1)
where al is an Carguments).
result: an <argument-list> or <absent+.
step 1. If al does not contain a (subscript-commalist? then return <absent+; otherwise
let scl be the {subscript-commalistP immediately contained in al.
Step 2. Let n be the number of CsubscriptP immediate components of scl. The Csubscript)
immediate components of scl must not immediately contain *.
Step 3. Let xal be an <argument-list> and let ectil, i=l,...,n be the CexpressioxQ
simple components of scl taken in left-to-right order.
Step 4. For each element, ecCi1, i=l,...,n, perform Steps 4.1 and 4.2.
Step 4.1. Perform create-expression(ec[i3) to obtain an <expression>,eacCil.
Step 4.2. Let rdd be the <data-description> immediately contained in eacCi1. Append to
xal an
<argument>:
eacCi1 rdd.
step 5. Return xal.
4.4.5.7 Create-builtin-function-reference
A builtin-function-name is a sequence of uppercase letters and digits such that the
corresponding sequence of lowercase letters and digits followed by "-bif' is the
category-name of a subnode of Cbuiltin-function>.
Operation: create-builtin-function-reference(ad,al)
where ad is a <declaration>,
al is an [<argument-list>].
result: a <builtin-function-reference>.
step 1. Let id be the <identifier> contained in ad. There must be a <builtin-
function>,bf. whose name or abbreviation (as listed in Section 2.7) corresponds
to the concrete-representation of id. Let bfr be a <builtin-function-
reference>: bf.
Step 2. The number of <argument>s in al must be as shown in the "Arguments" section of
the description of bf (see Chapter 9).
step 3.
Case 3.1. al is not *absent>.
Step 3.1.1. All <data-description>s immediately contained in the <argument>s in al
must satisfy the constraints given in the "Constraints" section of the
same builtin-function description.
step 3.1.2. Append al to bfr.
Case 3.2. (Otherwise).
No action.
step 4. Construct a <data-description>,rdd, as specified in the "Attributes" section of
the came (builtin-function> description. Append rdd to bfr.
step 5. Return bfr.
Chapter 4: The Translator 123
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 142

4.4.5.8 Create-pseudo-variable-reference
Operation: create-pseudo-variable-reference(ad,al)
where ad is a <declaration>.
al is an [<argument-list>].
result: a <pseudo-variable-reference>.
step 1. Let id be the <identifier> contained in ad. There must be a <pseudo-variable>,pv
whose name corresponds to the concrete-representation of id. Let pvr be a
<pseudo-variable-reference>: pv.
Step 2. The number of <argument>s in al must be as shown in the 'Arguments" section of
the description of pv (see Chapter 71.
step 3.
Case 3.1. al is not *absent+.
Step 3.1.1. All <data-description% immediately contained in the <argument>s in al
must satisfy the constraints given in the "Constraints" section of the
description of pv.
Step 3.1.2. Attach al to pvr.
Case 3.2. (Otherwise).
No action.
step 4. Construct a <data-description>,rdd, as specified in the 'Attributes" section of
the same <pseudo-variable> description. Attach rdd to pvr.
step 5. Return pvr.
4.4.5.9 Create-entry-reference
Operation: create-entry-reference(vr,all
where vr is an <value-reference> whose <data-type> has <entry>,
al is an [<argument-list>].
result: a <procedure-function-reference> or a <subroutine-reference>.
step 1. Let dd be the <data-description> immediately contained in vr.
Step 2.
Case 2.1. dd simply contains a <parameter-descriptor-list>,pdl. al must not be
<absents. The number of elements in pdl must be equal to the number of
elements in al.
Case 2.2. (Otherwise).
al must be +absent,. Go to Step 4.
step 3. For each <argument>,arg, immediate component of al, perform Steps 3.1 through
3.3.
Step 3.1. Let pdd be the <data-description> immediate component of the <parameter-
descriptor> corresponding to arg.
Step 3.2.
Case 3.2.1. arg immediately contains <expression>: <value-reference>: <variable-
reference>,var. Perform test-matching(var,pddl to obtain tv. If tv is
<false,, then attach <dummy> to arg.
124
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 143

Case 3.2.2. (Otherwise).
Attach <dummy> to arg.
Step 3.3.
Case 3.3.1. <dummy> was attached to arg in Step 3.2.
Let rdd be the <data-description> immediately contained in arg. rdd
must be proper for assignment to pdd (see Section 4.4.2.31.
Case 3.3.2. (Otherwise).
No action.
Step 4.
Case 4.1. dd simply contains a <returns-descriptor>,rd.
Let rde be a copy of the <data-description> immediately contained in rd.
Return <procedure-function-reference>: vr Call rde.
Case 4.2. (Otherwise).
Return <subroutine-reference>: vr Call.
4.4.5.10 Test-matchinq
Operation: test-matchinq(var,pd)
Step 1.
Step 2.
step 3.
step 4.
Step 5.
where var is a <variable-reference>,
pd is a <data-description> immediate component of a <parameter-
descriptor>.
result: - <trues or <false>.
Let dcl be the <declaration> designated by the <declaration-designator> in var.
If dcl contains a <defined> whose <base-item> contains an <isub>, return
<false,.
Let dd be a copy of the <data-description> immediately contained in var. Let
pdd be a copy of pd.
If any of the following subtrees exist as a component of dd or pdd then delete
every occurrence.
<local>
<initial>
<variable-reference> as component of an <offset>
<parameter-descriptor-list>
<returns-descriptor>
<options>
<identifier-list> as a component of a <structure-data-description>.
If pdd and dd are not equal, disregarding comparison of the subnodes of any
maximum-length>, <bound-pair>, or <area-size>, then return <false,.
For each <extent-expression>,el, in pdd, perform Step 5.1.
Step 5.1. If there does not exist a corresponding <extent-expression>,e2, in dd.
return *false,. If e2 contains a <refer-option>, return +false*. Let exl
and ex2 be the <expression>s in el and e2, respectively. If exl or ex2
contains a <declaration-designator>, return <false>. Perform evaluate-
restricted-expression(ex1) and evaluate-restricted-expression(ex2) to obtain
vl and v2. If vl or v2 is not a <constant> having <computational-type>,
return <false,. Otherwise, perform evaluate-expression-to-integer(exl) and
evaluate-expression-to-integer(ex2) to obtain <integer-value% il and i2
respectively. If il and i2 are equal, return <true+; otherwise return
<false*.
Chapter 4: The Translator 125
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 144

Step 6. Return <true,.
4.4.5.11 Select-generic-alternative
Operation: select-qeneric-alternative(ga,arg)
where ga is a {generic-attribute),
arg is an [<argument-list>].
Step 1.
result: a <value-reference>.
Case 1.1. arg is <absent,.
ga must have at least one *generic-element) that does not have a <generic-
description-commalist). Let rr be the {reference) immediately contained in
the first such fgeneric-element).
Case 1.2. arg is an <argument-list>.
Step 1.2.1. Let na be the number of <argument>s in arg. There must be at least one
{generic-element;) in ga. Let ge be the first such <generic-elemenq.
Step 1.2.2. Let ngd be the number of Cgeneric-description)s in ge that do not
declaration-contain MEMBER. If ngd does not equal na, go to step 1.2.5.
Step 1.2.3. For i=l,...,na, perform Steps 1.2.3.1 and 1.2.3.2.
Step 1.2.3.1. Let ai be the i'th <argument> in arg.
Let gd be the i'th {generic-description in ge that does not
declaration-contain MEMBER and let gdl be sgeneric-description-
list): gd.
Case 1.2.3.1.1. There is a {generic-description) in ge that follows gd and
does not declaration-contain MRMBER.
Let gdf be the leftmost such $generic-description+ Append to gdl, in left-to-right order. copies of all {generic-
description)s in ge between gd and gdf.
Case 1.2.3.1.2. (Otherwise).
Append to gdl in left-to-right order, copies of all
(generic-descriptions in ge following gd.
step 1.2.3.2. Perform test-generic-matchingtai,gdl) to obtain tval. If tval is
<false go to step 1.2.5.
Step 1.2.4. Let rr be the $reference) immediately contained in ge. Go to step 2.
Step 1.2.5. Let ge be the next {generic-elemenq of ga following the current ge.
There must be such a {generic-element?. Go to Step 1.2.2.
step 2. Perform find-applicable-declaration0 to obtain a $declaratioe,dcl, which
must not contain a (generic-attribute).
Step 3. Perform create-referenceCrr,<value-reference>) to obtain vr. Return vr.
126
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 145

4.4.5.12 Test-qeneric-matchinq
Operation: test-qeneric-matchinq(airgdl)
where ai is an <argument>,
gdl is a {generic-description-list).
result: *ws or *false*.
Case 1. gdl contains only {generic-description): *.
Return t-3.
Case 2. (Otherwise).
step 2.1. If ai is of the form
<argument>:
<expression>:
<value-reference>:
<variable-reference>,vr;;;
then let dd be the <data-description> immediately contained in vr.
Otherwise, let dd be the <data-description> immediately contained in ai.
Step 2.2. Perform test-generic-aggregation(gdl,dd) to obtain tval. Return tval.
4.4.5.13 Test-qeneric-aqqreqation
Operation: test-qeneric-aqqreqation(gdl,dd)
where gdl is a *generic-description-list?,
dd is a <data-description>.
result: - *true* or *false,.
step 1. Let gd be the first Tgeneric-description) component of gdl. If gd declaration-
contains DIMENSION then perform Steps 1.1 through 1.3.
Step 1.1. If dd does not immediately contain a <dimensioned-data-description> then
return <false,.
Step 1.2. If the number of * components of the +Z+-commalisQ of gd is not equal to the
number of <bound-pair> components of the <bound-pair-list> of dd, then
return <false>.
Step 1.3. Delete the DIMENSION {asterisk-bounds? declaration-contained in gd.
Step 2. If dd immediately contains a <dimensioned-data-description> then let dd be a
<data-description>: tdd; where tdd is the immediate component of the <element-
data-description> of dd.
Step 3.
Case 3.1. gd declaration-contains STRUCTURE.
Step 3.1.1. Let Iv be the value of the {level) of gd. Let nl be the number of
{level) components of gdl with value equal to Iv+1 which follow gd
without any intervening component of gdl whose {level) has value less
than or equal to Iv.
Step 3.1.2. If dd does not immediately contain a <structure-data-description>,sdd
then return <false+. Let ndd be the number of <data-description> simple
components of sdd. If ndd does not equal nl then return <false,.
Chapter 4: The Translator 127
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 146

Step 3.1.3. For i=l,..., nl perform Steps 3.1.3.1 to 3.1.3.4.
Step 3.1.3.1. Let tgdl be a copy of gdl. Delete from tgdl the Cgeneric-
description2 immediate components that precede the i'th {generic-
description) whose {level) has value lv+l.
step 3.1.3.2. Let tdd be the i'th <data-description> of dd.
Step 3.1.3.3. Perform test-generic-aggregation(tgdl,tdd) to obtain tval.
Step 3.1.3.4. If tval is <false> then return <false>.
Case 3.2. (Otherwise).
Step 3.2.1. Delete any {level) or MEMBER components of gd.
Step 3.2.2. Perform test-generic-description(gd,dd) to obtain tval.
Step 4. Return tval.
4.4.5.14 Test-qeneric-description
Operation: test-qeneric-description(gd,dd)
where gd is a {generic-description'),
dd is a <data-description>.
result: - <true+ or *false+.
Step 1.
Case 1.1. dd immediately contains <structure-data-description>.
Return ffalse,.
Case 1.2. dd immediately contains <dimensioned-data-description>.
Return ffalse,.
Case 1.3. dd immediately contains <item-data-description>.
No action.
Step 2. If dd does not have <alignment> then delete any ALIGNED or UNALIGNED
declaration-contained in gd.
Step 3. If dd has neither <varvinq> nor <nonvarvinq> then delete any VARYING or
NONVARYING declaration-contained in gd.
Step 4. Let gdal be the $generic-data-attribute-list) in gd. For each fgeneric-data-
attribute),gda in gdal whose first immediate component atr appears in Table 4.1,
perform Steps 4.1 to 4.4.
Step 4.1. Perform create-abstract-equivalent-treetatr) to obtain absatr.
Step 4.2. If dd does not simply contain a node whose node type is the same as absatr
then return <false>.
Step 4.3. If atr is neither ENTRY nor PRECISION then delete gda from gdal.
Step 4.4. If atr is ENTRY or PRECISION and is the sole component of gda, then delete
gda from g&l.
128
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 147

Step 5. If gdal contains a Cgeneric-data-attributep,gda which immediately contains a
{generic-precision),gprec then perform Steps 5.1 to 5.3.
Step 5.1. If dd does not have <precision> or if dd has <pictured>, then return
ff alse,.
Step 5.2. Let prec be the <precision> of dd. Perform test-generic-
precision(gprec,prec) to obtain tval.
Step 5.3. If tval is *false> then return <false,. Otherwise delete gda from gdal.
Step 6. If gaal contains a Cgeneric-data-attributeg,gda which immediately contains a
{description-commalist$, then perform steps 6.1 to 6.4.
Step 6.1. If dd does not have a <parameter-descriptor-list> then return <false+.
Step 6.2. Let pdl be the <parameter-descriptor-list> simply contained in dd.
Step 6.3. gd must not contain any {identifier) components. Perform create-data-
description(gd) to obtain a <data-description>,gdd. Perform replace-
concrete-designators(gdd). Let gpdl be the <parameter-descriptor-list> in
gdd. For each <parameter-descriptor>,gpd in gpdl. perform validate-
descriptor(gpd1.
Step 6.4. If gpdl is not equal to pdl then return <false,. Otherwise delete gda from
gdal.
Step 7. If gdal contains a (generic-data-attribute),gda which immediately contains a
{returns-descriptor) then perform Steps 7.1 to 7.4.
Step 7.1. If dd does not have a <returns-descriptor> then return <false,.
Step 7.2. Let rd be the <returns-descriptor> simply contained in dd.
Step 7.3. ga must not contain any <identifier) components. Perform create-data-
description(qd1 to obtain a <data-description>,gdd. Perform replace-
concrete-designators(gdd). Let grd be the <returns-descriptor> in gdd.
Perform validate-descriptor(grd).
Step 7.4. If grd is not equal to rd then return +false>. Otherwise delete gda from
gdal.
Step 8. If gdal contains a {generic-data-attribute3.gda which immediately contains
$picture).p then perform Steps 8.1 to 8.4.
Step 8.1. If dd does not have a <pictured> then return <false>.
Step 8.2. Let pd be the <pictured> component of dd.
Step 8.3. Perform create-picture(p) to obtain a <pictured>,ap.
Step 8.4. If ap is not equal to pd then return <false>; otherwise delete gda from
gdal.
Step 9. All nodes must have been deleted from gdal. Return <true>.
Table 4.1. Concrete Terminals of Significance to Test-generic-description.
i i
1 ALIGNED COMPLEX FLOAT POINTER
I AREA DECIMAL FORMAT PRECISION 1
1 BINARY ENTRY LABF,L REAL
1 BIT FILE NONVARYING UNALIGNED
1 CHARACTER FIXED OFFSET VARYING 1 L-------,----------------------------------------------------------------,
Chapter 4: The Translator 129
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 148

4.4.5.15 Test-qeneric-precision
Operation: test-qeneric-precision(gprec,prec)
where gprec is a fgeneric-precision),
prec is a <precision>.
result: *true, or <false+.
step 1.
Case 1.1. gprec contains Cnumber-of-digits) C:) {number-of-digits).
Let gplo be the value of the {integer) component of the first (number-of-
digits) of gprec and let gphi be the value of the *integer) component of the
second {number-of-digits) component of gprec.
Case 1.2. (Otherwise).
Let gplo and gphi both be the value of the Cintegera component of the sole
{number-of-digits) component of gprec.
Step 2. Let p be the value of the <integer> component of the <number-of-digits> of prec.
If it is not the case that gplo I p 5 gphi then return <false>.
Step 3.
Case 3.1. gprec does not have a {scale-factor) and prec does not have a <scale-
factor>.
Case 3.2. gprec has a {scale-factor) and prec does not have a <scale-factor>.
Return <false,.
Case 3.3. gprec has {scale-factor+ C:) {scale-factor).
Let gslo be the value of the {integer) component of the first {scale-factor)
of gprec and let gshi be the value of the Cintegert component of the second
{scale-factor) of gprec.
Case 3.4. (Otherwise).
Let gslo and gshi both be the value of the Cinteger) component of the sole
{scale-factor? component of gprec.
Step 4. Let s be the value of the <signed-integer> component of the <scale-factor> of
prec.
Step 5. If it is the case that gslo I s 5 gshi then return *true>; otherwise return
<false+.
130
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 149

4.4.6 CREATE-PICTURE
A $picture) may occur as a component of a {declaration) or a Cformat-item. In both
cases it is translated to a <pictured>. Elements of a {picture) may be repeated by the
specification of a {repetition-factor) which is expanded into a sequence of elements
first. Then the (picture) is translated to a <pictured-character> or <pictured-numeric>.
The content of a {picture) is governed by the following syntax:
spicture-content):: =$picture-item-list) [{picture-scale-factor?1
(picture-item)::= [*repetition-factor?1 {picture-element)
eepetition-factor)::= ({integer?)
{picture-element)::= ~l~l~1~I~I~I~l~l~I~l~l~l~l+l-l*l~l~l~l~l~l~
Cpicture-scale-factor)::= F ({signed-integer?)
Operation: create-picture(p)
where p is a $picture.
result: a <pictured>.
Step 1. There must be a {string-or-picture-symbol-listt,spsl in p. Let t be a Cpicture-
content) whose concrete-representation is the same as that of spsl. The tree t
must exist and be unique.
step 2. For each component of t which is a
{picture-item),c:
{repetition-factor*,rf
{picture-elementa,pe;
let iv be the decimal value of the Tinteger) in rf. iv must not be equal to
zero. Replace c by iv occurrences of a {picture-item): pe.
step 3.
case 3.1. t contains a {picture-element*: A; or a {picture-element): X.
If p is a component of a $declaration),d, a $descriptioe,d, or a Cgeneric-
description),d then d must not declaration-contain REAL or COMPLEX. All
terminal nodes of t must be A, X, or 9. Return <pictured>: <pictured-
character>: <character-picture-element-list>,cpel;; where the concrete-
representation of cpel is the same as that of t.
Case 3.2. (Otherwise).
Perform create-numeric-picture(t) to obtain a <pictured-numeric>,pn. If P is a component of a $declarationt, {descriptioq, or sgeneric-description
which declaration-contains COMPLEX, then replace the <*> component of pn
by <comnlelr>- Return <picture=: pn.
Chapter 4: The Translator 131
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 150

4.4.6.1 Create-numeric-picture
The picture-validation syntax is as follows:
*numeric-picture-specification*::= <fixed-point-picture) 1 <floating-point-picture)
{fixed-point-picture)::= *non-drifting-field) 1 {drifting-field)
{non-drifting-field)::= C Cdigits) l [{signpI l [$I 3 1
C *digits* l [$I 1 C {credit? 1 Cdebit) 3
{digits)::= Cpic-digit-list) C V C$pic-digit-list)11 I
V $pic-digit-list+ I
{Z-list) [{scaled-digits-field?] I
C(Z-list)1 V (Z-list) I
(+-list) [{scaled-digits-field)3 I
[$*-list)1 V $*-list)
sscaled-digits-field)::= Cpic-digit-list) C V C(pic-digit-list*11 I
V CCpic-digit-list)]
(pit-digit)::= 9lIIRITIY
{sign)::= S I + I -
<drifting-field)::= ( (drifting-sign-field) l [$I 1 I
i[ {drifting-dollar-field? l C+z.ign)l 1 I
$drifting-dollar-field* C$credit) 1 $debit)l
{credit)::= CR
(debit)::= DE
sdrifting-sign-field*::= $signs% C+acaled-digits-field?] I
S [@-listI+ V {S-list* I
+ CCplus-list)1 V (plus-list? I
- [Qninus-list+1 V winus-list?
{plus)::= +
#minus)::= -
(signs)::= S {S-list) I <plus) $plus-list) I Qninus) *inus-list)
{drifting-dollar-field)::= $ ($-list) [{scaled-digits-field?] I
$ [($-list)1 V +$-list+
{floating-point-picture)::= {pit-mantissa? {pit-exponent*
$pic-mantissa)::= [<sign)1 (digits) I {drifting-sign-field?
$pic-exponent)::= CEIX) C$sign)l C Cpic-digit-list? 1
CC (Z-list) I $+-list) I C$pic-digit-list?: )I
A diqit-position is any occurrence of a Cpic-digit) or 2 or l , or any occurrence of S, +,
-, or $ in an {S-list+, {plus-list?, {minus-list), or +$-list).
132
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 151

Operation: create-numeric-picture(p)
where p is of the form {picture-content): {picture-item-list),pil [{picture-
scale-factor+,psfl.
result: a <pictured-numeric>.
Step 1. Let pilw be a copy of pil. Delete from pilw any <picture-item) which contains a
{picture-element) containing a
, or
J; :,,or
B' if that {picture-item) does not immediately follow a {picture-item):
{picture-element): D.
A terminal node so deleted must not have occurred immediately between a C and an
R nor immediately between a D and a B.
Step 2. It must be possible to construct a {numeric-picture-specificatioM,nps according
to the picture-validation syntax (above), such that the concrete-representation
of nps is the same as that of pilw.
If psf exists, then nps must contain a {fixed-point-picture?.
Step 3. Let pn be a partial tree
<pictured-numeric>:
<numeric-picture-specification>,anps
<arithmetic>:
<mode>:
<real>;
(base>:
<decimal>;
<scale>,s
<precision>,prec:
<number-of-digits>: n;;;;
where n is the number of digit-positions in {fixed-point-picture) or in {pit-
mantissa?, in nps. n must not be greater than the maximum Cnumber-of-digits>
for <base>: <decimal>; and <scale> to be set below.
Note: The node <real> may be replaced by <complex> at a later stage.
Step 4. If nps contains a {floating-point-picture? then attach <float> to s; otherwise
attach <fixed> to s.
Step 5.
Case 5.1. nps contains a {fixed-point-picture?.
Step 5.1.1. nps must not contain more than one (pit-digit* which has a T, I, or R.
nps must not contain a Cpic-digit) which has a T, I, or R if it also
contains S, +<, -, {credit), or {debit). Attach to anps a (fixed-point-
picture> with the same concrete-representation as pil.
Step 5.1.2. If psf exists, then let v be the result of interpreting the $signed-
integer),si. in psf as a decimal constant, and attach to anps a
<picture-scale-factor> containing the <signed-integer> abstract-
equivalent of si; otherwise, let v=O. Let vl=(n-vl where n is the
number of digit-positions after the V in nps if V appears, and 0
otherwise. Attach a <scale-factor>: vl; to prec.
Case 5.2. nps contains a {floating-point-picture?.
Step 5.2.1. Let pm be a <numeric-picture-element-list> whose concrete-representation
is the same as that of pi1 up to (but not including) the E or K. pm
must not contain any of T, I, or R.
Step 5.2.2. Let pe be a <numeric-picture-element-list> whose concrete-representation
is the same as that of pi1 beginning with the E or K. pe must not
contain any of T, I, or R.
Chapter 4: The Translator 133
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 152

step 5.2.3. Attach to anps a
<floating-point-picture>:
<picture-mantissa>:
pn; <picture-exponent>:
pe-
Step 6. Return pn.
4.4.7 CREATE-CONSTANT
Operation: create-constant(c)
where c is a {constant).
result: a <constant>.
case 1. c contains a {simple-character-string-constant).
Step 1.1.
Case 1.1.1. c contains no *string-or-picture-symbol-list*.
Let csv be a *character-string-value+: *null-character-strinq*.
Case 1.1.2. (Otherwise).
step 1.1.2.1. Let csv be a *character-string-value* whose $symbol)s (in order)
have the same concrete-representations as the Cstring-or-picture-
symbol)s in c, except that each {string-or-picture-symbol?: $*');
in c becomes a $symbol): $*p; in csv.
Step 1.1.2.2. If c contains a /replicated-string-constant),rsc, perform Step
1.1.2.2.1.
step 1.1.2.2.1. Let j be the value obtained by interpreting the $integer) in
rsc as a decimal constant: let i be the number of
*character-value98 in csv.
If j=O, let v be a *null-character-string+: otherwise, let v
be a icharacter-value-list, with i+j components, the
(n+i+k)'th component equalling the k'th <character-values of
CSV, for k=l,...,i, and n=O,...,(j-1).
Replace the <character-value-list* in csv by v.
Step 1.2. Return a <constant>: fbasic-values: csv; dt; where dt is a <data-type>
containing <character>, <nonvarvinq>, and Unaximum-length>: <asterisk>.
Case 2. c contains a Csimple-bit-string-constant*.
Step 2.1.
Case 2.1.1. c contains no *string-or-picture-symbol-list*.
Let bsv be a fbit-string-value*: <null-bit-strinqs.
Case 2.1.2. (Otherwise).
step 2.1.2.1. Let m be 1,2,3,4 according to whether sradix-factor) in c has Bl,
82, 83, or B4. Let slil, i=l ,...,k, be the @tring-or-picture-
symboys in c.
Each stil must have an entry in Table 4.2 which is valid for the
value of m. Let bsv be a <bit-string-value+ containing m+k tbit-
value%, such that <bit-value% (i+m+l-m) through (i*m) are
obtained from Table 4.2 as a function of m and sCi1, i=l,...,k.
134
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 153

Step 2.1.2.2. If c contains a <replicated-string-constant)rrsc, perform step
2.1.2.2.1.
Step 2.1.2.2.1. Let j be the value obtained by interpreting the *integer* in
rsc as a decimal constant: let i be the number of fbit-
value7s in bsv.
If j=O, let v be a -null-bit-strinq7; otherwise, let v be a
*bit-value-list* with i+j components, the (n+i+k)'th
component equalling the k'th fbit-value7 of bsv, for
k=l,...,i, and n=O,...,(j-1).
Replace the <bit-value-list* in bsv by v.
Step 2.2. Return a <constant>: <basic-value>: bsv; dt; where dt is a <data-type>
containing &b>, <nonvarvinq>, and Unaximum-length>: <asterisk>.
Case 3. c contains an {arithmetic-constant),ac.
Step 3.1.
Step 3.2.
Step 3.3.
Step 3.4.
Perform evaluate-real-constant(rc1, where rc is the (real-constant) in ac,
to obtain a *value-and-type,: <real-value7.v <data-type>,t.
Let ds be a partial $declaration) containing:
CONSTANT
if c contains F, then FIXED
if c contains E. then FLOAT
if c contains I, then COMPLEX
otherwise, REAL.
(ds is partial in that it contains no fidentifier)).
If c does not contain P, then attach a partial $unit),u: Cdeclare-
statement),d; to the {procedure) or pegin-block) which block-contains c,
attach a coPY# cds, of ds to d, perform apply-defaults(cds), let ds be a
copy of cds, and delete u.
Let dt be a partial <data-type>: <computational-type>: <arithmetic>;; and
complete it as follows:
Step 3.4.1. For each of the following which is contained in ds, append the abstract-
equivalent to dt: BINARY, DECIMAL, FIXED, FLOAT, REAL, COMPLEX.
If dt is still without <base> or <scale> (or both), copy the <base7 or
<scale7 (or both) from t.
Step 3.4.2. Let cp be the converted <precision> of t for the <base> and <scale> of
dt.
step 3.4.3.
Case 3.4.3.1. ds contains a $precision),p.
Perform create-abstract-equivalent-tree(p) to obtain a
<precision>,ap.
If dt contains <float7 , ap must not contain a <scale-factor>.
If dt contains <fixed7 and ap contains no <scale-factor>, attach
<scale-factor>: 0; to ap.
If dt contains <fixed>, the amount by which the <number-of-
digits7 exceeds the <scale-factor> in ap must not be less than
that for cp.
Attach ap to dt.
Case 3.4.3.2. (Otherwise).
Attach cp to dt.
chapter 4: The Translator 135
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 154

Step 3.5.
Case 3.5.1. ac immediately contains a +real-constant).
Perform convert(dt,t,v) to obtain a *real-value,,rv. Let bv be a
*basic-value,: rv.
Case 3.5.2. ac contains an {imaginary-constant).
Let rdt be a <data-type> which has <u> but is otherwise as dt.
Perform convert(rdt,t,v) to obtain a *real-value>,rv. Let bv be a
<basic-value+: *complex-value+; with real part: 0; and imaginary part:
rv.
Step 3.6. Return a <constant>: bv dt.
Table 4.2. Table of <bit-value% as a Function of +qmbol~s and {radix-factor)s for
Create-constant.
r’-------------------------------------------------
IContents of i'th +symbol) [Contents of <bit-value%;
lor {string-or-picture- I(i*m+l-m) through (i*m) 1
---------------------------------------------------
indicates that the corresponding (symbol) or I
{string-or-picture-symbol) is invalid for this I
value of m
I 0 indicates <zero-bit+
1 1 indicates <one-bit+
L------------------------------------------------,
136
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 155

4.5 Validation of the Abstract Procedure
An abstract <procedure> has been constructed so far by the Translator, corresponding to
the specified concrete {procedure). This abstract <procedure> is now examined by some
final tests before being returned as the result of the translate operation. These tests
validate each declaration and apply constraints.
Operation: validate-procedure(ap1
where ap is a <procedure>.
Step 1. Perform apply-constraints(ap).
step 2. For each <declaration>,ad component of ap perform validate-declaration(ad).
Step 3. For each <parameter-descriptor>,d and for each <returns-descriptor>,d contained
in ap perform validate-descriptor(d).
Step 4. ap must satisfy all the Constraints appearing under the heading 'Constraint:" in
the Abstract Syntax.
Step 5. Each <do-spec> must satisfy the Constraints specified in Section 6.3.4.
4.5.1 VALIDATE-DECLARATION
When all the <declaration>s have been completed, and their contained <expression>s and
<value-reference>s properly completed, some additional validation is required to ensure
that such <declaration>s do represent realistic data entities.
Operation: validate-declaration(ad)
where ad is a <declaration>.
Step 1. If ad contains <automatic>, <based>, <controlled>, <defined>, <parameter>, or
<static> then perform validate-automatic-declaration(ad), validate-based-
declaration(ad). validate-defined-
declaration(ad)j
validate-controlled-declaration(ad1,
validate-parameter-declaration(ad), or validate-static-
declaration(ad), respectively.
Step 2. If ad contains <named-constant>, then perform validate-static-declaration(ad).
Step 3. If ad contains any <data-description>,dd which simply contains <initial>, then
for each such dd perform Step 3.1.
Step 3.1. The <data-description> immediate component of each <parenthesized-
expression> immediate component of an <initial-element> contained in dd must
be proper for assignment to dd.
4.5.2 VALIDATE-AUTOMATIC-DECLARATION
Automatic declarations must satisfy constraints which enable them to be allocated and
possibly initialized at the time that the block to which they belong is being activated.
Operation: validate-automatic-declaration(ad)
where ad is a <declaration>.
step 1. ad must not contain an <area-size>: <asterisk>;, a maximum-length>:
<asterisk>;, or a <bound-pair>: <asterisk>;, except as subnodes of <entry>.
Step 2. Each <extent-expression> in ad must not contain a <refer-option>.
Step 3. If ad contains a <variable-reference>: <declaration-designator>,dd; then dd must
not designate a <declaration>,d, containing <automatic> or <defined> if d is a
block-component of the same block as ad.
Chapter 4: The Translator 137
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 156

4.5.3 VALIDATE-BASED-DECLARATION
Based declarations may contain <structure-data-descriptiotis some of whose <member-
description% refer to other (previous) members of the same structure by means of the
<refer-option>.
Operation: validate-based-declarationtbd)
where bd is a <declaration>.
step 1. Bach <area-size>, <maximum-length>, or <bound-pair> in bd must not contain
<asterisk>, except as subnodes of <entry>.
step 2. For each <refer-option>,ro in bd, perform steps 2.1 to 2.4.
Step 2.1. ro must be simply contained in a -ember-description>,ms.
Step 2.2. Let idCl1 ,...,idtnl be the components of the <identifier-list> in ro. id[ll
must be equal to the <identifier> immediately contained in bd; bd must
simply contain a <structure-data-descriptioti,sdCll: idlC11 mdlCl1. For
i=l,...,(n-l), idltil must have an <identifier>,idctil equal to idCi+ll; for
i=l,...,(n-2), the <member-description> in mdlCi1 corresponding to idcCi1 in
idlCi3 must simply contain a <structure-data-description>,sdCi+ll: idlCi+ll
mdl[i+ll; the <member-description>,mo in mdlCn-11 corresponding to idcCn-11
in idlCn-11 must immediately contain a <data-description>: <item-data-
description>: <data-type>: <computational-type>;:; and mo must not be
contained in a <dimensioned-data-description>.
Step 2.3. mo must occur to the left of ms in bd.
Step 2.4. For every <structure-data-description>,sdd other than sdCl1 which contains
both mo and ms perform Step 2.4.1.
Step 2.4.1. For every <item-data-description>,iddl which is contained in sdd and is
to the right of ms, there must exist at least one <item-data-
description>,idd2 contained in sdd, which either is simply contained in
ms or is to the left of ms, such that iddl and idd2 match as defined in
Step 2.4.1.1.
Step 2.4.1.1. Let iddl' and idd2' be copies of iddl and idd2, modified as
follows. Delete any occurrences of <initial>, Unaximum- length>,
<number-of-digits>, and <local>; delete any subnodes of <offset>,
<entry>, and <area>; replace any occurrences of <pictured> by
<string>: <string-type>: <character>; <nonvarvinq>.
For iddl and idd2 to match, iddl' and idd2' must be equal, and,
if iddl and idd2 have <arithmetic>, the <number-of-digits> of
iddl must be less than or equal to the <number-of-digits> of
idd2.
4.5.4 VALIDATE-CONTROLLED-DECLARATION
Operation: validate-controlled-declaration(cd)
where cd is a <declaration>.
step 1. cd must not contain an <area-size>: <asterisk>;, a Unaximum-length>: <asterisk>;
or a <bound-pair>: <asterisk>;, except as subnodes of <entry>.
Step 2. Each <extent-expression> in cd must not contain a <refer-option>.
step 3. If cd has <external>, then for each <expression> , e simple component of an
<extent-expression> of cd, perform Step 3.1.
Step 3.1. Perform evaluate-restricted-expression(e) to obtain c. Sf c is a <constant>
having <computational-type> then replace the first immediate component of e
by c. (This is preparatory to consistency checking of constant <extent-
expression>s in validate-external-declaration. If c is *fails, this
indicates that e is not a restricted-expression , and e remains unchanged.)
138
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 157

4.5.5 VALIDATE-DEFINED-DECLARATION
Operation: validate-defined-declaration(dd)
where dd is a <declaration>.
step 1. Perform validate-automatic-declaration(dd1.
step 2. Let dd contain the form <defined>,def: (base-item>: <variable-reference>,vr. If
vr contains an <isub> then def must not contain a <position> and vr must not
contain an <asterisk>.
step 3. Each <declaration-designator> in dd must not designate dd.
step 4. The <data-description> immediately contained in vr and the <data-description>
immediately contained in the <variable> in dd must not contain <varvinq>, other
than as a subnode of <entry>.
4.5.6 VALIDATE-PARAMETER-DECLARATION
Operation: validate-parameter-declaration(pd)
where pd is a <declaration>.
step 1. Each <extent-expression> in pd must not contain a <refer-option>.
step 2. For each <extent-expression>: <expression>,e; in pd perform steps 2.1.
Step 2.1. Perform evaluate-restricted-expression(e) to obtain c. c must be a
<constant>. Replace the first immediate component of e by c.
4.5.7 VALIDATE-STATIC-DECLARATION
Operation: validate-static-declaration(sd)
where sd is a <declaration>.
step 1. Perform validate-automatic-declaration(sd).
step 2. For each <expression>,e simple component of sd which is not a component of an
<offset> perform evaluate-restricted-expression(e) to obtain c, which must be a
<constant> or a <value-reference>. If c is a <value-reference> then it must be
a component of an <initial-element>. Replace the first immediate component of e
by c.
4.5.8 VALIDATE-DESCRIPTOR
Operation: validate-descriptor(d)
where d is a <parameter-descriptor> or a <returns-descriptor>.
step 1. For each <extent-expression>: <expression>,e; in d, perform step 1.1.
Step 1.1. Perform evaluate-restricted-expression(e) to obtain c. c must be a
<constant>. Replace the first immediate component of e by c.
step 2. If d is a <parameter-descriptor>, any <entry> simply contained in d must not
have any subnodes.
step 3. d must not contain any <refer-option%.
Chapter 4: The Translator 139
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 158

4.5.9 EVALUATE-RESTRICTRD-EXPRESSION
Operation: evaluate-restricted-expression(e)
where e is an <expression>.
result: a <constant> or a <value-reference> or l =3+
step 1. Par each <expression>,ex simply contained in e , perform Steps 1.1 and 1.2.
Step 1.1. Perform evaluate-restricted-expression to obtain r- If r is *fail> then
return *fail*.
Step 1.2. Replace the first immediate component of ex by r.
step 2.
Case 2.1. e immediately contains a <value-reference>,vr.
If vr immediately contains either a <named-constant-reference> or a
<builtin-function-reference> which has <empty-bif> or <null-bif>, then
return vr; otherwise return <w>.
Case 2.2. (Otherwise),
Step 2.2.1. If e has an <infix-operator> which does not contain <e>, <subtract>,
Unultiply>. <divide>, or <s>, then return +m*-
step 2.2.2. Perform evaluate-expression(e) to obtain an *aggregate-value*,av. av
must immediately contain an *aggregate-type, which immediately contains
*scalar,. Let v be the fbasic-value> in av.
Step 2.2.3. Let dt be the <data-type> in the <data-description> immediate component
of e.
Step 2.2.4. Return a <constant>: v dt.
4.5.10 APPLY-CONSTRAINTS
In certain contexts of an abstract <procedure> only restricted forms of the specified
category are permitted. The restrictions are shown by a constraint-expression enclosed
in parentheses. The definition of a constraint-expression is as follows.
constraint-expression::= multiple-constraint1
constraint-expression 1 multiple-constraint
multiple-constraint::= constraint 1
multiple-constraint & constraint
constraint::= v constraint 1 {area 1 based 1 character 1 condition 1 named-constant 1
defined I computational-type I file 1 format I controlled I label 1
locator 1 pointer I scalar 1 variable)
Operation: apply-constraints(p)
where p is a <procedure>.
Step 1. For each subnode, c in p corresponding to a category-name in the Abstract Syntax
with an attached constraint-expressionrce, if c is an <expression-list>, let c
be each component of the list in turn, and perform steps 1.1 and 1.2; otherwise,
perform steps 1.1 and 1.2.
140
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 159

Step 1.1.
case 1.1.1. c is a <variable-reference> and ce contains a constraint containing
'defined" or "based".
Let d be the <declaration> designated by the <declaration-designator>
immediately contained in c.
Case 1.1.2. c is a <declaration-designator>.
Let d be the <declaration> designated by c.
Case 1.1.3. c is a <variable-reference> (other than as in Case 1.1.11, a <value-
reference>, an <expression>, a <parenthesized-expression>, a <target-
reference>, or a <named-constant-reference>.
Let d be the <data-description> immediately contained in c.
Step 1.2. Perform test-constraints(d,cel to obtain r.
r must be <trues.
4.5.11 TEST-CONSTRAINTS
Operation: test-constraints(d,ce)
Case 1.
Case 2.
Case 3.
Case 4.
Case 5.
Case 6.
Case 7.
where d is a <declaration> or a <data-description>,
ce is a constraint-expression , a multiple-constraint, or a constraint.
result: <true, or <false>.
ce is of the form constraint-expression: constraint-expression,cce 1 multiple-
constraint,mc.
Perform test-constraints(d,ccel to obtain rl; perform test-constraints(d,mc) to
obtain r2. If rl is *true+, return ge+; otherwise return r2.
ce is of the form multiple-constraint: multiple-constraint,mc a constraint,ct.
Perform test-constraints(d,mc) to obtain rl; perform test-constraints(d,ct) to
obtain r2. If rl is <false+, return <false+; otherwise return r2.
ce is of the form constraint: 7 constraint,&.
Perform test-constraints(d,ct) to obtain r. If r is *false,, return 4-5;
otherwise return <false.
ce is a constraint: scalar.
If d is a <declaration> which has <variable> immediately containing <data-
description>: <item-data-description>; then return *e>- If d is a <data-
description>: <item-data-description>: then return +a+- Otherwise return
afalse*.
ce is a constraint: computational-type.
If d contains <non-computational-type> then return <false>; otherwise return
<w> -
ce is a constraint other than in Cases 3 and 4.
If d contains a category-name equal to ce other than as a subnode of <entry>,
return *true,; otherwise return <false,.
(Otherwise).
Let cs be the immediate component of ce. Perform test-constraints(d,cs) to
obtain r. Return r.
Chapter 4: The Translator 141
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 160

4.6 Validate-program
Operation: validate-program
Step 1. For each distinct <identifier> component of the <program> which is an immediate
component of a <declaration> which contains <external>, let ad1 be a
<declaration-list> containing copies of all such <declaration>s and perform
validate-external-declaration(ad1).
4.6.1 VALIDATE-EXTERNAL-DECLARATION
Operation: validate-external-declaration(ad1)
where ad1 is a <declaration-list>.
Step 1. Delete all <identifier-list> components of ad1 which are immediate components of
<structure-data-description>.
Step 2. Delete all <variable-reference>s which are immediate components of an <offset>.
step 3. For each <declaratioti,d which has <storage-class>: <static>; and for each
<item-data-description> component of d which contains an <initial>, change any <initial-element> which contains an <iteration-factor> and an <initial-element-
list> into the eguivalent number of <initial-element>s.
Step 4. If any <declaration>,d component of ad1 contains <storage-class>: <controlled>:
then delete every <initial> component of d and every <extent-expression>
component whose <expression> does not immediately contain <constant>.
Step 5. For each <extent-expression>,ee in ad1 containing a <constant>,c, perform Step
5.1.
Step 5.1. Let e be the <expression> of ee. Perform evaluate-expression-to-integer(e)
to obtain an <integer-value,,iv. Replace e by iv.
step 6. For each <initial-element>,ie in ad1 containing a <constant>,c, perform Step
6.1.
Step 6.1. Let tdt be the <data-type> immediately contained in the <item-data-
description> containing ie- Let cdt be the <data-type> of c, and let cbv be
the <basic-value+ of c. Perform convert(tdt,cdt,cbv) to obtain a *basic-
values, bv. Replace c by <constant>: bv tdt.
step 7. Delete any <local> which is an immediate component of a Cnon-computational-
type>.
step 8. In an implementation-defined fashion , compare <declaration>s of ad1 which have
<environment> components with those which do not have corresponding
<environment> components, and compare corresponding <environment> components of
the <declaration>s of adl. Delete all <environment> components of adl.
step 9. In an implementation-defined fashion, compare <declaration>s of ad1 which have
<options> components with those which do not have corresponding <options>
components, and compare corresponding <options> components of the <declaration>s
of adl. Delete all <options> components of adl.
Step 10. All <declaration> components of ad1 must be equal.
142
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 161

Chapter 5: The PL/I Interpreter
5.0 Introduction
This chapter gives the interpretation-state part of the Machine-state Syntax and also
introduces the interpretation phase of the definition. Section 5.1 defines the
interpretation-State. Section 5.2 defines some terminology used in the subsequent
chapters. Section 5.3 gives the operation interpret and some operations called from it
to initialize and terminate the interpretation phase. The subsequent chapters complete
the definition of the interpretation phase.
5.1 The Interpretation-state
M6.
M7.
<interpretation-state,::= <program-state+ <allocated-storage, [fdataset-list+3
<program-state>::= <program-directory>
C&lock-state-list+3
Iffile-information-list,3
5.1.1 DIRECTORIES
MS. <program-directory+::= *static-directory,
fcontrolled-directory+
tfile-directory>
M9.
MlO.
Mll.
M12.
fstatic-directory,::= [<static-directory-entry-list*1
fstatic-directory-entry+::= C <external> 1 <declaration-designator> 3
<identifier> <generation,
<controlled-directory*::= [fcontrolled-directory-entry-list,3
<controlled-directory-entry,::= E <external> 1 <declaration-designator> 1
<identifier> [<generation-list91
M13. tfile-directory*::= [4file-directory-entry-list91
M14. ffile-directory-entry+::= C <external> 1 <declaration-designator> 3
<identifier> [<subscript-value-list91
ffile-information-designator,
M15. fsubscript-value+::= *integer-value>
5.1.2 BLOCK STATE
M16. *lock-state,::= <block-directory+ <block-control9
<linkage-part+ [*block-environments1
[<established-on-unit-list91 [<condition-bif-value-list+1
Cfcopy-file91
M17. &lock-directory*::= *automatic-directory+
<defined-directory+
[*parameter-directory91
Ml8.
Ml.9.
M20.
eutomatic-directory%::= [tautomatic-directory-entry-list91
automatic-directory-entry,::= <identifier> <generation,
idefined-directory+::= [<defined-directory-entry-list,3
Chapter 5: 'The PL/I Interpreter 143
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 162

M2l.
M22.
M23.
M24.
M25.
M26.
M27.
M28.
M29.
M30.
tf31.
eM32.
M33.
M34.
M35.
M36.
M37.
M38.
M39.
M40.
M41.
M42.
M43.
M44.
M45.
M46.
M47.
M48.
M49.
M50.
*defined-directory-entry+::= <identifier> fevaluated-data-description,
<parameter-directory>::= [*parameter-directory-entry-list,3
<parameter-directory-entry+::= <identifier>
{*undefined, 1 <established-argument,)
<established-argument,::= <generation+ f *dummy* 1 tnot-duurny+ 1
<evaluated-entry-reference,::= <entry-value+ [<established-argument-list91
*lock-control+::= <executable-unit-designator> <group-control,
*statement-control+ [<string-io-control>1
[<data-item-control-list,1 [*format-control-list91
[<current-scalar-item-list,3 [*remote-block-state,3
[*current-file-value,3
-emote-block-state>::= fblock-state-designator,
<current-file-values::= *file-value,
<group-control,::= [fcontrolled-group-state-list91
fcontrolled-group-state,::= <spec-designator> fcv-target, tcv-type>
[<by-value, <converted-by-type>1
Cfto-value, <converted-to-type91
fcv-target,::= <evaluated-target+
fcv-type>::= <data-type>
*y-value*::= *real-value* 1 <complex-value*
<converted-by-type>::= <data-type>
+to-value,::= *real-value*
*converted-to-type>::= <data-type>
<statement-control>::= <operation-list+
<string-io-control>::= 4character-string-value9 [*string-limit91 [<first-comma91
f&ring-limit+::= tinteger-value,
*first-comma>::= fan+ 1 goff9 -
<data-item-control+::= *data-list-indicator+ <data-item-indicator+
<data-list-indicator+::= *designator+
fdata-item-indicator,::= (designator9 1 <undefined,
<format-control,::= <format-specification-list-designator> <format-list-index,
Ifformat-iteration-value,3 [4format-iteration-index,]
[<format-statement-designator>1 [<remote-block-state91
<format-list-index,::= *integer-value+
*format-iteration-values::= <integer-value+
*format-iteration-index,::= (integer-value,
<current-scalar-item+::= <basic-value, <data-type> Ctdata-name-field,] 1
tevaluated-target,
<data-name-field,::= $symbol-lisq
<linkage-part+::= [<entry-point-designator>1
[<returned-value+ 1 <returned-onsource-value91
[<prolowe-flaq91
144
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 163

M51. (returned-value+::= faggregate-value,
M52. <returned-onsource-value,::= <character-string-value+
M53. &lock-environment>::= tblock-state-designator>
M54. <established-on-unit,::= *evaluated-condition+ Itentry-value, 1 <system-action>)
[<->I
M55. <evaluated-condition+::= <computational-condition> 1 <area-condition>
<evaluated-io-condition9 1 <error-condition>
<programmer-named-condition> 1 <finish-condition> 1
<storaqe-condition>
M56. *evaluated-io-condition,::= <io-condition> <file-value>
M5-7. <condition-bif-value+::= fonchar-values 1 foncode-value> 1 tonfield-value, 1
tonfile-value> 1 tonkey-value9 1 fonloc-value, 1
fonsource-value,
M58. fonchar-value+::= <integer-value,
M59. toncode-value::= <integer-value3
M60. fonfield-values::= <character-string-value+
M61. fonfile-value,::= <character-string-value>
M62. fonkey-value+::= echaracter-string-value>
M63. fonloc-value,::= <character-string-value>
M64. fonsource-value+::= *character-string-value+
M65. <copy-file+::= *file-value+
5.1.3 FILE INFORMATION
M66. tiile-information>::= <open-states ffilename,
<file-description> [<file-opening91
M67. <open-state+::= fopen3 1 <closed,
M68. <filename+::= <character-string-value+
M69. <file-opening*::= fdataset-designators <complete-file-description,
<current-position9 [<delete-flaq>l
[<allocated-buffer93 [<page-number91 Cffirst-comma+1
M70. <complete-file-description,::= <evaluated-file-description-list,
M71. <evaluated-file-description,::= <stream> I <record> I <input> 1 <output> I
<update> 1 <sequential> I <direct> I <print> [
<keyed> I <environment>)
devaluated-tab-option% I tevaluated-titles
*evaluated-linesizes I *evaluated-pagesizes'
M72. <evaluated-tab-option>::= <integer-value-list%
M73. <evaluated-title,::= <character-string-value9
M74. <evaluated-linesizeS::= *integer-value+
M75. <evaluated-pagesize,::= <integer-value+
M76. &current-positions::= tdesignator, I <undefined,
M77. <allocated-buffer,::= tgeneration, [<key91
M78. <page-number*::= <integer-value>
Chapter 5: The PL/I Interpreter 145
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 164

5.1.4 STORAGE AND VALUES
M79.
M80.
M81.
M82.
M83.
M84.
M85.
M86.
M8-7.
M88.
M89.
M90.
M91.
M92.
M93.
M94.
M95.
M96.
M97.
M98.
M99.
MlOO.
MlOl.
Ml02.
M103.
M104.
M105.
M106.
146
<allocated-storage,::= [<allocation-unit-list,3
<allocation-unit,::= *basic-value-list+
&basic-value+::= <real-value3 1 <complex-value+ 1
<character-string-value, 1 <bit-string-value+ [
<entry-value+ 1 *label-value,
<format-value> I ffile-value,
<pointer-value* I <offset-value, 1
<area-value> I <undefined>
freal-value+::= <real-number3
<complex-value,::= *complex-number,
<complex-number,::= {<real-number, I <undefined,) ffreal-number, I <undefined+)
<real-number+::=
The members of the set of real numbers are the alternative choices as immediate
and terminal components of <real-number+.
iinteger-value>::=
The members of the set of integers are the alternative choices as immediate and
terminal components of +integer-value,.
*character-string-value+::= <character-value-list, I fnull-character-strincl,
<character-value+::= fsymbol) I <undefined+
&it-string-value+::= tbit-value-list, I <null-bit-strinq3
*it-value>::= <zero-bit+ 1 <one-bit% [ <undefined+
<entry-value+::= <entry-point-designator> [iblock-state-designator>]
<label-value+::= <executable-unit-designator> [<block-state-designator>]
<format-value+::= <format-statement-designator> [<block-state-designator31
<file-value,::= ffile-information-designator3
<pointer-value+::= <generation+ I <null+
<offset-value>::= <evaluated-data-description+
*significant-allocation-list,
<storage-index-list, I f&3
<area-value+::= farea-allocation-list+ <significant-allocation-list> I *empty>
<area-allocation+::= <significant-allocation-list3 <allocation-unit+
fsignificant-allocation+::= <evaluated-data-description+ <occupancy+
<occupancy,::= <allocated+ I *freed>
<aggregate-value+::= *aggregate-type+ <basic-value-list>
<aggregate-type+::= <dimensioned-aggregate-type> 1
<structure-aggregate-type+ 1 <scalar>
<dimensioned-aggregate-type*::= <element-aggregate-type+ <bound-pair-list>
<element-aggregate-type+::= <structure-aggregate-type, I <scalars
<structure-aggregate-type+::= fmember-aggregate-type-list,
fmember-aggregate-type+::= <aggregate-type+
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 165

GENERATIONS, EVALUATED DATA DESCRIPTIONS, AND EVALUATED TARGETS 5.1.5
Mlo7.
M108.
Mlo9.
MllO.
Mill.
M112.
M113.
5.X.6
M114.
M115.
Ml16.
M117.
M118.
M119.
Ml20.
M12l.
M122.
M123.
M124.
<generations::= <evaluated-data-description+
<allocation-unit-designator+
<storage-index-list,
*evaluated-data-description+::= <data-description>
Note: <extent-expression> components of <evaluated-data-description+: <data-
description> contain only <integer-value%. (See Section 7.1.)
fstorage-index*::= <basic-value-index, [<position-indee
fbasic-value-index>::= <integer-value>
<position-index%::= <integer-value,
*evaluated-target,::= <generation+ 1 <evaluated-pseudo-variable-reference+
<evaluated-pseudo-variable-reference,::= <pseudo-variable>
[*generation> 1 *aggregate-values1
[<aggregate-values1
[<aggregate-value31
DATASET
*dataset*::= fdataset-name ffrecord-dataset, 1 <stream-dataset,}
fdataset-name>::= <character-string-value+
<record-dataset+::= <sequential-dataset,
<keyed-dataset,
<keyed-sequential-dataset,
fsequential-datasets::= <alpha, [<record-list,1 fomeqa,
&eyed-dataset,::= [<keyed-record-list%1
keyed-sequential-dataset+::= <alpha> Ifkeyed-record-list,1 fomeqa,
(keyed-record+::= irecord, <key%
eecord+::= <evaluated-data-description+ *basic-value-list+
ikey,: := <character-string-value*
<stream-dataset,::= <alpha+ 4omeua9 [<stream-item-lists]
<stream-item+::= ssymhol) 1 <linemark, 1 fpaqemarks 1 <carriage-return*
chapter 5: The PWI Interpreter 147
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 166

5.2 Terminology and Definitions
The following terms are employed at various places throughout the operations which
comprise the interpretation phase.
5.2.1 CURRENT
(1)
(2)
(3)
(4)
(5)
(6)
The last <block-state, member (if any) of the <block-state-list+ is termed the
current <block-state,.
Excepting only components of its <controlled-group-state-list+ simple component (if
any), any component of the current <block-state+ is termed current. For example,
the <executable-unit-designator> simple component of the current <block-state+ is
termed the current <executable-unit-designator>.
The last fcontrolled-group-state> member (if any) of the current fcontrolled-group-
state-lists is termed the current +controlled-group-state,.
Any component of the current tcontrolled-group-state> is termed current. For
example, the <by-value, component of the current <controlled-group-state+ is termed
the current <by-value,.
The corresponding block (see Section 5.2.2) of the current <block-state+ is termed
the current block.
MY simple component of the current block is also termed current. For example, the
<end-statement> immediate component of the current block is termed the current
<end-statement>.
5..2.2 BLOCK
The term block is used to refer to a <begin-block>, a <procedure>, or an <abstract-
external-procedure>. Each <block-state+ present in the <machine-state> is created to be
associated with some particular block. -That block is termed the correspondinq block of
the <block-state+ and it may be located since it is that block which has as block-
component the <executable-unit> designated by the <executable-unit-designator> simple
component of the <block-state> in question.
5.3 The Interpret Operation and the Initialization of the Interpretation State
5.3.1 INTERPRET
First, the <interpretation-state+ is initialized, and then the <program> is executed.
Operation: interpret(dl,ev)
where dl is a fdataset-list,,
ev is an *entry-value+.
Step 1. Perform initialize-interpretation-statetdl).
Step 2. Let eer be <evaluated-entry-reference+: ev. Perform activate-procedure(eer1.
Step 3. Perform program-epilogue.
148
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 167

5.3.2 INITIALIZE-INTERPRETATION-STATE
Initialize-interpretation-state constructs the initial configuration of the
*interpretation-state>, including certain portions that are a function of the <program>
to be interpreted.
Operation: initialize-interpretation-statetdl)
where dl is a fdataset-list+.
Step 1. Append to the <machine-state+ the tree
<interpretation-state+:
<program-state+:
<program-directory+:
<static-directory+
fcontrolled-directory+
<file-directory,;;
<allocated-storage+
dl.
Step 2. Perform build-file-directory-and-informations.
Step 3. Perform build-controlled-directory.
Step 4. Perform allocate-static-storage-and-build-static-directory.
5.3.3 BUILD-FILE-DIRECTORY-AND-INFORMATIONS
Operation: build-file-directory-and-informations
Step 1. For each <declaration>,d in the <program>, which has <named-constant> with
<file>, selected in any order:
Case 1.1. d has <external>, and there exists in the <file-directory> a ffile-
directory-entry, with both <external> and an <identifier> that is equal to
the <identifier> in d.
No action.
Case 1.2. (Otherwise).
Case 1.2.1. d does not have a <bound-pair-list>.
Perform build-fdi(d).
Case 1.2.2. d does have a <bound-pair-list>,bpl.
step 1.2.2.1.
Step 1.2.2.2.
Step 1.2.2.3.
Step 1.2.2.4.
bpl must not contain <asterisk> or <refer-option>.
Let n be the number of <bound-pair>s in bpl. For i=l,...,n, let
lbCi1 and ubCi3 be the <lower-bound> and <upper-bound>
respectively in the i'th <bound-pair> of bpl.
For i=l,...n, perform evaluate-expression-to-integer(lbCi1) to
obtain an <integer-value,,elblil and perform evaluate-expression-
to-integer(ubii3) to obtain an <integer-value+,eubiil.
For each distinct *subscript-value-list,rsubl having n
fsubscript-value% such that the i'th contained <integer-value,
lies in the inclusive range defined by elblil and eubCi1,
selected in any order, perform build-fdi(d,subl).
Chapter 5: The PWI Interpreter 149
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 168

5.3.4 BUILD-FDI
Operation: build-fdi(d,subl)
step 1.
step 2.
Step 3.
Step 4.
where d is a <declaration>,
sub1 is a Ifsubscript-value-list,3.
Let fn be a <filename whose component $symbol)s are those of the <identifier>
in d, and are taken in the same order. Let fd be the <file-description> in d.
Letinfobe a
<file-information+:
<open-state*:
fclosed,;
fn
fd.
Append info to the *file-information-list*- Let fid be a Qfile-information-
designators which designates this appended <file-information+ node.
If d has <external>, then let SC be <external>; otherwise let SC be a
<declaration-designator> designating d. Let id be the <identifier> in d. Let
fde be
<file-directory-entry+:
SC
id
fid.
If sub1 is present, append it to fde.
Append fde to the tfile-directory-entry-list+.
5.3.5 BUILD-CONTROLLED-DIRECTORY
Operation: build-controlled-directory
step 1. For each <declaratioti,d in the <program>, selected in any order:
Case 1.1. d has <controlled>.
Case 1.1.1. d has <external>, and there exists in the fcontrolled-directory, a
<controlled-directory-entry+ with both <external> and an <identifier>
that is equal to the <identifier> in d.
No action-
Case 1.1.2. (Otherwise).
Step 1.1.2.1. If d has <external>, then let SC be <external>; otherwise let SC
be a <declaration-designator> designating d. Let id be the
<identifier> in d.
step 1.1.2.2. Append, to the <controlled-directory-entry-list*, the tree
<controlled-directory-entry+: SC id.
Case 1.2. (Otherwise).
No action.
150
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 169

5.3.6 ALLOCATR-STATIC-STORAGE-ARD-BDILD-STATIC-DIRECTORY
Operation: allocate-static-storaue-and-build-static-directorv
step 1. For each <declaration>,d in the <program>. selected in any order:
Case 1.1. d has <static>.
Case 1.1.1. d has <external>, and there exists in the *static-directory+ a fstatic-
directory-entry+ with both <external> and an <identifier> that is equal
to the <identifier> in d.
No action.
Case 1.1.2. (Otherwise).
step l-1.2.1. Let dd be the <data-description> in d. Perform evaluate-data-
description-for-allocation to obtain an <evaluated-data-
description+.edd.
step 1.1.2.2. Perform allocate(edd) to obtain a fgeneration+,g.
step 1.1.2.3. If d has <initial>, then perform initialize-generation(d,g).
step 1.1.2.4. If d has <external>, then let SC be <external>; otherwise let SC
be a <declaration-designator> designating d. Let id be the
<identifier> in d.
step 1.1.2.5. Append, to the <static-directory-entry-list+, a tstatic-
directory-entry+: SC id g.
Case 1.2. (Otherwise).
No action.
5.3.7 PROGRAM-BPILOGGE
Operation: prosram-epilogue
Step 1.. Par each <file-information,,fi containing t-9 perform ClOSe(fV), where fv iS
a tfile-value9 designating fi.
Chapter 5: The PWI Interpreter 151
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 170

Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 171

Chapter 6: Flow of Control
6.0 Introduction
The definition of the control mechanism of the PL/I Interpreter. introduced in Chapter 5,
is completed in this chapter. The definition treats in order the three levels of
control, pertaining to the program, the block, and the operations within the block. This
is followed by the definition of the control of interrupt operations.
Within the execution of the program, there may be several blocks active at any time, but
execution proceeds sequentially only within the most recently activated block while the
execution of the other blocks is temporarily suspended.
6.1 Program Activation and Termination
The activation of a program is described in Section 5.3, the initialization of the
<program-state, being followed by the performance of the activate-procedure operation.
This causes the first <block-state, to be created.
A <program-state+ in general contains a list of fblock-state%., one for each block
activated within it and not yet terminated. When the execution of a program is
terminated, all the contained block activations are also terminated. The deletion of the
last remaining <block-state, results in control returning to Step 3 of the interpret
operation and performance of the program-epilogue operation (see Section 5.3.7).
6.1.1 PROGRAM TERMINATION
A program may be terminated:
(1) "abnormally', by execution of a <stop-statement>, or
(2) "normally", by execution of an <end-statement> or <return-statement>, in
circumstances which lead to the epilogue operation
original <block-state+. Since the <end-statement>
also be used for other purposes, their execution will
6.3.
being performed in the
and <return-statement> can
be described in Section
6.1.1.1 Execute-stop-statement
Operation: execute-stop-statement(ss)
where ss is a <stop-statement>.
Step 1. Perform raise-condition(<finish-condition>) -
Step 2. Perform stop-program.
6.1.1.2 Stop-proqram
Operation: stop-proqram
step 1. For each <block-state+ contained in the <program-state, except the current
&lock-state,, replace the <statement-control9 by
*statement-control+:
<operation-list,:
<operation9 for epilogue.
Step 2. Perform epilogue.
Chapter 6: Flow of Control 153
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 172

6.2 Block Activation and Termination
Block activation is des&ibed by defining first those actions which are different for
<procedure>s and <begin-block>s. The operations prologue and epilogue are the same for
both kinds of block.
6.2.1 ACTIVATE-PRCCEDDRR
A <procedure> may be activated by execution of a <call-statement>, or by evaluation of a
<value-reference> which is a <procedure-function-reference>. Also an <on-unit> has a
<procedure> which may be activated on the occurrence of an interrupt.
This operation completes when epilogue (see Section 6.2.41 is executed and eliminates the
<block-state+ and its contained operations.
operation: activate-nrocedureteer,cbifs)
where eer is an *evaluated-entry-reference*,
cbifs is a [<condition-bif-value-list,l.
step 1. Let epd be the <entry-point-designator> of eer, designating an <entry-point>,ep,
which is a simple component of a <procedure>,p.
Step 2. If there exists in the <block-state-list3 a <block-state* whose corresponding
block is p, then p must simply contain <recursive>, unless p is the inmediate
component of an <on-unit>.
Step 3. Let eud be a designator of the first <executable-unit> after ep in the <entry-
or-executable-unit-list> simply containing ep. If eer contains a fblock-state-
designator>,bsd, let ble be fblock-environment+: bsd. otherwise ble is <absent,.
If bsd is present, it must designate an existing *block-states.
Step 4. Let bs be a
<block-states:
*block-directory*:
<automatic-directory+
<defined-directory*
<parameter-directory+;
<block-control3:
<executable-unit-designator>:
eud;
<group-control+
<statement-control+:
*operation-list*:
*operation+ for instal-argumentsteer);;;
<linkage-part*:
epd.
If ble is a <block-environment+ then attach ble to bs. If cbifs is a
*condition-bif-value-list, then attach cbifs to bs.
Step 5. Append bs to the fblock-state-list3.
154
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 173

6.2.1.1 In&al-arsuments
Operation: instal-argumentsteer)
where eer is an *evaluated-entry-reference,.
Step 1. If the <parameter-name-list>,pnl of the <entry-point> designated by the <entry-
point-designator> of eer exists, then perform step 1.1.
Step 1.1. Let eal be the festablished-argument-list, of eer. Attach to the current
*parameter-directory, a *parameter-directory-entry-list%,pdel, with the same
number of immediate components as pnl and whose i'th immediate component is
<parameter-directory-entry+:
<identifier> of the i'th immediate component of pnl
<established-arguments, the i'th immediate component of eal.
Step 2. For each <identifier>,id immediately contained in a <declaration> containing
<parameter> in the current block, and not contained in pnl, append to pdel a
<parameter-directory-entry+:
id
<undefined*.
step 3. Perform prologue.
6~2.2 ACTIVATE-BEGIN-BLOCK
A <begin-block> is activated when the operation execute-executable-unit is applied to the
<executable-unit> ixanediately containing it.
Operation: activate-beqin-block
Step 1. Let eud be a designator of the first <executable-unit> of the <executable-unit>:
<begin-block>: <executable-unit-list>;; designated by the current <executable-
unit-designator>. Let bsd be a <block-state-designator* designating the current
&lock-state>. Append to the fblock-state-list> a
<block-state,:
<block-directory+:
4automatic-directory>
fdefined-directory,;
<block-control+:
<executable-unit-designator>: eud:
<group-controls
*statement-control+:
*operation-list*:
<operation+ for prologue;;;
<linkage-part+
<block-environment,:
bsd.
Chapter 6: Flow of Control -155
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 174

6.2.3 PROLOGUE
This operation is invoked at the beginning of every block activation to establish the
<automatic> and <defined> variables local to that block. The <automatic> variables are
initialized if their <declaration% specify initialization. Any <expression>s evaluated
during the prologue, such as in <extent-expression>s or <expression>s in <initial>, are
not allowed to reference other <automatic> or <defined> variables local to this block.
The operation find-directory-entry will impose the restriction when it finds a reference
to a variable declared in a block for which there exists a fproloque-flaq,. The
gproloque-flaq, is only present while the prologue operation is active.
Operation: proloque
step 1. Attach a fproloque-flaq9 to the current <linkage-part,.
step 2. For each <declaration>,d. of the current block, that contains <automatic> or
<defined>, perform Step 2.1.
Step 2.1. Let id be the <identifier> immediately contained in d, and let dd be the
<data-description> immediately contained in the <variable> of d. Perform
evaluate-data-description-for-allocation to obtain an tevaluated-data-
description9,edd.
Case 2.1.1. a contains <automatic>.
Step 2.1.1.1. Perform allocate(edd) to obtain a fgeneration,,g.
Step 2.1.1.2. Append to the current <automatic-directory-entry-list, an
<automatic-directory-entry+: id g.
Step 2.1.1.3. If d contains <initial> then perform initialize-generation(grd).
Case 2.1.2. d contains <defined>.
Append to the current <defined-directory-entry-list> a fdefined-
directory-entry>: id edd.
step 3. Delete the tproloque-flaq9 of the current <linkage-part>.
step 4. Replace the current *statement-control9 by a
*statement-control>:
*operation-list*:
<operation> for advance-execution.
6.2.4 EPILOGUE
This operation is used to terminate the execution of a block and may be invoked from
executing an <end-statement>, a <return-statement>, a <stop-statement>, or a <goto-
statement> which causes a transfer of control out of a block. This operation, which
deletes the <block-state+, normally causes a return to activate-procedure or activate-
begin-block, in the previous (block-state,.
Operation: epiloque
step 1. For each current <parameter-directory-entry+,pde, which contains *dummy,, let g
be the <generation> in pde and perform free(g).
Step 2. For each current gautomatic-directory-entry,rade. let h be the *generation> in
ade and perform free(h).
Step 3. If there is a current fonsource-value,, then attach its immediate subtree to the
fireturned-onsource-value, of the preceding <block-state+ Of the fblock-state-
lists.
Step 4. Delete the current *block-states.
156
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 175

6.3 Control within a Block
The operation normal-sequence sets the current <executable-unit-designator> to designate
the next <executable-unit>. This is normally the last action of each execution of an
<executable-unit>. The special cases of execution of a <goto-statement> and <end-
statement> may set the <executable-unit-designator> independently, while the <return-
statement> and <stop-statement> have no further need for it. The <executable-unit-
designator> is initialized in each activation of a <procedure> or <begin-block>, and may
be reset on entering a <group> or <if-statement>.
6.3.1 NORMAL-SEQUENCE
Operation: normal-sequence
Step 1. Let eu be the <executable-unit> designated by the current <executable-unit-
designator>. Let eul be the <executable-unit-list> or <entry-or-executable-
unit-list> which contains eu, but does not contain any other <executable-unit-
list> or <entry-or-executable-unit-list> which also contains eu.
Step 2. Let eu2 be that immediate component of eul which either contains eu or is
exactly eu.
Case 2.1. eul is an <executable-unit-list>.
Let eu3 be that <executable-unit> which immediately follows eu2 as an
immediate component of eul.
Case 2.2. eul is an <entry-or-executable-unit-list>.
Step 2.2-I. Let eu4 be that <entry-or-executable-unit> which immediately follows eu2
as an immediate component of eul. If eu4 immediately contains <entry-
point>, then let eu2 be eul) and go to Step 2.2.1.
Step 2.2.2. Let eu3 be the <executable-unit> immediate component of eu4.
Step 3. Set the current <executable-unit-designator> to designate eu3.
6.3.1.1 Advance-execution
This operation is the "driver" which initiates execution of each <executable-unit> as
selected by the current <executable-unit-designator>.
Operation: advance-execution
Step 1. Perform execute-executable-unit.
Step 2. Go to Step 1.
6.3.2 EXECUTE-EXECUTABLE-UNIT
The current <executable-unit-designator> designates the <executable-unit> to be executed.
Execution of an <executable-unit> consists of performing the appropriate "execute'
operation. That operation normally terminates with the current <executable-unit-
designator> designating some other <executable-unit> in the <program>. Return of control
to advance-execution then causes execute-executable-unit to be applied again.
Operation: execute-executable-unit
Step 1. Let f be the rightmost immediate component of the <executable-unit> designated
by the current <executable-unit-designator>.
Step 2. Perform execute-xxx(f), where "xxx" is replaced by the sequence of symbols
forming the name of the type of f.
Chapter 6: Flow of Control 157
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 176

6.3.3 EXECUTE-BEGIN-BLOCK
Operation: execute-begin-block(b)
where b is a <begin-block>.
Step 1. Perform activate-begin-block.
step 2. Perform normal-sequence.
6.3.4 EXECUTE-GROUP
Constraints: In a <do-spec>,dsp, let tr be tire <target-reference> component. For each
<spec> of dsp, let
e be the <expression> immediately contained in the <spec>,
b be the <expression> in the <by-option>,
te be the <expression> in the <to-option>,
r be the <expression> in the <repeat-option>,
if such options are present. The following constraints must hold for each
<spec>:
(1) If te is present then tr and e must have <computational-type> and the
derived modes of tr, e, b, and te, must all be <a>.
(21 If r is present then tr, e, and r must all have:
<computational-type>, or
<locator>, or
<non-computational-type>, with immediate subnodes of the <non-
computational-type>s belonging to the same category other than
<locator>.
(3) If tr has <pointer> and either or both of e and r has <offset>, then
each such <offset> must contain a <variable-reference>. If tr has
<offset> and either or both of e and r has <pointer>, then the
<offset> in tr must contain a <variable-reference>.
Operation: execute-group(g)
where g is a <group>.
step 1. Let feu be the first <executable-unit> simply contained in g.
Case 1.1. g has a <non-iterative-group>.
Set the current <executable-unit-designator> to designate feu.
Case 1.2. g has a <while-only-group>.
Let exp be the <expression> of the <while-option> of g. Perform establish-
truth-value(exp) to obtain t. If t is t-9, then set the current
<executable-unit-designator> to designate feu; otherwise, perform normal-
sequence.
Case 1.3. g has a <controlled-group>.
Let dsp be the <do-spec> of g. Perform establish-controlled-groupfdsp1 to
obtain t. If t is *true,, then set the current <executable-unit-designator>
to designate feu; otiiZ%se, perform normal-sequence.
158
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 177

6.3.4.1 Establish-controlled-qroup
This operation is used to set up an iteration in the cases of a <controlled-group>,
<list-directed-input>, <list-directed-output>, <edit-directed-input>, <edit-directed-
output>, and <data-directed-output>.
If the controlling <do-spec> is such as to indicate iteration, then an appropriate
<controlled-group-state+ is established and <ws is returned. If the controlling <do-
spec> indicates no iteration, then no <controlled-group-state> is established and ffalse+
is returned.
Operation: establish-controlled-qroup(dsp)
where dsp is a <do-spec>.
result: ftrue, or <false*.
Step 1. Let tr be the <target-reference> of dsp, and dt be the <data-type> of tr.
Perform evaluate-target-referencettr) to obtain an <evaluated-target+,et.
Step 2. Let sp be the first <spec> of dsp. Append to the current .&ontrolled-group-
state-list,, the tree
<controlled-group-state+:
<spec-designator>: a designator designating sp;
fcv-target,: et;
fcv-type,: dt.
step 3. Perform initialize-spec-options.
Step 4. Perform test-spec to obtain tv.
Case 4.1. tv is <trues.
Return t-3.
Case 4.2. tv is <false,.
Perform establish-next-spec to obtain tv2. If tv2 is f-3, then go to
Step 4. Otherwise, delete the current <controlled-group-state> and return
<false,.
6.3.4.2 Initialize-spec-options
Operation: initialize-spec-options
Step 1. Let sp be the <spec> designated by the current <spec-designator>. Let e be the
<expression> immediate component of sp.
Step 2. Perform steps 2.1 through 2.3 in any order.
step 2.1. Perform evaluate-expression(e) to obtain an <aggregate-value,,av.
Step 2.2. If sp contains a <to-option>,t: <expression>.et; then perform Steps 2.2.i
through 2.2.3.
step 2.2.1. Let etdt be the <data-type> of et. Let dt be a
<data-type>:
<computational-type>:
<arithmetic>:
<mode>:
<real>;
the dezd common <base> of etdt and the current tcv-type>
the derived common <scale> of etdt and the current fcv-type+
the converted <precision> of etdt.
Chapter 6: Flow of Control 159
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 178

Step 2.2.2. Perform evaluate-expression to obtain an *aggregate-value>,x. Let y
be the *basic-value, in x. Perform convert(dt,etdt,y) to obtain a
fbasic-value,: freal-value9,z. Attach a <to-value+: z; to the current
<controlled-group-state+.
Step 2.2.3. Let cp be the converted <precision> of the current fcv-type,, where dt
is used as the target <data-type> for determining cp. Replace the
<precision> tree in dt by cp. Attach a <converted-to-type+: dt; to the
current <controlled-group-state,. (The fconverted-to-type, will be used
later as the target <data-type> when the value of the &v-target+ is
converted for comparison with the <to-value+.)
Step 2.3. If sp contains a <by-option> or a <to-option> then perform Steps 2.3.1
through 2.3.5.
Step 2.3.1.
Case 2.3.1.1. sp contains a <by-option>: <expression>,eb.
Perform evaluate-expression (eb) to obtain an <aggregate-value+,x.
Let ebv be the *basic-value, in x. Let ebdt be the <data-type>
of eb.
Case 2.3.1.2. sp contains a <to-option> but not a <by-option>.
Let ebv be a *basic-value,: <real-valuep: 1. Let ebdt be a
<data-type> which is integer-type, except that its <base> has
<decimal> and its <number-of-digits> has 1.
Step 2.3.2. Let dt be a
<data-type>:
<computational-type>:
<arithmetic>:
derived common <mode> of ebdt and current tcv-type>
derived common <base> of eodt and current fcv-type+
derived common <scale> of ebdt and current fcv-type,
converted <precision> of ebdt.
Perform convert(dt,ebdt,ebv) to obtain a <basic-value>,x. Let y be the
treal-value, or tcomplex-value, immediately contained in x. Attach a
<by-value>: y; to the current <controlled-group-state>.
Step 2.3.3. Let cp be the converted <precision> of the current icv-type,, where dt
is used as the target <data-type> for determining cp. Let p be the
<number-of-digits> of cp, and let r be the <number-of-digits> of dt.
Step 2.3.4.
Case 2.3.4.1. dt has <float>.
Change the value of the <number-of-digits> of dt to max(p,r).
Case 2.3.4.2. dt has <fixed>.
Let q be the <scale-factor> of cp, and let s be the <scale-
factor> of dt. Let m=min(N,max(p-q,r-S)+max(q,s)+ll, where N is
the maximum <number-of-digits> allowed for <fixed> with the
<base> of dt. Let n=maxfq,s). Change the value of the <number-of-
digits> in dt to m, and change the value of the <scale-factor> in
dt to n.
Step 2.3.5. Attach a *converted-by-type+: dt; to the current tcontrolled-group-
state,. (The *converted-by-type, will be used later as the result
<data-type> for the addition of the <by-value, and the value of the fcv-
targets.1
Step 3. Let cvt be the immediate component of the current fcv-targets. Let dd be the
<data-description> immediate component of e. Perform assign(cvt,av,ddl.
160
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 179

6.3.4.3 Test-spec
This operation is used to test whether the current controlling <spec> in a <do-spec>
indicates continuati .on (t-9 returned) or termination (ffalse> returned).
Operation: test-spec
result: *true> or *false,.
Step 1,. If the immediate component of the current <controlled-group-state, contains a
tto-value,,y, perform Steps 1.1 through 1.3.
Step 1.1. Let cvt be the immediate component of the current fcv-target,. Perform
value-of-evaluated-target(cvt) to obtain an <aggregate-value> containing a
*basic-value9.x. Let xt be the current fcv-type,.
Step 1.2. Let dt be the current <converted-to-type>. Perform convert(dt.xt,x) to
obtain a fbasic-value>,cx.
Step 1.3. Let bv be the current &y-value,. If bv L 0 and cx > y, or if bv < 0 and
cx < y* return <false+.
Step 2. If the <spec> designated by the current tspec-designator> contains a <while-
option>: <expression>,e; then perform Step 2.1.
Step 2.1. Perform establish-truth-value(e) to obtain tv. Return tv.
Step 3. Return fz>.
6.3-4.4 Establish-next-spec
This operation is used to advance through the list of <spec>s in a <do-spec>. If there
is a next <spec> available, then conditions are established to use it and <true+ is
returned. If there is no next <spec> available, then *false+ is returned.
Operation: establish-next-spec
result: *true+ or ffalse,.
Step 1. Let sp be the <spec> designated by the current <spec-designator>. Let spl be
the <spec-list> which immediately contains sp. If sp is the last component of
spl then return <false+.
Step 2. Replace the immediate component of the current <spec-designator> by a designator
of the next <spec> component of spl.
Step 3. If the current <controlled-group-state+ contains a fby-value+ and a fconverted-
by-type, or a <to-value+ and a <converted-to-type+ then delete them.
Step 4. Perform initialize-spec-options. Return te+.
Chapter 6: Flow of Control 161
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 180

6.3.4.5 Test-termination-of-controlled-qroup
This operation is used to test for the termination of an iteration set up by establish-
controlled-group. If the controlling <do-spec> is such as to indicate termination, then
the current <controlled-group-state+ is deleted and t-3 is returned. If the
controlling <do-spec> indicates continuation, then the appropriate cexecutable-unit-
designator> or <data-item-indicator> is set to continue and <false, is returned.
Operation:
Step 1. Let
the
Case 1.1.
Case 1.2.
test-termination-of-controlled-group
result: dtrue, or tfalse,.
sp be the <spec> designated by the current <spec-designator>. Let evt be
<evaluated-target> of the current fcv-target,.
sp contains a <repeat-option>: <expression>,re. .
Let dd be the <data-description> immediate component of re. Perform
evaluate-expression to obtain an <aggregate-value+,av. Perform
assign(evt,av,dd).
sp contains a <by-option> or a <to-option>.
Step 1.2.1. Let M be the <data-type> in the current <converted-by-type+. Let cvt
he the <data-type> in the current fcv-type,. Let cvt2 be a <data-type>
that is the same as bt except for its <precision>, which is the
converted <precision> of cvt, with bt being the target <data-type> for
determining the converted <precision>.
Step 1.2.2. Let et be the *evaluated-target, in the current fcv-targee. Perform
value-of-evaluated-target to obtain an <aggregate-value?,xl. Let x2
be the tbasic-value, in xl. Perform convert(cvt2,cvt,x2) to obtain a
<basic-value+.x3. Let x be the <real-value> or <complex-value+ in x3.
Step 1.2.3. Let y be the <real-value+ or tcomplex-value, in the current <by-value>.
Perform arithmetic-result(x+y,bt) to obtain z, where z is a <real-value>
or a <complex-value>.
Step 1.2.4. Let rtd be a
<data-description>:
<item-data-description>:
<data-type>:
bt.
Let w be an
<aggregate-value+:
<aggregate-type*:
<scalar*;
<basic-value-list,:
<basic-value*:
2.
Perform assign(et,w,rtd).
Case 1.3. sp does not contain a <repeat-option>, a <to-option>, or a <by-option>.
Go to Step 3.
Step 2. Perform test-spec to obtain tv. If tv is t-9, return <false+ (which indicates
that the group does not terminate).
Step 3. Perform establish-next-spec to obtain tV2. If tv2 is <true+ then go to Step 2;
otherwise delete the current <controlled-group-state> and return <true*.
162
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 181

6.3.5 EXECUTE-IF-STATEMENT
Operation: execute-if-statement(ifs)
where ifs is an <if-statement>.
Step 1. Let e be the <expression> immediate component of the <test> of ifs. Perform
establish-truth-value(e) to obtain tv.
Step 2.
Case 2.1. tv is 4-3.
Replace the immediate component of the current <executable-unit-designator>
by a designator of the <executable-unit> of the <then-unit> of ifs.
Case 2.2. tv is *false+ and ifs simply contains an <else-unit>,eu.
Replace the immediate component of the current <executable-unit-designator>
by a designator of the <executable-unit> of eu.
Case 2.3. tv is <false9 and ifs does not simply contain an <else-unit>.
Perform normal-sequence.
6.3.5.1 Establish-truth-value
Operation: establish-truth-value(exp)
where exp is an <expression>.
result: t*> or *false*.
step 1. Perform evaluate-expression(exp1 to obtain an *aggregate-value,,av: fbasic-
value-list+: <basic-value9,sv. Let sdt be the <data-type> of exp.
Step 2. Let tdt be a
<data-type>:
<computational-type>:
<string>:
<string-type>:
<&>;
<maximum-length>:
<asterisk>.
Perform convert(tdt,sdt,sv) to obtain b.
Step 3. If b contains a <bit-value+: <one-bit+; then return *true>; otherwise return
4falses.
6.3.6 EXECUTE-CALL-STATEMENT
Operation: execute-call-statement(cs)
where cs is a <call-statement>.
step 1. Let sr be the <subroutine-reference> component of cs. Perform evaluate-entry-
referencefsr) to obtain an <evaluated-entry-reference+,eer.
Step 2. Perform activate-procedure(eer>.
Step 3. Perform normal-sequence.
Chapter 6: Flow of Control 163
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 182

6.3.6.1 Entry-references
An entry-reference - either <subroutine-reference> <procedure-function-
reference>. (<builtin-isunction-refzrence>s are described in S%A.?n 9.4.)
The main difference between a <subroutine-reference> and a <procedure-function-reference>
is that normal termination of a <procedure> in the <subroutine-reference> case is by a
<return-statement> not containing an <expression>, or by an <end-statement>, whereas in
the <procedure-function-reference> case it is by a <return-statement> containing an
<expression>.
Evaluation of an entry-reference normally takes place just before activation of a
<procedure>.
6.3.6.1.1 Evaluate-entry-reference
Operation: evaluate-entry-referencecer)
where er is a <subroutine-reference> or a <procedure-function-reference>.
result: an <evaluated-entry-reference+.
step 1. Let vr be the <value-reference> immediate component of er. Perform evaluate-
value-referencefvr) to obtain an <aggregate-value,,ag. Let ev be the fentry-
value, in ag.
step 2. Let ep be the <entry-point> designated by the <entry-point-designator> in ev.
Let id be the <identifier> in the <statement-name> in ep. Let p be the
<procedure> that block-contains ep. Let b be the abstract-block that block-
contains p. Let dl be the <declaration-list> immediately contained in b. Let d
be the <declaration> in dl whose <identifier> equals id. Let de be a copy of
the <entry> immediately contained in the <named-constant> of d, and let vre be a
copy of the <entry> of the <data-description> of vr.
Delete from de and vre all <variable-reference>s which are immediate components
of <offset>. In an implementation-defined fashion, compare corresponding
<options> components of de and vre and, if either de or vre has <options>
components and the other does not have corresponding <options> components,
compare de and vre. Delete all <options> components of de and vre.
For each <extent-expression>,ee in de or vre containing a <constant>,c, let e be
an <expression>: c; and perform evaluate-expression-to-integer(e) to obtain an
<integer-value>,iv and replace ee by <extent-expression>: iv. de and vre must
now be equal. (This checks that the entry point to be invoked agrees with the
declaration of the entry value reference, vr.)
Step 3.
Case 3.1. er does not contain an <argument-list>.
Return an <evaluated-entry-reference>: ev.
Case 3.2. er contains an <argument-list>,al.
Let n be the number of <argument% in al. Let eal be an festablished-
argument-list> with n <established-argument+ immediate components. For
i=l,...,n, taken in any order, perform step 3.2.1.
Step 3.2.1. Let arg be the i'th <argument> in al. Let dd be the <data-description>
in the i'th <parameter-descriptor> in the <parameter-descriptor-list> in
the <data-type> of vr. Perform establish-argument(arg,dd> to obtain an
<established-argument*,x. Replace the i'th *established-argument+ of
eal by x.
Step 4. Return an <evaluated-entry-reference+: ev eal.
164
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 183

6.3.6.1.2 Establish-arqument
Operation: establish-arsument(arg,dd)
where arg is an <argument>,
dd is a <data-description>.
result: an <established-argument9.
Case 1. arg does not immediately contain <dummy>.
Let vr be the <variable-reference> simple component of arg. Perform evaluate-
variable-reference(v) to obtain a fgeneration*,g. Return an testablished-
argument,: g <not-dummy+.
Case 2. arg immediately contains <dummy>>.
Step 2.1. Let e be the <expression> in arg. Perform evaluate-expression(e) to obtain
an <aggregate-value+,av.
Step 2.2. Let avdd be the <data-description> immediate component of e. Perform
promote-and-convert(dd,av,avdd) to obtain an <aggregate-value+,av2.
Step 2.3. Let cdd be a copy of dd. Replace each <bound-pair> in cdd that is not a
component of <entry> by the corresponding <bound-pair> in av2. For each
<area-size>,x, contained in cdd, let y be an *integer-value> determined by
an implementation-defined algorithm, and replace x by an
<area-size>:
<extent-expression>:
Y-
For each <data-type>,st simply contained in cdd that simply contains a tree
of the form <string>: <maximum-length>: <asterisk>;; perform Step 2.3.1.
Step 2.3.1. Let m be the maximum of the lengths of all <character-string-value% or
<bit-string-values in av2 that correspond to st. Replace the <maximum-
length> in st by a
Unaximum-length>:
<extent-expression>:
<integer-value,:
m.
Step 2.4. Perform evaluate-data-description-for-allocation(cdd) to obtain an
*evaluated-data-description,,edd. Perform allocate(edd) to obtain a
fgeneration9.g.
Step 2.5. Let bvl be the <basic-value-list+ in av2. Perform set-storagetg, bvl).
Step 2.6. Return an <established-argument?: g <dummy+-
Chapter 6: Flow of Control 165
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 184

6.3.7 EXECUTE-GOTO-STATEMENT
Operation: execute-goto-statement(gs)
where gs is a <goto-statement>.
step 1. Let vr be the <value-reference> of gs. Perform evaluate-value-referencetvrl to
obtain an *aggregate-value+,ag. ag must contain the form
<label-value+,lv:
<executable-unit-designator>,tp
<block-state-designator,,bsn.
step 2. The *block-state-list+ must contain a &lock-state,,bs, designated by bsn. (Its
corresponding block contains the <executable-unit> designated by tp.)
Case 2.1. bs is the current fblock-state,.
Perform local-goto(
Case 2.2. bs is not the current <block-state+.
Step 2.2.1. vr must not immediately contain a <data-description> whose <data-type>
has <local>.
Step 2.2.2. The <statement-control+ component of bs must not contain an <operation,
for execute-allocate-statement, execute-locate-statement, or prologue.
Step 2.2.3. Replace the <statement-control+ component of bs by:
<statement-control+:
<operation-list>:
<operation, for advance-execution
*operation9 for trim-io-control
<operation> for local-goto(
Step 2.2.4. For each <block-state+,b that occurs after bs and before the Current
<block-state, in the <block-state-list+ (of the (interpretation-state+)
perform Steps 2.2.4.1 and 2.2.4.2.
step 2.2.4.1. The *statement-control, component of b must not contain an
*operations for execute-allocate-statement, execute-locate-
statement, or prologue.
Step 2.2.4.2. Replace the <statement-control9 Of b by fstatement-control*:
*operation-list*: <operation* for epilogue.
Step 2.2.5. Replace the current <statement-controls by -statement-control*:
*operation-list+: <operation9 for epilogue.
6.3.7.1 Local-qoto
Operation: local-qoto(lv)
where Iv is a *label-values.
Step 1. Let tp be the <executable-unit-designator> in Iv. Let eu be the <executable-
unit> designated by tp. If there is an <iterative-group>,g that contains en but
does not contain an <iterative-group> or <begin-block> that contaiam eu, then
the current <executable-unit-designator> must designate an <executable-utilit>
that is contained in g-
Step 2. Perform trim-group-control(tp).
Step 3. Replace the current <executable-unit-designator> by tp#-
166
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 185

6.3.7-Z Trim-group-control
Operation: trim-group-control(eud)
where eud is an <executable-unit-designator>.
Step 1. Let eu be the <executable-unit> designated by eud. Let b be the <begin-block>
or <procedure> that block-contains eu.
Step 2.
Case 2.1. b contains a <controlled-group> that contains eu.
Let n be the number of <controlled-group>s that contain eu and are contained
in b. If the current <controlled-group-state-list, contains more than n
<controlled-group-state%, delete those after the n'th +controlled-group-
state>.
Case 2.2. (Otherwise).
If there is a current Gzontrolled-group-state-list>, delete it.
6.3.8 EXECUTE-NULL-STATEMENT
Operation: execute-null-statementtns)
where ns is a <null-statement>.
Step 1. Perform normal-sequence.
6.3..9 EXECUTE-RETURN-STATEMENT
Operation: execute-return-statement(rs)
where rs is a <return-statement>.
Step 1. Let p be the last <block-state+ of the *lock-state-list, whose corresponding
block is a <procedure>.
Case 1.1. rs does not contain an <expression>.
There must not be a <returns-descriptor> in the <entry-point> designated by
the <entry-point-designator> of p. If p is the first fblock-state, in the
<block-state-list> then perform raise-condition(<finish-condition>).
Case 1.2. rs contains an <expression>,e.
There must be a <returns-descriptor>,rd in the <entry-point> designated by
the <entry-point-designator> of p. Let dd be the <data-description>
immediate component of e. dd must be proper for assignment to rd (see
Section 4.4.2.3).
Step 1.2-l. Perform evaluate-expression(e) to obtain ev.
Step 1.2.2. Perform promote-and-convert(rd,ev,dd) to obtain an <aggregate-value+,av.
Attach <returned-value>: av; to the <linkage-part> of the fblock-state,
immediately preceding p in the fblock-state-list%.
step 2. For each iblock-states (if any) which is or which follows p in the fblock-state-
list+ except the current <block-state+, replace its <statement-control% by a
*statement-control*:
(operation-list,:
eoperation, for epilogue.
step 3. Perform epilogue.
Chapter 6: Flow of Control 167
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 186

6.3.10 EXECUTE-END-STATEMENT
Operation: execute-end-statementtes)
where es is an <end-statement>.
step 1. Let eul be that <executable-unit-list> or <entry-or-executable-unit-list, which
contains es but does not contain any other <executable-unit-list> or <entry-or-
executable-unit-list> which also contains es.
Let n be the node which immediately contains eul.
Case 1.1. n is a <procedure>,.
Step 1.1.1. The <entry-point> designated by the <entry-point-designator> component
of the current *linkage-part> must not contain a <returns-descriptor>.
Step 1.1.2. If the fblock-state-list+ contains only one fblock-state+ then perform
raise-condition(<finish-condition>).
step 1.1.3. Perform epilogue.
Case 1.2. n is a <begin-block>.
Perform epilogue.
Case 1.3. n is a <non-iterative-group>.
Step 1.3.1. set the current <executable-unit-designator> to designate the
<executable-unit> which simply contains n.
Step 1.3.2. Perform normal-sequence.
Case 1.4. n is a <while-only-group>.
Set the current <executable-unit-designator> to designate the <executable-
unit> which simply contains n.
Case 1.5. n is a <controlled-group>.
Set the current <executable-unit-designator> to designate the cexecutable-
unit> that simply contains n. Perform test-termination-of-controlled-group
to obtain t.
Case 1.5.1. t is *true.
Perform normal-sequence.
Case 1.5.2. t is <false>.
Set the current <executable-unit-designator> to designate the first
<executable-unit> of eul.
168
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 187

6.4 Conditions and Interrupts
There are two distinct concepts of "condition" and "interrupt'. When a "condition'
occurs, e.g. raise-condition(<overflow-condition>) is performed, it may lead to an
'interrupt", i.e. invocation of the interrupt operation.
The circumstances in which the various "conditions' occur are defined throughout Chapters
6 to 9 at the appropriate points, wherever the operation raise-condition is to be
performed. This section defines how the occurrence of a "condition' may also be
signalled explicitly, and how the operations raise-condition, interrupt, and system-
action, are performed.
6.4.1 CONDITIONS
6.4.1.1 Raise-condition
A condition may be 'raised" either implicitly from circumstances defined elsewhere, or
explicitly by the execution of a <signal-statement>. In either case, the operation test-
enablement is used to determine whether the "condition' is enabled and hence to determine
whether the operation interrupt is to be performed.
Operation: raise-condition(c,cbifsl
where c is an <evaluated-io-condition+, a <programmer-named-condition>. or a
terminal node of <condition-name> apart from the <named-io-
conditiotis or <programmer-named-condition>s,
cbifs is a [<condition-bif-value-list>].
step 1. There must exist at least one .?block-state,.
step 2. If c is one of the terminal nodes of a <computational-condition>, then perform
test-enablement(c) to obtain r, which must not be <disable*.
Step 3. Let cc be a copy of cbifs. If c is a <conversion-condition> and the current
&lock-control, contains a <current-file-value+: <file-value+,fv;, and cc does
not contain an fonfile-values, then attach to cc an fonfile-value,: fn; where fn
is the fcharacter-string-value+ in the <filename> in fi, where fi is the ffile-
information+ designated by fv.
step 4. Perform interrupt(c,cc).
6.4.1.2 Test-enablement
Operation: test-enablement(c)
where c is one of the terminal nodes of <computational-condition>.
result: <enabled+ or <disabled+.
step 1. Let eu be the <executable-unit> designated by the current <executable-unit-
designator>.
Case 1.1. The current <linkage-part, does not contain a <prologue-flaq+ and the
current <block-control, does not contain a <remote-block-state*.
Let tp be eu.
Case 1.2. The current <linkage-part+ does not contain a fprologue-flaqs and the
current fblock-control, contains a <remote-block-state3,rbs.
Let fc be the last *format-control, of the current *format-control-list+
that contains a *remote-block-state*. (This -emote-block-state+ equals
rbs.f Let tp be the <format-statement> designated by the <format-statement-
designator> of fc.
Chapter 6: Plow of Control 169
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 188

Case 1.3. The current <linkage-part+ contains a fproloque-flaq>.
Let tp be the <procedure> or <begin-block> of which eu is a block-component.
step 2. If tp is a <begin-block> then let tp be the <executable-unit> inunediately
containing tp.
step 3. If tp immediately contains a <condition-prefix-list>,cpl,,and if cpl contains a
<condition-prefix>,cp, containing a <computational-condition> equal to
<computational-condition>: c;, then return the second component of cp.
step 4.
Case 4.1. There exists a block, b, which has tp as block-component.
Let tp be b and go to Step 2. .
Case 4.2. There is no such block.
If c is <size-condition>, Cstrinqranqe-condition>, or <subscriptranqe-
condition>, then return <disable&; otherwise return *enabled,.
6.4.1.3 Execute-siqnal-statement
operation: execute-siqnal-statement(ss1
where ss is a <signal-statement>.
step 1. Let cn be the <condition-name> immediate compohent of ss and let c be the
<named-io-condition> or <programmer-named-condition> or otherwise the terminal
subnode, of cn. If c is not one of the terminal nodes of <computational-
condition>, then let r be <enabled,. Otherwise perform test-enablement(c) to
obtain r.
Case 1.1. r is *disabled,.
Perform normal-sequence and terminate this operation.
Case 1.2. r is tenable-.
Case 1.2.1. c is <conversion-condition>.
Let cbifs be a
<condition-bif-value-list+:
<condition-bif-value,:
<onsource-value,:
fcharacter-string-valueJ:
tnull-character-strinqs;;;
<condition-bif-value,:
+onchar-value,:
ginteger-values: 0.
Case 1.2.2. c contains <name-condition>.
Let cbifs be a
<condition-bif-value-list,:
<condition-bif-value,:
tonfield-value,:
(character-string-values:
&null-character-strinqs.
170
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 189

Case 1.2.3. c contains <key-condition>.
Let cbifs be a
<condition-bif-value-list>:
<condition-bif-value,:
fonkey-value,:
<character-string-value%:
tnull-character-strinqs.
Case 1.2.4. (Otherwise).
Let &ifs be <absent,.
step 2. If c is a <named-io-condition> then perform evaluate-named-io-condition(c) to
obtain ec; otherwise let ec be c.
step 3. Perform interrupt(ec.cbifsl.
step 4. Perform normal-sequence.
6.4.1.4 Evaluate-named-io-condition
Operation: evaluate-named-io-condition(nioc)
where nioc is a <named-io-condition>.
result: an <evaluated-io-condition>.
Step 1. Let vr be the <value-reference> immediate component of nioc. Perform evaluate-
file-option to obtain a *file-value+,f.
step 2. Let ioc be the <io-condition> component ot nioc. Return tevaluated-io-
conditioe: ioc f.
6.4.2 INTERRUPTS
The <on-statement> and <revert-statement> may be used to influence the action taken on
the occurrence of an interrupt operation. First these statements are described, and then
the operation interrupt itself is defined.
6.4.2.1 Execute-on-statement
Operation: execute-on-statement(os)
where OS is an <on-statement>.
step 1. For each <condition-name>.cn in the <condition-name-list> component of OS taken
in left-to-right order perform Steps 1.1 through 1.5.
Step 1.1.
Case 1.1.1. cn has <named-io-condition>,nic.
Perform evaluate-named-io-condition(nic) to obtain an fevaluated-io-
condition*,eic. Let ec be an <evaluated-condition%: eic.
Case l-1.2. cn does not have <named-io-condition>.
Let cnl be the immediate subtree of cn. Let ec be an fevaluated-
condition,: cnl.
Step 1.2. If the current <established-on-unit-list+ contains an festablished-on-
unit+,eou containing ec, then delete eou.
Chapter 6: Flow of Control 171
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 190

Step 1.3.
Case 1.3-l. 0s contains an <on-unit>,ou.
Let epd be an <entry-point-designator> designating the <entry-point> of
ou. Let es be an
*entry-value+:
epd &lock-state-designator9 designating the current *block-state,.
Case 1.3.2. (Otherwise).
Let es be <system-action>.
Step 1.4. Let neou be
<established-on-unit+:
ec
es.
Step 1.5. If OS contains <B>, attach <snap> to neou. Append neou to the current
<established-on-unit-list+.
Step 2. Perform normal-sequence.
6.4.2.2 Execute-revert-statement
Operation: execute-revert-statementtrs)
where rs is a <revert-statement>.
Step 1. Let cnl be the <condition-name-list> immediate component of rs. For each
<condition-name>,c in cnl taken in left-to-right order perform Steps 1.1 and
1.2.
Step 1.1.
Case 1.1.1. c has <named-io-condition>,nic.
Perform evaluate-named-io-condition(nic) to obtain an fevaluated-io-
condition+,eioc. Let ec be an <evaluated-condition>: eioc.
Case 1.1.2. c does not have a <named-io-condition>.
Let ec be an *evaluated-condition+: the immediate component of c.
Step 1.2. If the current <established-on-unit-list, contains an <established-on-unit>,
eou containing ec, then delete eou.
Step 2. Perform normal-sequence.
6.4.3 INTERRUPT
Operation: interrupt(c,cbifs)
where c is an <evaluated-io-condition+, a <programmer-named-condition>, or a
terminal node of <condition-name> apart from the <named-io-
condition>s or <programmer-named-condition>s,
cbifs is a [<condition-bif-value-lists].
Step 1. Let bs be the current *block-state,.
172
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 191

Step 2. Append to cbifs each of the following components whose immediate subnode it does
not already possess (or let cbifs be a <condition-bif-value-list> with these
components if cbifs is fabsent>):
<condition-bif-value,:
foncode-value,:
<integer-values: an implementation-defined integer;;;
<condition-bif-value+:
fonloc-value,:
the <character-string-value> formed by finding the last
gblock-state, of the <block-state-list+ which has an
<entry-point-designator>, and taking the <identifier>
of the <statement-name> of the <entry-point> designated
by it:;
and if c is an <evaluated-io-condition, and cbifs does not contain an
fonfile-value+:
<condition-bif-value,:
fonfile-value,:
the <character-string-value9 in the *filename> of the
<file-information+ designated by the <file-value, of c.
Step 3. Let eoul be the *established-on-unit-list+ of bs.
Case 3.1. eoul contains an <established-on-unit+,eou, which contains an fevaluated-
condition+ with a subtree equal to c, or c is a <programmer-named-condition>
such that the following conditions are true:
(1) c designates a <declaration>.ad containing <external>, and
(2) eoul contains an <established-on-unit+,eou which contains a <programmer-
named-condition> designating a <declaration> equal to ad.
Step 3.1.1. If eou contains <snap>, then output implementation-defined information
(e.g. a list of names of currently active blocks) by an implementation-
dependent means.
Step 3.1.2.
Case 3.1.2.1. eou contains an fentry-value9,ev.
Let eer be an <evaluated-entry-reference>: ev. Perform activate-
procedure(eer,cbifs).
Case 3.1.2.2. eou contains <system-action>.
Perform system-action(c,cbifs).
Case 3.2. (Otherwise).
Case 3.2.1. bs is not the first Glock-state, of the <block-state-list+,bsl.
Let bs be the immediately preceding -lock-state, of bsl and go to step
3.
case 3.2.2. (Otherwise).
Perform system-action(c,cbifs).
step 4. c must not be <error-condition>, <fixedoverflow-condition>, <overflow-
condition>, <size-condition>, Cstringranqe-condition>, <subscriptranqe-
condition>, or <zerodivide-condition>.
Chapter 6: Flow of Control I73
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 192

6.4.4 SYSTEM-ACTION
POY every <evaluated-condition*, there is the possibility, as defined in Section 6.4.3,
that an interrupt operation for it will lead to the operation system-action.
Operation: system-action(c,cbifs)
Case 1.
Case 2.
Case 3.
Case 4.
Case 5.
where c is an <evaluated-io-condition+, a <programmer-named-condition>, or a
terminal node of a <condition-name> apart from the <named-
io-condition>s or <programmer-named-condition>s,
cbifs is a *condition-bif-value-list>.
c is <finish-condition> or <strinqsize-condition>.
Terminate this operation.
C is <programmer-named-condition> or <underflow-condition>, or contains <name-
condition>.
Perform comment.
c contains <endpaqe-condition>.
Let fv be the <file-value, component of c. Perform put-page(fv) (see Section
8.7.2.12).
c is <error-condition> or <storaqe-condition>.
The action is implementation-defined.
(Otherwise).
Step 5.1. Perform comment.
Step 5.2. Perform raise-condition(<error-condition>,cbifs).
6.4.4.1 Comment
Operation: comment
output implementation-defined information by an implementation-dependent
means.
174
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 193

Chapter 7: Storage and Assignment
7.0 Introduction
This chapter defines all the operations of the PL/I Machine that change the Callocated-
storage, of the fmachine-state,. The main Sections are:
7.1 The Generation
7.2 The Allocation of Storage
7.3 Initialization
7.4 The Freeing of Storage
7.5 Assignment
7.6 Variable-reference
7.7 Reference to Named Constant
The <allocated-storage, consists of an <allocation-unit-list+. Elements are appended to
this list by the allocate operation; this may be invoked either during the execution of
an <allocate-statement>, <read-statement>, or <locate-statement>, or directly by any of
the operations allocate-static-storage-and-build-static-directo~, establish-argument, or
prologue.. An fallocation-unit, is deleted from the list by the free operation: this may
be invoked either during the execution of a <free-statement>, <read-statement>, or
<locate-statement>, or directly by the epilogue operation.
An <area-value* also contains an <allocation-unit-list,; elements may be added to this
list by the suballocate operation during the execution of an <allocate-statement> and may
be deleted by the free-based-storage operation during the execution of a <free-
statement>.
Elements of the <basic-value-list+ of an <allocation-unit, are changed by the set-storage
operation; this may be invoked during the execution of the <assignment-statement>, the
<group>, the <read-statement>, the <get-statement>, or during initialization.
7.1 The Generation
The evaluation of a <variable-reference> yields a <generation+; a *pointer-value9 is also
a <generation+. A <generation+ describes some or all of the elements of the tbasic-
value-list+ of the <allocation-unit9 designated by the <allocation-unit-designator+ of
the <generation+. The elements of the istorage-index-list, component of the <generation+
specify which elements of the *basic-value-list, of the iallocation-unit+ are being
described. Each such element is a scalar-element.
The <evaluated-data-description+ component of a <generation, contains a <data-
description> where each <extent-expression> contains only an <integer-value+.
7.1.1 THE NUMBER OF ELEMENTS IN THE STORAGE-INDEX-LIST OF A GENERATION
Let the <evaluated-data-description+ of the <generation,,g, have the immediate <data-
description> component dd. Because dd is contained in an <evaluated-data-description+,
each <bound-pair> of dd has two <extent-expression>s which contain only <integer-value%;
that is, no <bound-pair> contains an <asterisk> or <expression> with a <variable-
reference>. The number of elements of the <storage-index-list, of g is the number of
scalar-elements corresponding to dd. This number is determined by the operation scalar-
elements-of-data-description.
Chapter 7: Storage and Assignment 175
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 194

Operation: scalar-elements-of-data-description(dd)
where dd is a <data-description>,.
result: an integer.
Case 1. dd is of the form <data-description>: <item-data-description>.
Return the integer 1.
f!ase 2. dd is of the form <data-description>: <structure-data-description>: Qnember-
description-list>,mdd.
For each tree of the form <member-description>: <data-description>,ddltil; in
mdd, i=l,...,m, perform scalar-elements-of-data-description(ddltil1 to obtain an
integer, nCi1. Return the integer
m
c nCi3.
i=l
Case 3. dd is of the form <data-description>:
<dimensioned-data-description>:
<element-data-description>,edd
<bound-pair-list>,bpl.
Let ic be the immediate component of edd. Let ddl be a <data-description>: ic.
Perform scalar-elements-of-data-description(dd1) to obtain the integer n. For
each tree of the form <bound-pair>,bpCif in bpl, i=l...lrm, let ublil and lb[il
be the <integer-value components of the <upper-bound> and <lower-bound>
respectively of bplil. Return the integer
m
n * n (ubCil-lbCil+l).
i=l
7.1.2 CORRESPONDENCE BETWEEN AN ITEM-DATA-DESCRIPTION AND A BASIC-VALUE
There is a correspondence between an element of the <storage-index-list% of a
*generation+ and a *basic-value+ in the +.llocated-storage,. There is also a
correspondence between a <storage-index+ and an <item-data-description> of the
*evaluated-data-description, of the <generation+. In general. this is a many-to-one
correspondence defined by the operation find-item-data-description which finds the <item-
data-description> of a <data-description> that corresponds to a given element of the
<storage-index-list,.
Operation: find-item-data-description(dd,ord>
where dd is a <data-description>,
ord is the ordinal of an element of a <storage-index-list+.
result: an <item-data-description>.
Case 1. The immediate component of dd is an <item-data-description>.
Return this <item-data-description>.
Case 2. The immediate component of dd is a <structure-data-description>.
Let sdd be this <structure-data-description>. Let mdl be the Unember-
description-list> of sdd. Let ntil be the number of scalar-elements
corresponding to the i'th element of mdl, obtained by performing scalar-
elements-of-data-description(ddIi1) where ddiil is the <data-description> of the
i'th element of mdl. Let smCO1 be 0. Let smCi1 be the sum of the nCj1 for the
first i elements of mdl. Let j be such that sm[j-lJ<ordlsm[jl. Perform find-
item-data-description(dd[jl,k), where k = ord-smij-11, to obtain an <item-data-
description>,idd. Return idd.
176
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 195

Case 3. The immediate component of dd is a <dimensioned-data-description>.
Let ddd be this <dimensioned-data-description>.
Case 3.1. The <element-data-description> of ddd has an <item-data-description> as the
immediate component.
Return a copy of this <item-data-description>.
Case 3.2. (Otherwise).
Let ddl be a <data-description> immediately containing a copy of the
<structure-data-description> of add. Perform scalar-elements-of-data-
description(dd1) to obtain an integer n. Perform find-item-data-
description(ddl,k+l). where k is the value of the remainder obtained when
0ra-l is divided by n, to obtain an <item-data-description>,idd. Return
iaa.
7.1.3 VALUE OF A GENERATION
Operation: value-of-qenerationtg)
where g is a <generation+.
result: an <aggregate-value+.
step 1. The <allocation-unit+ designated by the fallocation-unit-designator+ of g must
be contained in fallocated-storage+.
Step 2. Let av be an <aggregate-value+.
Step 3. Let ad be the <data-description> of g, with associated faggregate-type+,agt.
Attach agt to av.
Step 4. Let n be the number of elements in the <storage-index-list, of g, and let v be
the <basic-value-list+ of the <allocation-unit+ designated by the fallocation-
unit-designator+ of g.
step 5. For i=lr...,n. perform Steps 5.1 through 5.3.
Step 5.1. Perform find-item-data-description(dd,i) to obtain an <item-data-
description>,id. Let d be the <data-type> of id.
Step 5.2. Let p be the i'th <storage-index, immediately contained in g.
Step 5.3. Perform value-of-storage-index(p,d,v) to obtain a *basic-value+,bv. Append
bv to the *basic-value-list, of av.
Step 6. Return av.
Chapter 7: Storage and Assignment 177
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 196

7.1.4 VALUE OF STORAGE INDEX
Operation: value-of-storaqe-index(p,d,vl
where p is a *storage-index,,
d is a <data-type>,
v is a *basic-value-list+.
result: a *basic-value,.
step 1. Let i be the <basic-value-index+ of p.
step 2. Let vCi1 be the immediate component of the i'th *basic-value, of v.
Case 2.1. d does not have <string>: <nonvarvinq>; or <pictured>. .
Let e be a copy of v[il.
Case 2.2. d has <string>: <nonvarvinq>; or <pictured>.
Let N be the <maximum-length> component of d (if d has <string>) or the
associated character-string length of d (if d has <pictured>).
Case 2.2.1. N = 0.
Let e be the <null-character-strinq, (if d has <character> or
<pictured>) or the <null-bit-strinq, (if d has <bit>).
Case 2.2.2. N + 0.
Let M be the <position-index> component of p. Then there are integers j
and k such that:
(1) i 5 j 5 k (i=j=k is possible),
(2) v[il, v[i+ll,..., vtkl are either all <bit-string-valuess or all
<character-string-value+s.
j-l j
(3) c LIsl < M 5 c L[sl, and
s=i s=i
k
(4) M+N-1 I c LCSI.
s=i
Here LCxl is the length of vlxl. Let e be a <character-string-value+
(if vCi1 is a <character-string-value,) or a <bit-string-value+ Eif vCi1
is a fbit-string-value+). The length of e is N. The N components of e
are the same N successive components of vtjl, vtj+ll, . . . . vlkl
beginning at component number (M-Ltil-LLi+ll-...-LIj-11) in v[jl.
step 3. Return a <basic-value> containing a copy of e.
178
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 197

DECLARE 1 A AUTOMATIC UNALIGNED,
2 B CHARACTER(3).
2 C CHARACTER(S),
2 D CHARACTER(3).
X CHARACTER(4) DEFINED A POSITION(Z),
Y CHARACTER(61 DEFINED A POSITION(6);
A.B = '123':
A.C = '45678';
After the assignments, the fgenerations corresponding to A looks like
1112131 1415i6l7181 I I I I L---------J L--,---------------J L--------J
VCll vt23 vc33
Here v[il is the i'th component of the <basic-value-list+, a digit represents the
corresponding character, and an empty slot represents <undefined,. Evaluation of a
<variable-reference>,X yields a <scalar+ <generations accessing the <generation> of
A as follows.
1112131 l4151617181 I I I I L----,------l L-------------------J L--,-----J
I I L------------------J
X
Hence a <value-reference>,X yields '2345'. Evaluation of a <variable-reference> Y
yields a <scalar+ <generation, accessing the *generation> of A as follows.
1112131 1’+1~161718/ I I I I L-----------J L------------------J L--------J
I I I.,----------------e------J
Y
The value of the <generation> corresponding to Y has *undefined+ as its fourth,
fifth, and sixth components. Therefore, evaluation of a <value-reference>,Y at
this point would be in error.
Example 7.1. An Example of tgeneration% and *basic-value>s of Defined Variables.
Chapter 7: Storage and Assignment 179
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 198

7.2 The Allocation of Storage
7.2.1 EXECUTE-ALLOCATE-STATEMENT
This operation causes the construction of an <allocation-unit+ corresponding to the
<declaration> designated by the <declaration-designator>. Under certain circumstances,
the <storaqe-condition> or the <area-condition> may be raised.
Operation: execute-allocate-statement(ast1
where ast is an <allocate-statement>.
Step 1. For each <allocation>,al, in the <allocation-list> of ast, chosen in left-to-
right order, perform Steps 1.1 through 1.3.
Step 1.1. Let d be the <declaration> designated by the <declaration-designator> of al.
Step 1.2.
Case 1.2.1. The <storage-class> of d contains <controlled>.
Perform allocate-controlled-storagetal) to obtain the tgeneration+,g.
Case 1.2.2. The <storage-class> of d contains <based>.
Perform allocate-based-storage(a1) to obtain the fgeneration+,g.
Step 1.3. If d has an <initial> component, perform initialize-generation(g,d).
Step 2. Perform normal-sequence.
7.2.2 ALLOCATE-CONTROLLED-STORAGE
This operation causes the allocation of storage for a <controlled> variable and records
the allocation in the <controlled-directory+.
Operation: allocate-controlled-storaqe(al)
where al is an <allocation>.
result: a <generation>.
Step 1. Let d be the <declaration> designated by the <declaration-designator>,dp, of al.
Make a copy, dd, of the <data-description> of d.
Step 2. Perform evaluate-data-description-for-allocation to obtain the fevaluated-
data-description,,edd.
step 3. Perform allocate(edd) to obtain the fgeneration9.g.
Step 4. Perform find-directory-entry to obtain the fcontrolled-directory-entry+,e,
for the <declaration>.d.
Step 5. Append g to the <generation-list+ component of e.
Step 6. Return a copy of g.
180
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 199

7.2.3 ALLOCATE-BASED-STORAGE
This operation causes the allocation of storage for a <based> variable and the assignment
of the resulting <generation+ to a <locator> variable.
Operation: allocate-based-storaqe(al1
where al is an <allocation>.
result: a *generation+.
Step 1. Let d be the <declaration> designated by the <declaration-designator> of al.
Step 2.
Case 2.1. al has no <set-option> component.
The <based> component of d must have the component <value-reference> that
immediately contains a <variable-reference>. Let vrs be this <variable-
reference>.
Case 2.2. al has a <set-option> component.
Let vrs be the <variable-reference> of the <set-option> of al.
Step 3. Perform evaluate-variable-reference(vrs) to obtain a fgeneration+,gs.
Step 4. Perform evaluate-in-option(al,vrs). If the allocation is to be made in an area,
a <generations with an <area> component will be obtained; let this be gi:
otherwise the value <fail> will be obtained.
Xep 5. Make a copy, dd, of the <data-description> immediately contained in the
<variable> of d. Perform evaluate-data-description-for-allocation to obtain
an <evaluated-data-description,,edd.
ltep 6.
Case 6.1. A fgeneration>,gi, was obtained in Step 4.
Make an allocation for dd in the area fgeneration+,gi, by performing
suballocate(edd.gi). The result obtained will be either the fgeneration+,g,
or the value - <fail,. In the latter case, perform raise-
condition(<area-condition>); on normal return go to Step 4.
Case 6.2. <fail, was obtained in Step 4.
Make an allocation for dd in the <allocated-storage> by performing
allocate(edd) to obtain the fgeneration>,g.
Step 7. Perform Steps 7.1 and 7.2 in either order.
Step 7.1. Let av be an
*aggregate-value+:
<aggregate-type,:
<scalar+;
*basic-value-list*:
<basic-value>:
*pointer-value,:
g-
Let ddp be of the form <data-description>: <item-data-description>:
<pointer>. Let egs be <evaluated-target+: gs. Perform assign(egs,av,ddp).
Step 7.2. Perform initialize-refer-options(g) to carry out the initializations of g
specified by each <refer-option> in the <evaluated-data-descriptions of g.
Step 8. Return g.
Chapter 7: Storage and Assignment 181
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 200

7.2.4 EVALUATE-IN-OPTION
If an allocation is to be made in an area, this operation yields the area <generation+ in
which the allocation is to be made.
Operation: evaluate-in-option(al,vr1
where al is an <allocation>,
vr is a <variable-reference>.
result: a <generation? or <fail+.
Step 1. Let ds be the <declaration> designated by the <declaration-designator> component
of vr. ds is the <declaration> for the locator that will be used to identify the
<allocation-unit+ that will result from the <allocation>,al.
Step 2. If the component <locator> of ds has <offset> and this has the subnode
<variable-reference>, then let this <variable-reference> be vro.
Step 3.
Case 3.1. al has an <in-option> component.
Let the <variable-reference> of the <in-option> be vri.
Case 3.2. al has no <in-option> component.
Case 3.2.1. ds has an <offset> component.
vro must have been created in Step 2. That is, the area base for the
offset must have been specified. Let vri be the same as vro.
Case 3.2.2. ds has a <pointer> component.
Return the value <fail>, since the <allocation-unit, for al is to be
constructed in the *allocated-storage%.
Step 4. Perform evaluate-variable-referencetvri) to obtain the tgeneration+,gia.
Step 5. If vro has been created in Step 2, then perform evaluate-variable-reference(vro)
to obtain the fgeneration+,gib. gia must be the same as gib.
Step 6. Return gia.
7.2.5 ALLOCATE
This operation adds an <allocation-unit+ that is described by an fevaluated-data-
description+ to the iallocated-storage,. The result of the operation is the <generation+
that identifies the new fallocation-unit,.
Operation: allocate(edd)
where edd is an <evaluated-data-description+.
result: a *generation>.
Step 1. Perform either Step 1.1 or Step 1.2.
Step 1.1. Perform raise-condition(<storaqe-condition>). Go to step 1.
Step 1.2. Perform make-allocation-unittedd) to obtain an iallocation-unit+,au.
Step 2. Append au to *allocated-storage,.
Step 3. Perform scalar-elements-of-data-description(dd), to obtain n, where dd is the
<data-description> of edd. Let i be 1. Construct a *storage-index-list+,sil,
by performing Steps 3.1 through 3.3 n times.
182
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 201

Step 3.1. Perform find-item-data-description[dd.i) to obtain an <item-data-
description>,idd.
Step 3.2.
Case 3.2.1. idd contains either a <pictured> component or a <string> component that
has <nonvarvinq>.
Construct a <storage-index+,si, that contains a fbasic-value-index9 that
has an <integer-value> equal to i and also a *position-index+ that has
an <integer-value+ equal to 1.
Case 3.2.2. (Otherwise).
Construct a fstorage-index>,si, that contains a <basic-value-index+ that
has an <integer-value> equal to i.
Step 3.3. Append si to sil and set i to i+l.
Step 4. Return the <generation> constructed from an <allocation-unit-designator+
designating au, edd. and sil.
7.2.6 SUBALLOCATE
This operation constructs an farea-allocation, and adds it to the <area-allocation-list,
of an area <generation>. The result of this operation is the fgeneration+ that
identifies the new iarea-allocation+.
Operation: suballocate(edd,g)
where edd is an <evaluated-data-description+,
g is a egeneration,.
result: a *generation> or *u*-
Step 1. The value of g must not be <undefined+.
Step 2. Perform either Step 2.1 or Step 2.2.
step 2.1. Return the value *fail>.
Step 2.2. Perform make-allocation-unittedd) to obtain an <allocation-unit+,au.
Step 3. Perform value-of-generation(g) to obtain an <aggregate-value+,aqv wnose
<aggregate-type> must immediately contain <scalar+ and whose <basic-value-list>
contains an farea-value>,av. If av immediately contains <empty> then replace av
by
farea-value?=,av:
farea-allocation-list+
<significant-allocation-list+.
Append a
<significant-allocation>:
edd
<occupancy>:
tallocated,;;
to the <significant-allocation-list+,sal immediate component ot av. Append an
<area-allocation+: sal au; to the <area-allocation-list+ of av.
Chapter 7: Storage and Assignment 183
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 202

Step 4. Perform scalar-elements-of-data-description(dd), to obtain n, where dd is the
<data-description> of edd. Let i be 1. Construct a <storage-index-list+,sil,
by performing Steps 4.1 through 4.3 n times.
Step 4.1. Perform find-item-data-description(dd,i) to obtain an <item-data-
description>,idd.
Step 4.2.
Case 4.2.1. idd contains either a <pictured> component or a <string> component that
has <nonvaryinq>.
Construct a <storage-index9,s.i. that contains a <basic-value-index+ that
has an iinteger-value9 equal to i and also a <position-index> that has
an <integer-value+ equal to 1.
Case 4.2.2. (Otherwise).
Construct a fstorage-index+,si, that contains a *basic-value-index+ that
has an <integer-value, equal to i.
Step 4.3. Append si to sil and set i to i+l.
Step 5. Return the *generation> constructed rrom an <allocation-unit-designator+ that
designates au, edd, and sil.
7.2.7 EVALUATE-DATA-DESCRIPTION-FOR-ALLOCATION
This operation forms an <evaluated-data-description9 from a <data-description> by
evaluating each <extent-expression>.
Operation: evaluate-data-description-for-allocation
where dd is a <data-description>.
result: an <evaluated-data-description+.
Step 1. Let cdd be a copy of the <data-description>,dd. For each <extent-expression> in
cdd that immediately contains an <expression>, perform Step 1.1, with the
<extent-expression>s chosen in any order.
Step 1.1. Let the chosen <extent-expression> be ee. Evaluate ee by performing
evaluate-expression-to-integer(e) where e is the <expression> of ee, to
obtain the value i. The <expression> of the <extent-expression> is replaced
by an <integer-value+ with value i.
step 2. In each <bound-pair> of cdd, the value of the <integer-value> of the <upper-
bound> must be greater than or equal to the value of the <integer-value> of the
<lower-bound>.
step 3. In each <maximum-length> component of cdd, the value of the finteger-value, must
be greater than or equal to zero.
step 4. In each <area-size> component of cdd, the value of the <integer-value+ must be
greater than or equal to zero.
Step 5. For each <refer-option>,ro, in cdd perform step 5.1.
Step 5.1. Let mo be the <member-description> in cdd that ro references as in Step 2.2
of validate-based-declaration. Let n be an <integer-value+ such that the
result of find-item-data-description(cdd,n) is equal to the Gtem-data-
description> in mo. Replace the <identifier-list> component of ro by n.
step 6. Return the <evaluated-data-description+: cdd.
184
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 203

7.2.8 FIND-DIRECTORY-ENTRY
This operation searches the appropriate fmachine-states directory for an entry
corresponding to a declaration.
Operation: find-directory-entry
where dp is a <declaration-designator> designating a <declaration>,d.
result: a <static-directory-entry3, or a <controlled-directory-entry+, or an
<automatic-directory-entry+, or a *parameter-directory-entry%=, or
a <defined-directory-entry+.
Step 1. Let st be the <storage-type> of d.
Step 2.
Case 2.1. st contains <automatic>, <parameter>, or <defined>.
Perform find-block-state-of-declaration(dp) to obtain a <block-state+,bs.
If st contains <automatic> or <defined> then bs must not contain a
fproloque-flaq+ in its <linkage-part+. Find the directory entry, e, in the
<automatic-directory+, <parameter-directory+, or *defined-directory+ of bs,
as appropriate, such that the <identifier> of e is equal to the <identifier>
of d.
Case 2.2. st contains <static> or <controlled>.
Case 2.2.1. The <scope> of d has an <external> component.
Search the <static-directory+ or the fcontrolled-directory+, as
appropriate, for the directory entry e whose <identifier> is equal to
the <identifier> of d and that has an <external> component.
Case 2.2.2. The scope of d has an <internal> component.
Let e be the <static-directory-entry+ or <controlled-directory-entrys,
as appropriate, which contains a <declaration-designator> equal to dp.
step 3. Return e.
7.2.9 V-E-ALLOCATION-UNIT
This operation forms an <allocation-unit+ corresponding to an <evaluated-data-
description+.
Operation: make-allocation-unittedd)
where edd is an <evaluated-data-description+.
result: an <allocation-unit*.
step 1. Let dd be the simply contained <data-description> component of edd. Perform
scalar-elements-of-data-description(dd) to obtain n, the number of scalar-
elements that correspond to edd. Construct a *basic-value-list+,svl, by
performing Steps 1.1 through 1.2 for i=l,--.,n-
Step 1.1. Perform find-item-data-description(dd,il to obtain the <item-data-
description>,idd, that corresponds to the i'th scalar-element of edd.
step 1.2.
Case 1.2-l. The <mode> of idd contains <complex> and idd does not contain
<pictured>-
Append to svl a <complex-value+ consisting of two fundefined,
components.
Chapter 7: Storage and Assignment 185
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 204

Case 1.2.2. idd contains <pictured>.
Let m be the associated character-string length of the <pictured>
component of idd. Append to svl a <character-value-list, consisting of
m elements, each of which is gundefined%.
Case 1.2.3. idd contains <string> containing <nonvaryinq>.
Let m be the *integer-value, contained by the Unaximum-length> component
of <string>.
Case 1.2.3.1. The <string> of idd contains <character>.
If m is zero, append a *character-string-value, consisting of the
<null-character-strinq+ to svl; otherwise append a fcharacter-
string-value> consisting of a <character-value-list, with m
elements containing <undefined,.
Case 1.2.3.2. The <string> of idd contains <bit>.
If m is zero, append a *it-string-value, consisting of the
tnull-bit-strinq+ to svl; otherwise append a ait-string-value+
consisting of a <bit-value-list> with m elements containing
<undefined>.
Case 1.2.4. idd contains an <area> component.
Append an <area-value+: <empty+; to svl.
Case 1.2.5. (Otherwise).
Append an fundefined, element to svl.
Step 2. Return an fallocation-unit, containing svl.
7.2.10 INITIALIZE-REFER-OPTIONS
This operation initializes the object of each <refer-option> in a <generation%
Operation: initialize-refer-options(g)
where g is a <generation+ whose <aggregate-type> immediately contains
*structure-aggregate-type>.
Step 1. Let edd be the <evaluated-data-descriptions of g and let dd be the <data-
description> in edd. For each <refer-option>,ro, of edd, chosen in any order,
perform Steps 1.1 and 1.2.
Step 1.1.
Step 1.2.
The <refer-option>,ro, has an *integer-value> index. i, constructed by
evaluate-data-description-for-allocation; this identifies the element of g
that is the object of the <refer-option>,ro. Perform find-item-data-
description(dd,i) to obtain an <item-data-description>,idd, of the element.
Let eddr be an <evaluated-data-description, with idd as immediate component.
Construct the fgeneration~,gr, from eddr, a copy of the fallocation-unit-
designator+ of g and the <storage-index-list+ containing a single element, a
copy of the i’th element of the <storage-index-list+ of g.
ro is a component of an <extent-expression> that has an *integer-value>,iv.
Let ddi be a <data-description> with integer-type (see Section 9.1.2). Let
av be of the form
<aggregate-value3:
*aggregate-type,:
<scalar>;
*basic-value-list+:
<basic-value>:
*integer-value+,iv.
Let et be <evaluated-target,: gr. Perform assign(et,av,ddi) to assign the
value of the <extent-expression> to the object of the <refer-option>.
186
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 205

7.2.11 FIND-BLOCK-STATE-OF-DECLARATION
Operation: find-block-state-of-declaration(dp)
where dp is a <declaration-designator> designating the <declaration>,d.
result: a <block-state+.
step 1. Let bs be the current *block-state,. If the <block-control+ of bs contains the
form <remote-block-state>,rbs, then let bs be the <block-state> designated by
rbs.
step 2. Let bb be the corresponding block of bs (see Section 5.2.21. If bb contains d.
then return bs.
step 3. bs must have a <block-environment9,bv. Let bs be the <block-state+ designated
by bv. Go to Step 2.
Chapter 7: Storage and Assignment 187
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 206

7.3 Initialization
7.3.1 INITIALIZE-GENERATION
This operation initializes a <generation, according to the specification contained in a
<declaration>.
Operation: initialize-qeneration(g,d)
Case 1.
Case 2.
Case 3.
Step
where g is a fgeneration,,
d is a <declaration>.
The immediate component of the <data-description> immediately contained in the
<variable> of d is an <item-data-description>,idd.
Perform initialize-scalar-element(g,iddJ.
The immediate component of the <data-description> immediately contained in the
<variable> of d is a <dimensioned-data-description> whose <element-data-
description> is an <item-data-description>.
Perform initialize-array(g,d).
The immediate component of the <data-description>,dd immediately contained in
the <variable> of d is either a <structure-data-description>, or a <dimensioned-
data-description> whose <element-data-description> is a <structure-data-
description>.
Let sdd be the simply contained <structure-data-description> in dd. For each
<member-description> immediately contained in the <member-description-list> of
sdd, chosen in any order, that has an <initial> component, not necessarily
immediate, perform Steps 3.1 through 3.4.
3.1. Let the chosen -ember-description>,md be the i'th immediate component of
the <member-description-list>.
Step 3.2. Perform select-qualified-reference(g,il,d), where il is an <identifier-list>
consisting of the single <identifier>,id. that is a copy of the i'th
immediate component of the <identifier-list> of sdd. to obtain a modified
fgeneration,,gl.
Step 3.3. Let dc be a copy of the <declaration>,d, and let sddc be the copy of sdd
contained in dc.
Case 3.3.1. The immediate component of dd is <dimensioned-data-description> and the
<data-description> of md has a <dimensioned-data-description>,ddd.
Append to the <bound-pair-list> of dc the immediate components of the
<bound-pair-list> of ddd. Replace sddc by the immediate subtree of the
<element-data-description> of ddd; this will always be a <structure-
data-description> or an <item-data-description>.
Case 3.3.2. (Otherwise).
Replace sddc by the immediate subtree of the <data-description> of md.
Step 3.4. Perform initialize-generation(glrdc).
188
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 207

7.3.2 INITIALIZE-SCALAR-ELEMENT
This operation initializes a generation consisting of a single element.
Operation: initialize-scalar-element(g,idd)
where g is a fgeneration,,
idd is an <item-data-description>.
Step 1. If idd does not contain an <initial-element> then terminate this operation.
Step 2. If the <initial-element> component of idd has an <asterisk> then terminate this
operation.
Step 3. Perform evaluate-expression(e), where e is the <expression> simply contained in
the <initial-element> of idd, to obtain an <aggregate-value+,v.
Step 4. Create an *evaluated-target+,et, and attach g to it.
Step 5. Perform assign(et,v,ddl), where ddl is the <data-description> immediately
contained in e.
7.3.3 INITIALIZE-ARRAY
This operation initializes a <generation+ that has an array of <basic-value%.
<evaluated-initial-element,::= <asterisk> 1 <parenthesized-expression> (scalar) 1
{<iteration-factor> 1 <evaluated-iteration-factor,)
<evaluated-initial-element-list> 1
<evaluated-initial-item> 1
<absent+
<evaluated-iteration-factor+::= <integer-value+
<evaluated-initial-item,::= <basic-value+ <data-description>
Operation: initialize-arrav(g,d)
where g is a <generation,,
2 is a <declaration>.
Step 1. Let ead be a copy of the <evaluated-data-description+ of g in which the simply
contained <dimensioned-data-description> has been replaced by the subtree of its
<element-data-description>. If d does not have an <initial-element-list> then
terminate this operation. Let iel be the <initial-element-list> of d.
Step 2. Let mandnbel. Let mt be the number of elements in the <storage-index-1isW
of g. Let nt be the number of elements in the <initial-element-list>,iel.
Step 3. Construct an <evaluated-initial-element-list,reiel. by making a copy of iel and
replacing each <initial-element-list> node by an fevaluated-initial-element-
list+ and each <initial-element> node by an <evaluated-initial-element, in the
copy-
Step 4. Perform Steps 4.1 through 4.7 while m I mt and n I nt.
Step 4.1. Let the n'th element of eiel be eieIn1.
Step 4.2. If eieCn1 immediately contains an <iteration-factor>.itf, then perform
evaluate-expression-to-integer(e), where e is the <expression> of itf, to
obtain the *integer-value+,v, and replace itf by an tevaluated-iteration-
factor, containing v.
Chapter 7: 'Storage and Assignment 189
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 208

Step 4.3. If eie[nl immediately contains an <evaluated-initial-element-list+,eiell,
then for each <expression>,e, contained in eiell, chosen in any order,
perform Step 4.3.1.
Step 4.3.1.
Case 4.3.1.1. e is immediately contained in an <iteration-factor>,itfl.
Optionally perform evaluate-expression-to-integer(e) to obtain an
<integer-value9,il and replace itfl by an fevaluated-iteration-
factor, containing il.
Case 4.3.1.2. e is immediately contained in a <parenthesized-expression>,pe.
Optionally perform evaluate-expression(e) to obtain an
<aggregate-value> having a <basic-value>,bv, and replace pe by an
*evaluated-initial-items comprising of bv and a copy of the
<data-description> immediate component of pe.
Step 4.4. If eie[nl immediately contains an <evaluated-iteration-factors,eif, then
perform Steps 4.4.1 through 4.4.3.
Step 4.4.1. Let eie12 be the <evaluated-initial-element-list+ of eieCn1. Let i2 be
the <integer-value> of eif.
Step 4.4.2.
Case 4.4.2.1. i2 < 0.
Replace eie[nl by *absents.
Case 4.4.2.2. (Otherwise).
Let k be the number of elements in eie12. Replace eieinl by the
i2*k elements formed from i2 replications of the sequence of
elements of eie12. Let nt be nt+i2*k-1.
Step 4.4.3. Go to Step 4.1.
Step 4.5. If eieCn1 is neither an <asterisk> nor an <absent, then perform steps 4.5.1
and 4.5.2.
Step 4.5.1. If eieCn1 is a <parenthesized-expression>,pel, then perform evaluate-
expression(e), where e is the <expression> of pel, to obtain an
<aggregate-value, having the fbasic-value?,bvl, and replace pel by an
<evaluated-initial-item+ comprising of bvl and a copy of the <data-
description> immediate component of pel.
Step 4.5.2. eieCn1 is an <evaluated-initial-items. Let v be of the form
<aggregate-value*:
4aggregate-types:
*scalar>;
4basic-value-list>:
bv;;
where bv is the fbasic-values of eieCn1. Let dd be the <data-
description> immediate component of eieinl. Letsibe a copy of the
math element of the &storage-index-list, of g. Perform
assignffevaluated-target,:gl;rv,ddl, where gl is the <generation+
comprising of edd constructed in step 1, a copy of the <allocation-unit-
designator9 of g, and a <storage-index-list+ consisting of the single
element si.
step 4.6. lf eiefnl is not <absent+ then let m be m+l.
Step 4.7. Let n be n+l.
190
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 209

7.4 The Freeing of Storage
The <free-statement> causes storage allocated for specified <based> or <controlled>
variables to be freed.
7.4.1 EXECUTE-FREE-STATEMENT
Operation: execute-free-statement(fs1
where fs is a <free-statement>.
Step 1. For each <freeing>,fr, in the <freeing-list> of fs, chosen in left-to-right
order, perform Steps 1.1 and 1.2.
step 1.1. Let d be the <declaration> designated by the <declaration-designator>
component of fr.
Step 1.2.
Case 1.2.1. The <storage-class> of d contains <controlled>.
Perform free-controlled-storagetfrl.
Case 1.2.2. The <storage-class> of d contains <based>.
Perform free-based-storage(fr).
step 2. Perform normal-sequence.
7.4.2 FREE-CONTROLLED-STORAGE
This operation frees the most recent allocation of a <controlled> variable.
Operation: free-controlled-storaqetfr)
where fr is a <freeing>.
step 1. Let d be the <declaration> designated by the <declaration-designator>,dp, of fr.
step 2. Perform find-directory-entry to obtain a &controlled-directory-entry9,e,
corresponding to d.
Step 3. If e contains a <generation-list+,gl, perform Steps 3.1 to 3.3.
Step 3.1. Let g be the last <generation, in gl.
Step 3.2. Perform free(g).
Step 3.3. Delete g from gl.
Chapter 7: Storage and Assignment 131
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 210

7.4.3 FREE-BASED-STORAGE
This operation frees a <based> variable specified in a <freeing>.
Operation: free-based-storaqetfr)
step 1.
step 2.
Step 3.
where fr is a <freeing>.
fr can consist of three components:
a <declaration-designator>.dp
a <locator-qualifier>
and an <in-option>.
Of these, only dp always exists.
Let dd be the <data-description> of the <declaration> designated by dp. If fr
contains a <locator-qualifier>.lq then let vr be <variable-reference>: lq dp
dd. Otherwise let vr be <variable-reference>: dp dd. Let n be the number of
<bound-pair>s in dd. If n is not equal to zero, attach a <subscript-list>
containing n occurrences of <asterisk> to vr.
Perform evaluate-variable-reference[vr) to obtain the fgeneration+,gf to be
freed. Let au be the tallocation-unit, designated by the fallocation-unit-
designator+ of gf.
Step 3.1. If the <data-type> components of dd either all contain <character>,
<nonvaryinq>, and <unaliqned> or all contain <bit>, <nonvarvinq>, and
<unaliqned> then the number of elements in each fcharacter-value-list, or
<bit-value-list> in au must equal the corresponding Unaximum-length> in the
<evaluated-data-description, of gf.
Step 3.2. If there are n elements in the <basic-value-list+ of au then there must be n
elements in the <storage-index-list, of gf and the i'th element of the
<storage-index-list, must have a <basic-value-index+ that contains an
<integer-value> equal to i for all values of i from 1 through n.
Step 4. Perform deduce-in-option( If an area containing gf can be inferred from fr,
a fgeneration*,ga, will be obtained: otherwise <fail, will be obtained.
step 5.
Case 5.1. ga exists-
Step 5.1.1. Let av be the <area-value+ referred to by ga. The tarea-allocation-
list> of av must contain au. Let aa be the <area-allocation> containing
au and let sal be the <significant-allocation-list, of av. Let n be the
number of elements of the <significant-allocation-list, of aa. Replace
the tallocated, component of the n'th element of sal by *freed+.
Step 5.1.2,. If aa is the only *area-allocation+ of av then replace av by farea-
values: <empty+. Otherwise, delete aa from the <area-allocation-list+
of av and perform Step 5.1.2.1.
Step 5.1.2.1. If the last element, el, of sal contains *freed, then delete el
and go to Step 5.1.2.1.
Case 5.2. deduce-in-option returned *fail+ in Step 4.
The <allocation-unit> au must be an immediate component of the fallocation-
unit-list+ of *allocated-storage+. The <generation> gf must not:
(1) be equal to a component of any of the following:
(1.1) the <controlled-directory+
(1.2) the &static-directory,
(2) for any *block-state,, be equal to a component of
192
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 211

(2.1) the <automatic-directory+
(2.2) the <parameter-directory+ as the component of an
*established-argument, containing <dummy,
(31 for any <file-information+, be equal to a component of the
<allocated-buffer>.
Perform freecgf).
7.4.4 DEDUCE-IN-OPTION
This operation infers, if possible, an area <generation+ in which a freeing is to be
applied.
Operation: deduce-in-option .
where fr is a <freeing>.
result: a <generations or <fail%.
Case 1. fr contains an <in-option>.
Let io be the <in-option> of fr. Let vr be the <variable-reference> component
of io. Perform evaluate-variable-referencecvr) to obtain the fgeneration>,ga.
Return ga.
Case 2. fr contains no <in-option>.
Step 2.1.
Case 2.1.1. There is a <locator-qualifier> as an immediate subnode of fr.
Let vr be the <value-reference> contained in the <locator-qualifier>.
Case 2.1.2. There is no <locator-qualifier> as an immediate subnode of fr.
The <declaration-designator> of fr designates a <declaration>,d. The
<based> component of d must have a <value-reference>. Let vr be this
<value-reference>.
Step 2.2.
Case 2.2.1. vr contains a <variable-reference> with a <declaration-designator> that
designates a <declaration> whose <data-type> contains <offset>.
The <declaration-designator> of vr designates a <declaration>,dvr. The
<offset> component of dvr must have a <variable-reference> component,
vra. Perform evaluate-variable-reference(vra) to obtain the
fgeneration>,ga. Return ga.
Case 2.2.2. (Otherwise).
Return the value <fail*.
7.4.5 FREE
This operation frees an <allocation-unit+.
Operation: - free(g)
where g is a tgeneration+.
Step 1. Iet au be the <allocation-unit+ designated by the <allocation-unit-designator>
in g.
Step 2. Delete the fallocation-unit>,au, from the <allocation-unit-list, of fallocated-
storage>.
Chapter 7: 'Storage and Assignment 193
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 212

7 3 Assignment
Assignment involves changing *basic-value, components of storage (the common case),
components in the <file-information-list+ (in the case of the <paqeno-pv>), or components
in the <condition-bif-value% of the current &lock-state> (in the case of the <pseudo-
variable>s <onchar-pv> and <onsource-pv>). The components to be changed are determined
by evaluating a <target-reference> (Section 7.5.2). to obtain an <evaluated-target+
(Section 7.5.2.1). The actual assignment is effected by the operation assign (Section
7.5.31, which, in general, will involve conversion of <basic-value% and promotion of
<aggregate-value%. The full generality of assignment is available through the
<assignment-statement> (Section 7.5.1). but other constructions cause invocation of the
operation assign.
7.5.1 THE ASSIGNMENT STATEKENT
Operation: execute-assiqnment-statementtast)
where ast is an <assignment-statement>.
Step 1. Perform Steps 1.1 and 1.2 in either order.
Step 1.1. Let tr be the leftmost <target-reference> of the <target-reference-list> of
ast. Perform evaluate-target-referencectr) to obtain the fevaluated-
target+,et.
Step 1.2. Let e be the <expression> of ast. Perform evaluate-expression(e) to obtain
an <aggregate-value>,v.
Step 2. Perform assign(et,v,d), where d is the <data-description> immediately contained
in e.
Step 3.
Case 3.1. ast contains no unevaluated <target-reference>.
Perform normal-sequence.
Case 3.2. ast contains one or more unevaluated <target-reference>s.
Let tr be the leftmost unevaluated <target-reference>. Perform evaluate-
target-referencettr) to obtain the <evaluated-target+,et. GO to step 2.
7.5.2 TARGET REFERENCES
Attributes: The result <data-description> of a <target-reference> that immediately
contains a <variable-reference> is the same as the result <data-description>
of the <variable-reference>. For a <target-reference> that has a <pseudo-
variable-reference>, the result <data-description> is given in the
description of the <pseudo-variable> (Section 7.5.4).
Operation: evaluate-tarqet-referenceftr)
where tr is a <target-reference>.
result: an *evaluated-target,.
Case 1. The immediate component of tr is a <variable-reference>,vr.
Perform evaluate-variable-referencetvr) to obtain a fgeneration,,g. Return an
<evaluated-target,: g.
194
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 213

Case 2. The immediate component of tr is a <pseudo-variable-reference>,pvr.
Perform evaluate-fIxCll,...,x[n3)r where f is the immediate component of the
<pseudo-variable> contained by pvr, and x~ll,...,x[nl are the <argument>s, if
any, in the <argument-list>. Let g be the result of evaluate-f(xClJ,...,x[nl).
(It will be either a <generation+ or an <evaluated-pseudo-variable-reference>.)
Return an <evaluated-target+: g.
7.5.2.1 Evaluated Tarqets
Operation: value-of-evaluated-tarqet(t1
where t is an <evaluated-target>.
result: an *aggregate-value+.
Case 1. t immediately contains a tgeneration+,g.
Perform value-of-generation(g) to obtain an <aggregate-value>,av, which must not
contain <undefined>. Return av.
Case 2. t contains <imaq-pv> or <real-pv>.
Let g be the <generation+ in t, and perform value-of-generation(g) to obtain an
<aggregate-value,,x, which must not contain *undefined>. Let sr be the scalar-
result of performing Steps 1 and 2 of imag-bif (see Section 9.4.4.40) or Steps 1
to 3 of real-bif (see Section 9.4.4.661, respectively, taking:
the scalar-value of x to be the *basic-value> in x;
the scalar-result-type to be the <data-type> in the *generation, of t, with
<complex> replaced by Cm>.
Return an <aggregate-value, containing sr.
Case 3. t contains <onchar-pv> or <onsource-pv>.
Perform onchar-bif (see Section 9.4.4.55) or onsource-bif (see Sectior
9.4.4.61). respectively, to obtain an <aggregate-value>,av. Return av.
Case 4. t contains <paqeno-pv>.
The *file-value+,fv in t must obey the constraints of pageno-bif, Step 1 (see
Section 9.4.4.621. Perform Steps 2 and 3 of pageno-bif.
Case 5. t contains <substr-pv>.
Perform value-of-generation(g) to obtain an *aggregate-value+,sa, where g is the
<generations in t. sa must not contain <undefined+. Let st be the first
<aggregate-value% in t; let le be the second <aggregate-value+ in t, if present.
Let sr be the scalar-result of performing Steps 1 to 4 of substr-bif (see
Section 9.4.4.761, taking:
the scalar-value of sa (or of st or of le) to be the single <basic-value, in
sa (or in st or in le);
the scalar-result-type to be the <data-type> in the fgenerations in t.
Return an 4aggregate-value, containing sr.
Case 6. t contains Kunspec-pv>.
Let g be the <generation+ in t. Perform step 3 of unspec-bif (see Section
9.4.4.85).
Chapter 3: Storage and Assignment 195
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 214

7.5.3 THE ASSIGNMENT OPERATION
Operation: assiqn(et,sv,sd)
where et is an *evaluated-target>,
sv is an *aggregate-value,,
sd is a <data-description>.
Case 1. et immediately contains a fgeneration+,g.
Step 1.1. Let edd be the <evaluated-data-description+ of g. Perform promote-and-
convert(edd,sv,sd), to obtain an <aggregate-value+,av.
Step 1.2. Perform set-storage(g,svl), where svl is the <basic-value-list+ of av.
Case 2. et immediately contains
Let f be the component
the <pseudo-variable>.
7.5.3.1 Promote-and-convert
an <evaluated-pseudo-variable-reference>,epvr.
of the <pseudo-variable> in epvr. Thus f is the name of
Perform assign-f(epvr,sv,sd).
Let x and y be <aggregate-type+s. Then x is promotable to y if x and y are compatible
and y is the same as the common <aggregate-type+ of x and y. However where one has a
<bound-pair> which has <asterisk>, the other may have a <bound-pair> whose <expression>
components contain <integer-value%.
Operation: promote-and-convert(td,sv,sd)
where td is a <data-description> or <evaluated-data-description+,
sv is an <aggregate-value+,
sd is a <data-description>.
result: an *aggregate-value*.
step 1. Let ats be the <aggregate-type+ of sv.
Case 1.1. td is an <evaluated-data-description,.
Let tdd be td.
Case 1.2. td is not an fevaluated-data-description+-
Perform evaluate-data-description-for-allocation to obtain an
<evaluated-data-description,rtdd. For each tree of the form <bound-
pair>,bp: <asterisk>; contained in tdd that is not a component of an
<entry>, perform Step l-2.1.
Step 1.2.1. If a <bound-pair>,cbp corresponding to bp exists in ats, then replace bp
by cbp. Otherwise, replace bp by a <bound-pair> whose <lower-bound> and
<upper-bound> each have an <integer-value+ of 1.
Step 2. Each <bound-pair-list> in ats must equal the corresponding <bound-pair-list> in
tdd.
Step 3. Perform scalar-elements-of-data-description(tdd1 to obtain an integer, n. Let
av be an <aggregate-value> whose <aggregate-type, equals the <aggregate-type,
associated with the <data-description> of tdd, and whose *basic-value-list+,svl,
contains n <basic-value> nodes, each of which has no subnode.
Step 4. For each distinct value of j between 1 and n, taken in any order, perform steps
4.1 through 4.3.
Step 4.1. Perform find-item-data-description(tdd,j) to obtain an <item-data-
description> that describes the j'th scalar-element of av. Let ydt be the
<data-type> of this <item-data-description>.
196
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 215

step 9.2. Let x be the scalar-element of sv that corresponds to the j'th scalar-
element of av. Let xdt be the <data-type> of the <item-data-description> in
sd which corresponds to x in sv.
Step 4.3.
Case 4.3.1. Both xdt and ydt contain <computational-type> , or one contains <offset>
and the other contains <pointer>.
Case 4.3.2. Both xdt and ydt contain <area>.
Case 4.3.3.
Perform convert(ydt,xdt,x) to obtain yCj1.
Optionally perform raise-condition(<area-condition>). If there is a
normal return from this operation let yfjl be a <basic-value> containing
<undefined*. If raise-condition(<area-condition>) is not performed,
then let yCj1 be a copy of x.
(Otherwise).
Let yCj1 be a copy of x.
Step 5. Append the yCjl to av in order and return av.
7.5.3.2 The Set-storaqe Operation
This operation sets the elements of a <basic-value-list+ in <allocated-storage+ described
by a <generation+ to have the values contained in a fbasic-value-list>.
Operation: set-storaqe(g,ssvl)
where q is a <generation+,
ssvl is a <basic-value-list>.
Step 1. Let au be the <allocation-unit+ designated by the <allocation-unit-designator,
of q. au must be contained in <allocated-storage>. Let tsvl be the fbasic-
value-list+ of au.
Step 2. Let dd be the <data-description> of the <evaluated-data-description> of q and
let sil be the <storage-index-list+ of g.
Step 3. For each of the elements of sil, taken in any order, perform Steps 3.1 through
3.3.
Step 3.1. Let si be the chosen element and let i be its ordinal in sil. Let j be a
copy of the value of the <integer-valu++ of the *basic-value-index, of si.
Step 3.2. Let ssv be the i'th element of the <basic-value-list%,ssvl.
Step 3.3.
Case 3.3.1. si contains a <position-index+.
Let k be a copy of the value of the <integer-value+ of the fposition-
index, of si. ssv will contain either a <character-string-value+ or a
<bit-string-value>. Perform find-item-data-description(ddri) to obtain
the <item-data-description>ridd. of the target scalar-element.
If iaa contains <pictured> then let n be its associated strinq-length;
otherwise let n be the value of the <maximum-length> in idd. Let m be
1. Perform Steps 3.3.1.1 through 3.3.1.4 n times.
Step 3.3.1.1. Let tl be the *character-value-list, or <bit-value-list, of the
j'th element of tsvl.
Step 3.3.1.2. Replace the k'th element of tl by a copy of the m'th element of
ssv.
Chapter 7: 'Storage and Assignment 197
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 216

Step 3.3.1.3. Let k be k+l. If k is now greater than the number of elements in
tl,' then let k be 1 and let j be j+l. Repeat Step 3.3.1.3-l
while the j'th element of tsvl contains <null-character-strinq,
or <null-bit-strinq,.
Step 3.3.1.3.1. Set j to j+l.
Step 3.3.1.4. Let m be m+l.
Case 3.3.2. si does not contain a <position-index+.
Replace the j'th element of tsvl by a copy of ssv.
7.5.4 PSEUDO-VARIABLES
This Section presents the definitions of the <pseudo-variable% in alphabetical order.
For each <pseudo-variable>:
The Arguments Section indicates the number of arguments to the <pseudo-variable> and
supplies names by which the arguments are referenced in the Attributes and Constraints
Sections.
The Constraints Section specifies constraints on the arguments.
The Attributes Section defines the <data-description> of the <pseudo-variable-
reference> (and of the <target-reference> containing it) in terms of the <data-
description>s of the arguments.
One or two operations are defined for each <pseudo-variable>. This first operation,
evaluate-f, where f is the name of the <pseudo-variable>, is used by the operation
evaluate-target-reference. This operation returns a <generation> or an tevaluated-
pseudo-variable-reference.
The second operation, assign-f, where f is the name of the <pseudo-variable>, is defined
only for those <pseudo-variable>s whose evaluate-f operation yields an tevaluated-pseudo-
variable-references.
7.5.4.1 Imaq-pv
Arguments: X
Constraints: X must have the form <argument>: <expression>: <value-reference>:
<variable-reference>.
All <data-type>s of x must have <arithmetic> (including <arithmetic> in
<pictured-numeric>) with <mode>: <complex>.
Attributes: The result <aggregate-type, is the *aggregate-type+ of x. Each result
<data-type> is the same as the corresponding <data-type> of x except
that it has <mode>: <a>.
Operation: evaluate-imaq-pv(x)
Step 1. Let y be the <variable-reference> in x. Perform evaluate-variable-reference(y)
to obtain a fgeneration>,g. Return an <evaluated-pseudo-variable-reference+:
<pseudo-variable>: <imaq-pv>; g.
198
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 217

Operation: assiqn-imaq-pv(t,sv,sd)
where t is an <evaluated-pseudo-variable-reference>,
sv is an *aggregate-value>,
sd is a <data-description>.
Step 1. Let t have the form
*evaluated-pseudo-variable-reference>:
fgeneration3,g:
<evaluated-data-description,.dc
<allocation-unit-designator,.aud.
Let dr be the same as dc except that all <mode>s have <real>. Perform promote-
and-convert(dr,sv,sd) to obtain vr.
Step 2. For each <storage-index+,p in g, perform Steps 2.1 and 2.2.
Step 2.1. Let sdc, svr and sdr be the scalar-elements of dc, vr and dr, respectively,
corresponding to p-
Step 2.2.
Case 2.2.1. sdc has Karlthmetic> but not <pictured-numeric>.
Let sg be a
<generation,:
<evaluated-data-description,:
sdc;
aud
<storage-index-list,:
P-
Let zl be the value of sg (zl contains a single <complex-value+). Let
22 be a <complex-value+ with first component as zl and second component
equal to the component of svr. Perform set-storagecsg ,bvl) where bvl is
a <basic-value-list3 containing 22.
Case 2.2.2. sdc has <pictured-numeric>
Let sg be a
igeneration,:
<evaluated-data-description>:
sdr;
aud
<storage-index-list>:
P-
Let n be the associated character-string length of the <pictured-
numeric> in sdr. If p contains a <position-index,, increment its value
by n; otherwise, append to p a <position-index> of n+l. Perform set-
storage(sg,bvl), where bvl is a <basic-value-list> containing svr.
7.5.4.2 Onchar-pv
Arguments: (none)
Attributes: The result has 4aggregate-types: <scalar,. The result <data-type> has
<character>.
Operation: evaluate-onchar-pv
Step 1. Return an *evaluated-pseudo-variable-reference+:
<onchar-pv>.
<pseudo-variable>:
Chapter 7: Storage and Assignment 199
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 218

Operation: assiun-onchar-pv(t,sv,scU
where t is an <evaluated-pseudo-variable-references,
sv is an *aggregate-value>,
sd is a <data-description>.
step 1. In either order, perform get-established-onvalue(<onchar-value,) to obtain i and
perform get-established-onvalue(<onsource-value,) to obtain str. i and str must
not be ifail,.
step 2. Convert sv to <character> of length 1, using the <data-type> in sd as the source
<data-type> for the conversion, to obtain c.
step 3. If i>O, replace the i'th <character-value+ in str by the <character-value% in c.
7.5.4.3 Onsource-pv
Arguments: (none)
Attributes: The result has <aggregate-type>: <scalar+. The result <data-type> has
<character>.
Operation: evaluate-onsource-pv
step 1. Return an <evaluated-pseudo-variable-reference>:
<onsource-pv>-
<pseudo-variable>:
Operation: assiqn-onsource-pv(t,sv,sd)
where t is an fevaluated-pseudo-variable-reference+,
sv is an *aggregate-value*,
sd is a <data-description>.
Step 1. Perform get-established-onvalue(fonsource-value+) to obtain str. str must not
be *fail>. Let n be its length. Convert sv to <character> of length n, using
the <data-type> in sd as the source <data-type> for this conversion, to obtain
cv.
Step 2. Replace str by cv.
7.5.4.4 Paqeno-pv
Arguments: fn
Constraints: fn must have <aggregate-type+: <scalar*. The <data-type> of fn must have
<file>.
Attributes: The result has <aggregate-type*: <scalar,. The result <data-type> has
integer-type.
Operation: evaluate-paqeno-pv(fn)
Step 1. Perform evaluate-expression to obtain fv. Return an fevaluated-pseudo-
variable-reference+: <pseudo-variable>: <paqeno-pv>; fv.
200
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 219

Operation: assiqn-paqeno-pv(t,sv,sd)
where t is an <evaluated-pseudo-variable-rererence>,
sv is an *aggregate-values,
sd is a <dat.a-description>.
Step 1. Let f be the <file-information+ designated by the <file-value+ in t. f must
have fopens. The fcomplete-file-description+ of f must have <print>.
Step 2. Let bv be the <basic-value+ in sv. Let tdt be a <data-type> which is integer-
type (Section 9.1.2). Perform convert(tdt,sd,bv) to obtain a *real-value+,rv.
rv must be non-negative.
Step 3. Set the <page-number+ component in f to contain an <integer-values with the same
component as rv.
7.5.4.5 Real-pv
Arguments: x
Constraints: x must have the form <argument>: \expression>: <value-reference>: <variable-
reference>.
All <data-type>s of x must have <arithmetic> (including <arithmetic> in
<pictured-numeric>), with *ode>: <complex>.
Attributes: The result iaggregate-type+ is the <aggregate-type, of x. Each result
<data-type> is the same as the corresponding <data-type> of x except that it
has <mode>: <a>.
Operation: evaluate-real-pv(x)
Step 1. Let y be the <variable-reference> in x.
Step 2. Perform evaluate-variable-reference(y) to obtain a fgeneration>,g. Return an
fevaluated-pseudo-variable-reference,:<priable>: <real-pv>; g.
Operation: assiqn-real-pv(t,sv,sd)
where t is an <evaluated-pseudo-variable-reference>,
sv is an <aggregate-value+,
sd is a <data-description>.
Step 1. Let t have the form
ievaluated-pseudo-variable-reference+:
fgenerations,g:
<evaluated-data-description3,dc
fallocation-unit-designator+,aud.
Let dr be the same as dc except that all (mode>s have <m>- Perform promote-
and-convert(dr,sv,sd) to obtain vr.
Step 2. For each <storage-indexs,p in g, perform Steps 2.1 and 2.2.
Step 2.1. Let sdc, svr, and sar be the scalar-elements of ac. vr, and ar.
respectively, corresponding to p-
Chapter 7: Storage and Assignment 201
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 220

Step 2.2.
Case 2.2.1. sdc has <arithmetic>, but not <pictured-numeric>.
Let sg be a
fgeneratioti:
*evaluated-data-description9:
sdc;
aud
*storage-index-list+:
Pm
Let 21 be the value of sg (21 contains a single <complex-value+). Let
22 be a <complex-value9 with first component equal to the component of
svr and second component as zl. Perform set-storage(sg,bvl) where bvl
is a <basic-value-list+ containing 22.
Case 2.2.2. sdc has <pictured-numeric>.
Let sg be a
<generations:
<evaluated-data-descriptions:
sdr:
aud
<storage-index-list+:
P*
Perform set-storage(sg,bvl) where bvl is a fbasic-value-list+ containing
SVT.
7.5-4.6 Strinq-pv
Arguments: x
Constraints: x must have the form <argument>:<expression>: <value-reference>: y;;; where
y is a <variable-reference>. Let ad be the <declaration> designated by the
<declaration-designator> in x. Each <item-data-description> in ad must have
<unaliqned>. Further, one of the following two conditions must hold:
(1) all <data-type>s in ad must have <character>: <nonvaryinq> or
<pictured>, or
(2) all <data-type% in ad must have <&>: <nonvaryinq>.
Attributes: The result has <aggregate-type+: <scalar>. The result <data-type> has the
derived common <string-type> of the <data-type>s of x, and <nonvarvinq>.
Operation: evaluate-strinq-pv(x)
Step 1. Let y be the <variable-reference> in x. Perform evaluate-variable-reference(y)
to obtain a fgeneration,,gy, which must be connected. Let the fevaluated-data-
description*, <allocation-unit-designator+, and <storage-index-list+ components
of gy be cl, aud and s, respectively.
Step 2. If the <data-type> of the i'th scalar-element in the <generation, has <string>,
let kIi1 be the value of the corresponding Unaximum-length> component in d;
otherwise let ktil be the associated character-string length of the <pictured>
in d.
Step 3. Return a +generations,g, with components as follows. The fevaluated-data-
description+ of g is described under Attributes, above, with <maximum-length>
the sum of all the k[il. The <allocation-unit-designator> of g is a copy of
aud. The <storage-index-list+ of g contains a copy of the first component of s.
202
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 221

7.5.4.7 substr-pv
Arguments: t, st C,lel
Constraints: t must have the form <argument>: <expression>: <value-reference>:
<variable-reference>.
All the <data-type>s of t must have <string>. The <aggregate-type>s of the
<argument>s st and le must be promotable to the <aggregate-type+ of t. All
<data-type% of st and le must have <computational-type>.
Attributes: The result <aggregate-type+ is the <aggregate-type> of t. Each <data-type>
is the same as the corresponding <data-type> of t.
Operation: evaluate-substr-pv(t,st,le)
Step 1. Let ty be the <variable-reference> in t. In any order, perform evaluate-
variable-referenceftyl to obtain a fgeneration*,g, perform evaluate-
expression and evaluate-expression( if le occurs, to obtain fagqregate-
value%, x and y.
Step 2. Corresponding <bound-pair>s in the <aggregate-type& of qr XI and y must be
equal.
Step 3. Let x' and y* be *aggregate-value% whose <aggregate-type,s are the same as
those of x and y, respectively, and whose scalar-elements are obtained by
converting the corresponding scalar-elements of x and y to integer-type.
step 4. Return an <evaluated-pseudo-variable-reference+: <pseudo-variable>: <substr-pv>;
g x' Ly'l.
Operation: assiqn-substr-pv(t,sv,sdl
where t is an <evaluated-pseudo-variable-reference> as returned
by evaluate-substr-pv, i.e.
<evaluated-pseudo-variable-reference%+:
<pseudo-variable>:
<substr-pv>;
fueneration+.a: 2
<evaluated-data-description+,edd
<allocation-unit-desiqnator>,aud
<storage-index-list+,sil;
<aggregate-value>,xl
[<aggregate-value*,yll:,
sv is an <aggregate-value>,
sd is a <data-description>.
Step 1. Corresponding <bound-pair>s in the <aggregate-type% of sv and g must be equal.
For each *storage-index,,p, in sil, taken in any order, perform Steps 1.1
through 1.3.
Step 1.1. Let pp be the ordinal of p within sil. Perform find-item-data-
description(edd,pp) to obtain an <item-data-description>,idd. Let sg be a
<generation,:
<evaluated-data-description+:
<data-description>:
idd;;
aud
<storage-index-list>:
P.
Perform value-of-generation to obtain an 4aqgreqate-value+,av. If sd
has <varvinq> then av must not contain <undefined*. Let k be the length of
the *character-string-value+ or <bit-string-value% in av (cf. Section
9.1.3.4).
Step 1.2. Let i be the scalar-element in xl corresponding to p in sil. If yl exists,
let j be the scalar-element in yl corresponding to p in sil; otherwise let
j=k-i+l.
Chapter 7: Storage and Assignment 203
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 222

Step 1.3.
Case 1.3.1. 0 < i-l 5 j+i-1 I k.
Step 1.3.1.1. Let st be the <string-type> contained in sg. Let sgl be a
*generation*:
<evaluated-data-description+:
<data-description>:
<item-data-description>:
<data-type>:
<computational-type>:
<string>:
st
Unaximum-length>:
<extent-expression>:
<integer-value+:
3;;;
<nonvarvinq>**.*.. ,,,,,,
aud
<storage-index-list*:
P-
If P contains a <position-index,, increment its value by i-l:
otherwise append to p a <position-index+ of i.
Step 1.3.1.2. Perform convert(dtl,ssv, ssd) to obtain ssvl, where dtl is the
<data-type> of sgl, ssv is the scalar-element in sv corresponding
to pinsil, and ssd is the <data-type> in sd corresponding to p
in sil.
Step 1.3.1.3. Perform set-storage(sgl,ssvl).
Case 1.3.2. (Otherwise).
Perform raise-condition(<strinqranqe-condition>).
7.5.4.8 Unspec-pv
Argument: X
Constraints: x must have the form <argument>: <expression>: <value-reference>:
<variable-reference>.
x must have <aggregate-type+: fscalar>.
Attributes: The result has <aggregate-type+: <scalar+. The result <data-type> has
<bit>.
operation: evaluate-unspec-pv(xl
step 1. Let y be the <variable-reference> in x. Perform evaluate-variable-reference(y)
to obtain a fgeneration+,g. Return an <evaluated-pseudo-variable-reference>:
<pseudo-variable>: <unsnec-pv>; g.
Operation: assiqn-unspec-pv(t,sv,sd)
step 1.
Step 2.
204
where t is an <evaluated-pseudo-variable-reference>,
sv is an <aggregate-value>,
sd is a <data-description>.
sv must have <aggregate-type+: <scalar+. Let g be the *generation9 in t.
Convert sv to <bit> of length n, where n depends on g in an implementation-
defined fashion.
In an implementation-defined fashion construct from the converted value of sv a
easic-value+,v, depending on properties of g. This value may contain
<undefined%. Perform set-storage(g,v).
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 223

7.6 Variable-reference
7.6.1 EVALUATE-VARIABLE-REFERENCE
The result of this operation is the <generation+ referenced in the <variable-reference>.
Operation:
Step 1. Let
Step 2.
Case 2.1.
evaluate-variable-referencetvr)
where vr is a <variable-reference>.
result: a *generatio*.
d be the <declaration> designated by the <declaration-designator>,dp, of vr.
The <storage-type> of d has <based>.
Perform select-based-generation to obtain the fgeneration9.g.
Case 2.2. The <storage-type> of d has <defined>.
Perform evaluate-defined-referencetvr) to obtain the fgeneration+,g. Go to
Step 6.
Case 2.3. The <storage-type> of d has <parameter>.
Case 2.4.
step 3. If
Perform find-block-state-of-declaration(dp) to obtain the <block-state+,bs,
of d. Find the <parameter-directory-entry,rpde. in the <parameter-directory+
of bs whose <identifier> component is the same as the <identifier> immediate
component of d. pde must not immediately contain the <undefined+ component.
Let g be a copy of the <generation> of pde. The fallocation-unit>
designated by the <allocation-unit-designator+ of g must be contained in the
<allocation-unit-list+, possibly as a component of an <area-value+.
The <storage-type> of d has <automatic>, <controlled>, or <static>.
Perform find-directory-entry to obtain a directory entry. If the
<storage-type> of d has <controlled>, the entry must have a <generation-
list,; let g be a copy of the <generation9 most recently added to the
<generation-list>. Otherwise, let g be a copy of the <generation+ component
of the directory entry.
vr immediately contains an <identifier-list>,il, then perform select-
qualified-reference(g,il,d) to obtain a tgeneration+,g.
step 4. If vr has a <by-name-parts-list> perform evaluate-by-name-parts-list(g,vr,d) to
obtain a fgeneration+,g.
Step 5. If vr has a <subscript-list>, then perform select-subscripted-reference(g,sl),
where sl is the <subscript-list>, to obtain a fgeneration9,g.
step 6. In the <evaluated-data-description9 of g, replace each <offset> component by a
copy of the corresponding <offset> component of the <data-description> immediate
component of the <variable-reference>,vr.
Note: This is a definitional artifice to make the proper target <offset>s available
for conversion in the operation assign,.
step 7. Return g.
Chapter 7: Storage and Assignment 205
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 224

7.6.1.1 Connected Generations
A fgeneration+,g, is connected unless it is found to be unconnected by the following
Steps.
Step 1. Let g be
<generation+:
<evaluated-data-description+,edd:
<data-description>,dd;
<allocation-unit-designator+,aud
<storage-index-list9,si.l.
Let m be the number of elements in sil.
step 2. Let bvlil be the i'th element of the *basic-value-list, of the fallocation-unit,
designated by aud.
Step 3. Let i be the <integer-value+ contained in the <basic-value-index+ of the first
element of sil. Let j be the <integer-value+ contained in the tposition-index9
of the first element of sil, if this <position-index> exists, and 1 otherwise.
For k=l,...,m perform Steps 3.1 and 3.2.
Step 3.1. Perform find-item-data-description(dd.k) to obtain an <item-data-
description>,idd.
Step 3.2.
Case 3.2.1. idd contains both <string> and UlonvarYinq>.
Step 3.2.1.1. Let the k*th element of sil be
<storage-index+:
&basic-value-index+,bvi
<position-index+,poi.
Step 3.2.1.2. If the value contained in the <integer-value> of bvi is not equal
to i, g is unconnected.
Step 3.2.1.3. If the value contained in the <integer-value+ of poi is not equal
to j, g is unconnected.
Step 3.2.1.4. Let ml be the <maximum-length> of idd. Let j be j+ml.
Step 3.2.1.5. Let nelCi1 be the number of elements in the fcharacter-value-
list, or <bit-value-list* of bvCiJ. Repeat Step 3.2.1.5.1 while
j is greater than nellil.
Step 3.2.1.5.1. Let j be j-nelCi1. Let i be i+l.
Case 3.2.2. (Otherwise).
Step 3.2.2.1. Let the k*th element of sil be
<storage-index%
<basic-value-index9,bvi.
Step 3.2.2.2. If the value contained by the *integer-value+ of bvi is not equal
to i, g is unconnected.
Step 3.2.2.3. If j is not 1, g is unconnected.
Step 3.2.2.4. Let i be i+l-
206
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 225

7.6.2 SELECT-BASED-GENERATION
The result of this operation is the *generation, corresponding to the <based> variable
referenced in a <variable-reference> before any name-qualification or subscripting has
been performed.
Operation: select-based-qenerationtvr)
where vr is a <variable-reference>.
result: a *generatio*.
Step 1.
Case 1.1. vr has a <locator-qualifier>,lq.
Let valr be the <value-reference> of lq.
Case 1.2. vr has no <locator-qualifier>.
The <based> component of the <declaration> designated by the cdeclaration-
designator> of vr has a <value-reference>. Let this be valr.
Step 2. Perform evaluate-value-reference(valr) to obtain an <aggregate-values with a
fbasic-value>,v.
step 3. Let dt be the <data-type> of valr. If dt has <offset>, perform
convert(<pointer>,dt,vJ to obtain a <pointer-value+,v.
Step 4. v must not contain tm9- Let g be the <generation* in v.
Step 5. The <allocation-unit, designated by the <allocation-unit-designator, component
of g must be contained in the <allocation-unit-list* of *allocated-storage+,
possibly as a component of an farea-value,.
Step 6. Perform check-based-reference(g,mJ to obtain a fgeneratio+,gl.
Step 7. Return gl.
7.6.3 CHECK-BASED-REFERENCE
This operation checks that the attributes of the variable being referenced agree with
those of the dgeneration, being referenced.
Operation: check-based-reference(g,vrJ
where g is a <generation+.
vr is a <variable-reference>.
result: a *generatio*.
Step 1. Let d be the <declaration> designated by the <declaration-designator> of vr.
Let dd be the <data-description> immediately contained in the <variable> of d.
Step 2. Let g comprise the <allocation-unit-designator+,aud, the tevaluated-data-
description>,edd, and the <storage-index-list+,sil.
Step 3.
Case 3.1. The immediate component of dd is a <structure-data-description>,sdd.
Step 3.X.1. sdd contains an Identifier-list>,mil, and a <member-description-
list>,mdl.
Step 3.1.2. If vr contains an <identifier-list>,il, then let m be the position of
the element of mil that is identical to the first element of il.
Otherwise, let m be the number of elements in mil.
Chapter 7: Storage and Assignment 207
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 226

Step 3.1.3. Let ddl be
<data-description>:
<structure-data-description>:
<identifier-list>,ill
<member-description-list>,mdll;;
where ill contains a copy of the first m elements of il, and mdll
contains a copy of the first m elements of mdl.
Case 3.2. (Otherwise).
Let ddl be a copy of dd.
step 4. Perform evaluate-data-description-for-reference(dd1.g) to obtain an fevaluated-
data-description+,eddl.
step 5.
Case 5.1. eddl has no <refer-option> node and both eddl and edd are such that each
<data-type> component contains <unalisned> and
either (1)
or (2)
each <data-type> component contains Ulonvarvinq> and <w,
each <data-type> component contains either <pictured>, or
<nonvarying> and <character>.
Perform overlay-strings(eddlrgr1) to obtain a *storage-index-list>rsill.
Case 5.2. (Otherwise).
Let cedd be a copy of edd. Delete from cedd and edd2, a copy of eddl, all
occurrences of <refer-option>, <initial>, <local>, and all <variable-
reference> components of <offset>. Each subnode of edd2 must be equal to
the corresponding subnode of cedd except that if edd2 is an
*evaluated-data-description*:
<data-description>:
<structure-data-description>:
-ember-description-list>,mdl2;;;
then the <member-description-list> in cedd corresponding to md12 may have
more components than md12, and the excess components are ignoredi
Let ddle be the <data-descript.on> of eddl. Perform scalar-elements-of-
data-description(ddle) to obtain n, the number of scalar-elements
corresponding to ddle. Let sill be a <storage-index-list+ containing a copy
of the first n elements of sil.
step 6. Return a <generation>: aud eddl sill.
7.6.4 OVERLAY-STRINGS
This operation constructs a <storage-index-list, that reflects the fact that in a string-
overlay defined reference or a reference to a based variable, the resulting *generation,
may describe strings that start inside a <character-string-value+ or *bit-string-value+.
The <position-index, is used to define the starting point of the strings.
Operation: overlay-strinqg(edd,g,indx)
where edd is an <evaluated-data-description*,
g is a *generatio*,
indx is an integer.
result: a *storage-index-list+.
Step 1. Let dd be the <data-description> simply contained in edd. Perform scalar-
elements-of-data-description(dd) to obtain the number of scalar-elements, n,
described by edd. Perform Step 1.1 for i=l,...,n.
208
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 227

Step 1.1. Perform find-item-data-description(dd.i) to obtain the <item-data-
description>.idd of the i'th scalar-element described by dd. If idd
contains <string> then let mfil be the value of the maximum-length> of idd;
otherwise, let mCil be the associated character-string length of the <data-
type> of idd.
Step 2. Let ddg be the <data-description> simply contained in g- Perform scalar-
elements-of-data-description(ddg) to obtain the number of scalar-elements
corresponding to g, ng. Perform Step 2.1 for i=l,...,ng.
Step 2.1. Perform find-item-data-description(ddg,il to obtain the <item-data-
description>,idd, of the i'th scalar element corresponding to g. If idd
contains <string> then let mgiil be the value of the <maximum-length> of
idd; otherwise, let mglil be the associated character-string length of the
<data-type> of idd-
Step 3. Let indxc be a copy of indx. Let ml be the value of the sum mgfll +...+ mgfngl.
Let sil be a <storage-index-list+ with no elements. Let i be 1. Perform Steps
3.1 through 3.7 n times.
Step 3.1. Find the maximum integer j such that j<ng and such that the value of the sum
mgtll +... + mgijl is less than indxc. Let the value of this sum be sag.
Note that j and slg may be zero.
Step 3.2. Let j be j+l.
Step 3.3. Let bvi be the <basic-value-index+ of the j'th element of the tstorage-
index-list% of g, and let p be the <position-index, of the same element.
Step 3.4. Let px be the <integer-value, containing the value indxc-slg-l+p.
Step 3.5. Append
<storage-index*:
bvi
<position-indee:
px:;
to sil.
Step 3.6. Let indxc be indxc+m[il. The value of indxc must not be greater than ml+l.
Step 3.7. Let i be i+l.
Step 4. Return sil.
7.6.5 EVALUATE-DATA-DESCRIPTION-FOR-REFERENCE
This operation takes a <data-description> and a <generation* and constructs an
gevaluated-data-description>, using the <generation> to evaluate any <refer-option>.
Operation: evaluate-data-description-for-reference(dd,gJ
where dd is a <data-description>,
g is a *generation>.
result: an <evaluated-data-description,.
Step 1. Let cdd be a copy of dd. For each <extent-expression>,ee, of cdd, taken in
left-to-right order, perform Step 1.1.
Step 1.1.
Case 1.1.1. ee has a <refer-option>,ro.
Step 1.1.1.1. Let id1 be the <identifier-list> of ro. Let n be the number of
elements in idl. Let cddl be a copy of cdd and let ddp designate
the <data-description>,cddl. Let i = 1. Perform Steps 1.1.1.1-l
through 1.1.1.1.3 n times.
Chapter 7: Storage and Assignment 209
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 228

Step 1.1.1.1.1. Let id be the i'th element of idl. Let sdd be the
<structure-data-description> that is the immediate component
of the <data-description> designated by ddp. Let id11 be
the <identifier-list> of sdd. Let the m'th element of id11
be identical with id.
Step 1.1.1.1.2. Let mdll be the <member-description-list> of sdd. Delete
all elements following the m'th element in both id11 and
mdll. Set ddp to designate the <data-description>
immediately contained in the m'th element of mdll.
Step 1.1.1.1.3. Set i to i+l.
Step 1.1.1.2. Perform scalar-elements-of-data-description(cddl1 to obtain j,
the number of scalar elements corresponding to cddl.
Step 1.1.1.3. Let bv be the j'th element of the <basic-value-list, designated
by g- Replace the <expression> of ee by the *integer-value+
obtained by converting bv to integer-type. The source type for
this conversion is the <data-type> in the <data-description>
designated by ddp.
Case 1.1.2. (Otherwise).
Perform evaluate-expression-to-integer(e), where e is the <expression>
of ee, to obtain the *integer-value3.j. Replace e by j.
step 2. Return the <evaluated-data-description+ containing cdd, as modified, as its
immediate component.
7.6.6 SELECT-QUALIFIED-REFERENCE
This operation selects the part of a <generation, that corresponds to an <identifier-
list>.
Operation: select-qualified-reference(g,idl,d)
Step 1.
Step 2.
where g is a <generation+,
id1 is an <identifier-list>,
d is a <declaration>.
result: a *generation>.
Let tad be the immediate component of the <data-description> immediately
contained in the <variable> of d. Let csil be a copy of the fstorage-index-
list> of g and cedd be a copy of the <evaluated-data-description* of g. Let
read and tedd both be the immediate component of the <data-description> of cedd.
Let ecount be 1. Let m be the number of elements in idl. For j=l,...,m,
perform Steps 2.1 through 2.9.
Step 2.1. If tad is a <dimensioned-data-description> then perform Steps 2.1.1 and
2.1.2.
Step 2.1.1. Let bpl be the <bound-pair-list> of tedd. Let ub[il and lbCi1 be the
<integer-value, components of the <upper-bound> and the <lower-bound> of
the i'th <bound-pair> of bpl respectively. Let ecount be the value of
the product
k
l-r (ub[il-lb[il+l)
i=l
where k is the number of elements of bpl.
Step 2.1.2. Let tdd and tedd be the immediate components of the <element-data-
description>s of tdd and redd respectively.
210
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 229

Step 2.2. tdd and tedd will both be a <structure-data-description>. Let mdl and mil
be, respectively, the <member-description-list> and the <identifier-list> of
tdd. Let emdl be the <member-description-list> of tedd.
Step 2.3. Let the n*th element of mil be the one that is identical with the j'th
element of idl.
Step 2.4. Let the number of elements of mdl be nm. Let ddCk1 be the <data-
description> of the k'th element of emdl. Perform scalar-elements-of-data-
description(ddCk1) to obtain the number of scalar-elements nsefkl
corresponding to dd[kl for l<klnm.
Step 2.5. Let il be the value of the sum nseCll +.,..+ nsetn-11, i2 be nse[nl, and i3
be the value of the sum nseCn+ll+...+nseCnml.
Step 2.6. Let indx be l+(il+i2+i3)*(ecount-1). Perform Steps 2.6.1 through 2.6.3
ecount times.
Step 2.6.1. Delete i3 successive elements of csil starting with the element whose
index is indx+il+i2.
Step 2.6.2. Delete il successive elements of csil starting with the element whose
index is indx.
step 2.6.3. Set indx to indx-(il+i2+i3).
Step 2.7. Let tdd be the immediate component of the <data-description> of the n'th
element of mdl and let tedd be the immediate component of the <data-
description> of the n'th element of emdl.
Step 2.8.
Case 2.8.1.
Case 2.8.2.
Case 2.8.3.
Replace the <element-data-description> of redd by tedd.
(Otherwise).
Replace redd by tedd.
Step 2.9. Let tedd be redd.
tdd is a <dimensioned-data-description> and redd is a <dimensioned-data-
description>.
Append the elements of the <bound-pair-list> of tedd to the <bound-pair-
list> of redd. Replace the <element-data-description> of redd by the
<element-data-description> of tedd.
tdd is a <dimensioned-data-description> and redd is a <structure-data-
description>.
step 3. Return the <generation, comprising a copy of the <allocation-unit-designator+ of
g, csil, and cedd.
Chapter 7: Storage and Assignment 211
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 230

7.6.7 SELECT-SUBSCRIPTED-REFERENCE
This operation selects the part of a <generation, that corresponds to a given <subscript-
list>.
Operation: select-subscripted-reference(g,sbll
where g is a <generation+,
sbl is a <subscript-list>.
result: a <generation+.
Step 1. Let cg be a copy of g. cg will have the form
igeneration,:
<evaluated-data-description>:
<data-description>:
<dimensioned-data-description>:
<element-data-description>,eldd
<bound-pair-list>,bpl;;;
<allocation-unit-designator>
<storage-index-list>,sil.
Step 2. Let cbpl be a copy of bpl.
Step 3. Let dd be a <data-description> that immediately contains a copy of the immediate
subtree of eldd. Perform scalar-elements-of-data-description(dd) to obtain the
integer n.
Step 4. Let m be the number of elements of sbl. Let sblil be the i'th element of sbl.
Let cbplil be the i'th element of cbpl and let ubCi3 and lblil be, respectively,
the <integer-value, contained in the <upper-bound> and <lower-bound> of cbpCi1.
For i=l,...,m, where the values are chosen in any order, perform Steps 4.1 and
4.2.
Step 4.1.
Case 4.1.1. sb[il has <asterisk>.
Let ubp be a copy of cbp[il.
Case 4.1.2. (Otherwise).
Step 4.1.2.1. If sbCi1 immediately contains an <expression>,e then perform
evaluate-expression-to-integer(e) to obtain the tinteger-
value+,v. Otherwise let v be the <integer-value> contained in
sbCi1.
Step 4.1.2.2. If v is less than lblil or v is greater than ub[il, then perform
raise-condition(<subscript-ranqe-condition>). Let ubp be
<bound-pair>:
<upper-bound>:
<extent-expression>:
v; ;
<lower-bound>:
<extent-expression>:
V.
Step 4.2. Replace cbpCi.1 by ubp.
Step 5.. Perform extract-slice-of-array(bpl,cbpl,n,sil) to obtain a tstorage-index-
list+,nsil. Replace sil by nsil.
Step 6. Let nbpl be a <bound-pair-list> with no components. For i=l,...,m, if sbCi1 has
<asterisk> then append a copy of the i'th component of bpl to nbpl. If nbpl has
no components then replace the <data-description> of cg by dd; otherwise replace
bpl by nbpl.
Step 7. Return cg.
212
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 231

7.6.8 EVALUATE-BY-NAME-PARTS-LIST
This operation takes a igeneration, and a <by-name-parts-list> and constructs a new
<generation+ containing all the parts specified by the <by-name-parts-list> in the order
of that list.
Operation: evaluate-by-name-parts-list(g,vr,d)
where g is a <generation+,
vr is a <variable-reference>,
d is a <declaration>.
result:a *generation>.
Step 1. Let aud be the <allocation-unit-designator, of g.
Letrg be
<generation+:
<evaluated-data-description>:
<data-description>:
<structure-data-description>:
<member-description-list>,mdl:;:
aud
<storage-index-list>,sil.
Step 2. Let m be the number of elements in the <by-name-parts-list>,bnpl, of vr. For
i=l,...,m perform Steps 2.1 through 2.3.
Step 2.1. Let bnp be the i'th element of bnpl. Let id1 be a copy of the <identifier-
list> of vr, if one exists; otherwise let id1 be an <identifier-list> with
no elements. Append the elements of the <identifier-list> of bnp to idl.
Step 2.2. Perform select-qualified-reference(g,idl,dl to obtain a <generation,,ng.
Step 2.3. Let dd be the <data-description> of the tevaluated-data-description+ of ng.
Let md be a <member-description>: dd. Append md to mdl. Append the
elements of the <storage-index-list+ of ng to sil.
Step 3. Return rg.
7.6.9 EVALUATE-DEFINED-REFERENCE
This operation evaluates a <variable-reference> for a <variable> whose <storage-type> is
<defined> and yields a <generation?.
Operation: evaluate-defined-referencefvr)
where vr is a <variable-reference>.
result: a <generation,.
Step 1. Let dp be the <declaration-designator> of the <variable-reference>,vr, let d be
the <declaration> designated by dp, and let dc be the <defined> component of d.
Perform find-block-state-of-declarationtdp) to obtain the <block-state3,bs.
Step 2. Let dde be the <defined-directory-entry+ in bs whose <identifier> is equal to
that of d. Let edd be the <evaluated-data-descriptions of dde.
Step 3. d has a <base-item> with a <variable-reference>,bvrl. Let bvr2 be a copy of
bvrl. If bvr2 contains a <subscript-list> with <isub> components then delete
the <subscript-list> in bvr2.
Step 4. Perform evaluate-variable-reference(bvr21 to obtain the fgeneration~,g.
step 5.
Case 5.1. dc has a <position> component.
Perform evaluate-string-overlay-defined-reference(vr,~d,g) to obtain the
Chapter 7: Storage and Assignment 213
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 232

Case 5.2.
Case 5.3.
fgeneration>,gd.
dc has a <subscript-list> with an <isub> component.
Perform evaluate-isub-defined-reference(vr,edd,g) to obtain the
fgeneration+,gd.
(Otherwise).
Perform check-simply-defined-reference(vr). If the value obtained is
<true,, then perform evaluate-simply-defined-reference(vr,edd,g) to obtain
the fgeneration+,gd. Otherwise perform evaluate-string-overlay-defined-
reference(vr,edd,g) to obtain the fgeneration+,gd.
Step 6. Return gd.
7.6.10 EVALUATE-SIMPLY-DEFINED-REFERENCE
This operation takes a simply-defined <variable-reference>, the fevaluated-data-
description+ associated with the variable and the *generation> being referenced and
constructs the fgeneration, that is the result of the reference.
Operation: evaluate-simply-defined-reference(vr.edd,g)
where vr is a <variable-reference>,
edd is an <evaluated-data-description,*
g is a egeneration+.
result: a <generation+.
Step 1. Let eddg be the <evaluated-data-description, of g.
Step 2. For every <extent-expression>,e contained in edd that is not contained in a
<parameter-descriptor> or <returns-descriptor>, let v be the *integer-value,
contained in e.
Case 2.1. e is contained in a <member-description> or an <area-size>.
v must be equal to the corresponding <integer-value of eddg.
Case 2.2. e is contained in a <lower-bound> and is not contained in a Unember-
description>.
V must be greater than or equal to the corresponding <integer-value+ of
eddg.
Case 2.3. (Otherwise).
v must be less than or equal to the corresponding <integer-value9 of eddg.
Step 3. If edd contains any <bound-pair>, perform adjust-bound-pairs(g,edd) to yield a
new <storage-index-list,,sil, corresponding to the <bound-pair>s of edd. Let gl
be the <generation+ constructed from a copy of the <allocation-unit-designator+
of g, edd, and sil.
Step 4. If has
refer~&e(gl.idl?dn),
<identifier-list>,idl, then perform select-qualified-
where d is the <declaration> designated by the
<declaration-designator> of vr, to obtain a <generation+. Replace gl with this
<generation*.
Step 5. If vr has a <by-name-parts-list>, perform evaluate-by-name-parts-list(g,vr,d) to
obtain a <generation+. Replace gl with this <generation+.
Step 6. If vr has a <subscript-list>,sbl, perform select-subscripted-reference(gl,sbl)
to obtain a fgeneration,. Replace gl with this *generation+.
Step 7. Return gl.
214
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 233

7.6.11 ADJUST-BOUND-PAIRS
This operation takes a <generation+ ancl modifies the <storage-index-list> to reflect the
<bound-pair& contained in an <evaluated-data-description>.
Operation: adjust-bound-pairs(g,edd)
where g is a <generation*,
edd is an <evaluated-data-description+.
result: a <storage-index-list+.
Step 1. Let eddg be the <evaluated-data-description9 of g and let csil be a copy of the
fstorage-index-list> of g.
step 2. If the <data-description> of eda immediately contains a <dimensioned-data-
description>,ddd, then perform Steps 2.1 through 2.4.
Step 2.1. Let dddg be the simply contained <dimensioned-data-description> of eddg.
Let bpl and bplg be, respectively, the <bound-pair-list>s of ddd and dddg.
Step 2.2. Let m be the number of elements in bpl. For i=l,...,m, the values being
chosen in any order, perform Steps 2.2.1 through 2.2.5.
Step 2.2.1. Let bpCi1 and bpg[il be the i'th element of, respectively, bpl and bplg.
Step 2.2.2. Let ubCi1 and ubgCi1 be the <integer-value> contained in the <upper-
bound> of, respectively, bp[il and bpg[il.
Step 2.2.3. Let IbCil and IbgCil be the <integer-value+ contained in the <lower-
bound> of, respectively, bplil and bpglil.
Step 2.2.4. If ub[il is greater than ubg[il then perform raise-
condition(<subscriptranqe-condition>).
Step 2.2.5. If lbCi.1 is less than lbglil then perform raise-
condition(<subscriptranqe-con>).
Step 2.3. Let dd be the <data-description> of the <element-data-description> of dddg.
Perform scalar-elements-of-data-description(dd) to obtain the integer n.
Step 2.4. Perform extract-slice-of-array(bpl,bplg,n,csil) to obtain a fstorage-index-
list+,nsil.
step 3. Return nsil.
7.6.12 EVALUATE-ISUB-DEFINED-REFERENCE
This operation takes an isub-defined <variable-reference>, the fevaluated-data-
description+ associated with the <variable> and the <generation> being referenced, and
constructs the <generation+ that is the result of the reference.
Operation: evaluate-isub-defined-reference(vr,edd,g)
where vr is a <variable-reference>,
edd is an <evaluated-data-description+,
4 is a <generation,.
result: a <generation>.
Step 1. Let add1 and bpl be, respectively, the immediately contained <dimensioned-data-
description> and its <bound-pair-list> in the <data-description> of edd. Let n
be the number of elements in bpl.
Step 2. Let dp and sbl be, respectively, the immediately contained cdeclaration-
designator> and <subscript-list> of vr. Let sbla be a <subscript-list>
containing a copy of the first n elements of sbl and let sblb be a <subscript-
list> containing a copy of the remaining elements, if any, of sbl.
Chapter 7: Storage and Assignment 215
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 234

Step 3. Choose, in any order, each element of sbla that does not immediately contain
<asterisk> and perform Steps 3.1 through 3.3.
step 3.1. Let e be the chosen element of sbla and let i be its ordinal in sbla.
Step 3.2. Perform evaluate-expression(e) and convert the result obtained to integer-
type of value v. Replace the <expression> component of the i'th element of
sbla by v.
Step 3.3. Let lb and ub be, respectively, the <lower-bound> and the <upper-bound> of
the i'th <bound-pair> of bpl. If v is less than the <integer-value+
contained in lb or greater than the <integer-value, contained in ub, perform
raise-condition(<subscriptranqe-condition>).
step 4. Construct a <subscript-list-list>, sbll, whose single element is a copy of sbla.
If sbla contains any <asterisk> component, then expand sbll by performing
expand-list-of-subscript-lists(sbll,eddl to obtain a new <subscript-list-list>,
and replace sbll with this.
step 5. Let d be the <declaration> designated by dp. For each element, esbl, of sbll
perform transform-subscript-list(esbl,dJ to obtain a <subscript-list>, and
replace esbl in sbll with this. This transforms each <subscript-list> of sbll
from a <subscript-list> that applies to the <data-description> of d into one
that applies to the <data-description> of the <declaration> designated by the
<declaration-designator> of the <variable-reference> of the <defined> component
of d.
step 6. Construct a new <storage-index-list+,nsil by performing Steps 6.1 and 6.2 for
each element of sbll chosen in left-to-right order.
Step 6.1. Let sblc be the chosen element of sbll. Perform select-subscripted-
reference(g,sblc) to obtain a fgeneration>,gl.
Step 6.2. Append the elements of the <storage-index-list> of gl to nsil.
step 7. Let bplc be a copy of the <bound-pair-list>,bpl. Delete each element of sbla
that is not <asterisk> and also delete the element of bplc with the same
ordinal. Let vrc be a copy of vr. Replace the copy of sbl in vrc by a
<subscript-list> constructed by appending the elements of a copy of sblb to a
copy of sbla.
Step 8. Let eddgc be a copy of the <evaluated-data-description+ of q. The <data-
description> of eddgc has an immediate <dimensioned-data-description> component;
let this be ddd2. Let eddc be a copy of edd and let its contained copy of dddl
be dddlc.
Case 8.1. As a result of Step 7, bplc no longer exists.
Step 8.1.1. Replace dddlc in eddc by the subtree of its <element-data-description>.
Step 8.1.2. Replace ddd2 in eddgc by the subtree of its <element-data-description>.
Case 8.2. ~Otherwise).
Replace the <bound-pair-list> of eadgc by bplc.
Step 9. Let g2 be a <generation> constructed from a copy of the fallocation-unit-
designator> of g, eddgc, and nsil. Perform check-simply-defined-reference(vrc).
The value obtained must be <true,. Perform evaluate-simply-defined-
reference(vrc,eddc,g2) to obtain a tgeneration>,g3.
Step 10. Return 93.
216
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 235

7.6.13 EXPAND-LIST-OF-SUBSCRIPT-LISTS
ThiS operation expands a <subscript-list-list> so that each element that is an
<asterisk>, meaning a cross-section, causes the generation of an appropriate number of
elements in the list with one element for each element of the cross-section.
Operation: expand-list-of-subscript-lists(sbll,edd)
where sbll is a <subscript-list-list>,
edd is an *evaluated-data-description+.
result: a <subscript-list-list>.
Step 1. Let csbll be a copy of sbll. Let bpl be the <bound-pair-list> of the
<dimensioned-data-description> that is simply contained by edd. Perform Steps
1.1 through 1.3 until no <asterisk> remains in csbll.
Step 1.1. Choose an element, sbl, of csbll, such that one of the elements of sbl is
<asterisk>. Let an <asterisk> be the i'th element of sbl.
Step 1.2. Let bp be the i'th element of bpl. bp has the two components <upper-
bound>,ub, and <lower-bound>,lb.
Step 1.3. Replace sbl by tub-lb+11 copies of sbl modified such that the j'th copy has
its <asterisk> component in the i'th position replaced by an <expression>
containing the ginteger-value,, (lb-l+j).
Step 2. Return csbll.
7.6.14 TRANSFORM-SUBSCRIPT-LIST
This operation takes a <subscript-list> and performs the transformation specified by
isub-defining to generate a new <subscript-list>.
Operation: transform-subscript-list(sbl,d)
where sbl is a <subscript-list>,
d is a <declaration>.
result: a <subscript-list>.
Step 1. In the <variable-reference> of the <defined> component of d there is a
<subscript-list>. Let csbl be a copy of this <subscript-list>.
Step 2. For each <isub>.ic, of csbl perform Step 2.1.
Step 2.1. Let i be the value of the <integer> component of ic. i must be greater than
zero and less than or equal to the number of elements in sbl. Replace ic in
csbl by the finteger-value, of the i'th element of sbl.
Step 3. For each <expression>,e of csbl perform Step 3.1.
Step 3.1. Perform evaluate-expression-to-integer(e) to obtain an <integer-value,,iv,
and replace e in csbl by iv.
Step 4. Return csbl.
Chapter 7: Storage and Assignment 217
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 236

7.6.15 EVALUATE-STRING-OVERLAY-DEFINED-REFERENCE
This operation takes a <variable-reference> for a string-overlay-defined <variable>, the
<evaluated-data-description, associated with the <variable>, and the base <generation9
being referenced and constructs the <generation+ that is the result of the reference.
Operation: evaluate-strinq-overlay-defined-reference(~,edd,g)
step 1.
where vr is a <variable-reference>,
edd is an <evaluated-data-description9,
9 is a <generation+.
result: a <generation>.
g must be a connected <generations. The scalar-elements of g and the scalar-
elements described by edd must each contain <unaliqned> and
either (1)
or (2)
Step 2. Let d be the <declaration> designated by the <declaration-designator> of vr. If
the <defined> component of d contains <position>, then perform evaluate-
expression(e), where e is the <expression> of <position>, and convert the result
to integer-type of value p. Otherwise, let p be 1. The value of p must be
greater than or equal to 1.
Step 3. Perform overlay-strings(edd,grp) to obtain a fstorage-index-list+.nsil. Let gl
be the <generation> comprising the <allocation-unit-designator> of g, a copy of
edd, and nsil.
Step 4. If vr immediately contains an <identifier-list>,idl, then perform select-
qualified-reference(g1.idl.d) to obtain a fgeneration+,gl.
Step 5. If vr immediately contains a <by-name-parts-list>, then perform evaluate-by-
name-parts-list(g,vr,d) to obtain a fgeneration+,gl.
Step 6. If vr immediately contains a <subscript-list>,sbl, then perform select-
subscripted-reference(g,sblJ to obtain a tgeneration>,gl.
Step 7. Return gl.
each contain <nonvarying> and <m>,
each contain either <pictured>, or <nonvarvinq> and <character>.
7.6.16 CHECK-SIMPLY-DEFINED-REFERENCE
This operation checks that the relationship between the <declaration> of a <variable-
reference> to a variable that is <defined> and the <declaration> referenced in the <base-
item> is suitable for evaluation as a simply-defined reference.
Operation: check-simply-defined-referencefvrl
where vr is a <variable-reference>.
result: <true+ or *false,.
Step 1. Let d be the <declaration> designated by the <declaration-designator> of vr.
Let dd be a copy of the <data-description> immediately contained in the
<variable> of d. Replace each <bound-pair> in dd by <bound-pair>: <asterisk>;,
each <maximum-length> by <maximum-length>: <asterisk>;, and each <area-size> by
<area-size>: <asterisk>. -- Let pd be
<parameter-descriptor>:
dd.
Step 2. Let bvr be the <variable-reference> of the <defined> component of dd.
Step 3. Perform test-matching(bvr,pd). If the value obtained is <true, then return
<es ; otherwise return <false,.
218

218

218
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 237

7.6.17 EXTRACT-SLICE-OF-ARRAY
This operation selects the part of a fstoraqe-index-list, or *basic-value-list, according
to a <bound-pair-list>.
Operation: extract-slice-of-arrav(obpl,nbplrn,sil)
where obpl is a <bound-pair-list>,
nbpl is a <bound-pair-list>,
n is an integer.
sil is a <storage-index-list+ or <basic-value-list,.
result: a <storage-index-list+ or <basic-value-list>.
Step 1.
Step 2.
Step 3.
Step 4.
For each <bound-pair>,obpCil, ,...,m, i=l in obpl let oublil and olbCi1 be the
<integer-value+ components of the <upper-bound> and <lower-bound>, respectively,
of obpCi1.
For each <bound-pair>,nbpCil, ,...,m, i=l in nbpl let nublil and nlbCi3 be the
<integer-value+ components of the <upper-bound> and <lower-bound>, respectively,
of nbp[il.
Let n2 be
m
n+ r (nub[il-nlb[il+l).
i=l
If sil is a <storage-index-list+ then let nsil be a <storage-index-list, with n2
<storage-index+ elements. Otherwise, let nsil be a <basic-value-list, with n2
elements of the form <basic-value+: <undefined+.
For each set of integers s[i3, i=l,...,m, such that nlbCi1 < s[il 5 nub[il,
perform Steps 4.1 and 4.2.
Step 4.1. Let
m-l m
k = l+n+(stml-olbCml+ c ((slil-olbCil)+ m (oubCjl-olbijl+l)))
i=l j=i+l
and let
m-l m
k2 = l+n*(sCml-nlbCml+ c ((slil-nlbCil)* n (nubCjl-nlbCjl+l))).
i=l j=i+l
Step 4.2. Replace nsil[k2+jl by sil[k+jl for j=O,...,n-1.
Step 5. Return nsil.
Chapter 7: Storage and Assignment 219
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 238

7.7 Reference to Named Constant
7.7.Y. EVALUATE-NAMEDCONSTANT-REFERENCE
This operation obtains the value of a <named-constant-reference>.
Operation: evaluate-named-constant-referencetncr)
where ncr is a <named-constant-reference>.
result: an <aggregate-value*.
step 1. Let d be the <declaration> designated by the <declaration-designator> of ncr and
let nc be a copy of the Cnamed-constant> component of d.
step 2.
Case 2.1. nc contains a <bound-pair-list>,bpl.
Step 2.1-l. For each <expression>,e of bpl, chosen in any order, perform evaluate-
expression-to-integer(e) to obtain the <integer-value+,i and replace e
by i.
Step 2.1.2. For each <bound-pair>,bp[il in bpP, i=l,...,m, let ubCi1 and lb[il be
the *integer-value, components of the <upper-bound> and <lower-bound>,
respectively, of bpCi1.
Step 2.1.3. Let n be the integer
m
n (ub[il-lbCil+l),
i=l
and let svl be a *basic-value-list> consisting of n <undefined+
elements.
Case 2.2. (Otherwise).
Step 2.2.1. Let n be 1, and let svl be a <basic-value-list+ consisting of one
<undefined, element.
Step 3.
Case 3.1. nc has an <entry> component and d has <external>.
Let ep be the <entry-point> that is an element of the <entry-or-executable-
unit-list> that is immediately contained in a <procedure> of the <abstract-
external-procedure-list> and that has a <statement-name> with an
<identifier> that is equal to the <identifier> of d. Let epd be an <entry-
point-designator> designating ep and replace the *undefined+ component of
svl by epd.
Case 3.2. nc has an <entry> component and d has <internal>.
Step 3.2.1. Perform find-block-state-of-declaration(d) to obtain the fblock-
state+,bs. Let pl be the <procedure-list> of the <begin-block> or
<procedure> that simply contains d.
Step 3.2.2. For each <entry-point>,ep that is simply contained in pl and whose
<statement-name> has an <identifier> that is equal to the <identifier>
of d perform Steps 3.2.2.X and 3.2.2.2.
step 3.2.2.1. Let epd be an <entry-point-designator> designating ep and let bsd
be a <block-state-designator> designating bs. Let ev be an
<entry-value+: epd bsd.
220
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 239

Step 3.2.2.2.
Case 3.2.2.2.1. ep has a <signed-integer-list>,sl.
Let sCi1 be the i'th element of sl. Let sn be the integer
m-l
l+sCml-lb[ml+ c (s[il-lb[il)*a[il
i=l
where
m
aCi1 = n (ubljl-lb[jl+l).
j=i+l
Replace the <undefined, component of the sn'th element of
svl by ev.
Case 3.2.2.2.2. (Otherwise).
Replace the <undefined+ component of svl by ev.
Case 3.3. nc has a <file> component.
Perform search-file-directory(ncr,svl) to obtain a <basic-value-list+,bvl.
Replace svl by bvl.
Case 3.4. nc has a <format+ component.
Step 3.4.1. Perform find-block-state-of-declaration(d) to obtain the fblock-
state+,bs. Let fsl be the <format-statement-list> component of the
<begin-block> or <procedure> that simply contains d.
Step 3.4.2. For each <format-statement>,fs, element of fsl that has a Gtatement-
name> whose <identifier> is equal to the <identifier> of d, perform
Steps 3.4.2-l and 3.4.2.2.
Step 3.4.2.1. Let fsd be a <format-statement-designator> designating fs and let
bsd be a <block-state-designator9 designating bs. Let fv be a
<format-value>: fsd bsd.
Step 3.4.2.2.
Case 3.4.2.2.1. The <statement-name> of fs has a <signed-integer-list>,sl.
Let s[il be the i'*th element of sl. Let sn be the integer
m-l
l+s[ml-lb[ml+ c (sCil-lbCi1 )*aCil
i=l
where
m
a[il = n (ubljl-lb[jl+l).
j=i+l
Replace the <undefined, component of the sn'th element of
svl by fv.
Case 3.4.2.2.2. (Otherwise).
Replace the fundefined, component of svl by fv.
Chapter 7: 'Storage and Assignment 221
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 240

Case 3.5. nc has a <label> component.
Step 3.5.1. Perform find-block-state-of-declaration(d) to obtain the fblock-
state+.bs. Let eul be the <executable-unit-list> or <entry-or-
executable-unit-list> of the <begin-block> or <procedure>, respectively,
that simply contains d.
Step 3.5.2.. For each <executable-unit>,eu, component of eul that has a <statement-
name> whose <identifier> is equal to the <identifier> of d, perform
Steps 3.5.2. 1 and 3.5.2.2.
step 3.5.2.1. Let eud be an <executable-unit-designator> designating eu, and
let bsd be a &lock-state-designator+ designating bs. Let Iv be
a <label-value+: eud bsd.
Step 3.5.2.2.
Case 3.5.2.2.1. The <statement-name> of eu has a <signed-integer-list>,sl.
Let sfil be the i'th element of sl. Let sn be the integer
m-l
l+sfml-lb[ml+ c (sfil-lb[ill*aCil
i=l
where
m
alil = n (ubC jl-lb[jl+l).
j=i+l
Replace the <undefined+ component of the sn'th element of
svl by Iv.
Case 3.5.2.2.2. (Otherwise).
Replace the *undefined+ component of svl by Iv.
Step 4.
Case 4.1. nc contains <bound-pair-list>,bpl.
Let agt be
<aggregate-type+:
fdimensioned-aggregate-type+:
*element-aggregate-type,:
*scalar>;
bpl.
Case 4.2. (Otherwise).
Let agt be <aggregate-type+: <scalar>.
Step 5. Let av be <aggregate-value>: agt svl.
Step 6. If ncr has a <subscript-list> then perform Steps 6.1 through 6.4.
step 6.1. Let m, 1bCil and ubCi1, i=l,... ,m, be as determined in Step 2. Let bpl2 be
a <bound-pair-list> with m elements of the form
<bound-pair>:
<upper-bound>,ubZLil
<lower-bound>,lb21il;
for i=l,...,m. Perform Step 6.6.X for i=l,...,m. taken in any order.
222
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 241

Step 6.1.1.
Case 6.1.1.1. The i'th element of the <subscript-list> in ncr has an
<asterisk>.
Let lbZ[il and ub2Cil be, respectively, lb[il and ubCi3.
Case 6.1.1.2. The i'th element of the <subscript-list> in ncr has an
<expression>,exp.
Perform evaluate-expression-to-integercexpl to obtain the
<integer-value9,v and let lb2iil and ub2Ci.l be the integer in v.
If v is less than IbCil or greater than ub[il perform raise-
condition(<subscriptranqe-condition>).
Step 6.2. Perform extract-slice-of-array(bpl,bpl2,l,svl) to obtain a fbasic-value-
list,,bvl2.
step 6.3. Replace the <basic-value-list, in av by a copy of bv12.
step 6.4.
Case 6.4.1. Each element of the <subscript-list> in ncr has <expression>.
Replace the <aggregate-type+ of av by <aggregate-type+: *scalar>.
Case 6.4.2. (Otherwise).
Delete from the <bound-pair-list> of av the elements which correspond to
elements of the <subscript-list> of ncr which have <expression>.
Step 7. The <aggregate-values av must not contain <undefined>. Return av.
7.7.2 SEARCH-FILE-DIRECTORY
This operation searches the <file-directory+ for the entry or entries that correspond to
the <declaration> referred to in a <named-constant-reference>.
Operation: search-file-directorY(ncr,svl)
where ncr is a <named-constant-reference>,
svl is a <basic-value-list+.
result: a <basic-value-list+.
Step 1. Let bvl be a copy of svl. Let d be the <declaration> designated by the
<declaration-designator> of ncr. Perform Steps 1.1 through 1.2 until bvl
contains no <undefined, elements.
Step 1.1. Search the <file-directory, for a <file-directory-entry+,e, whose
<identifier> is identical with the <identifier> of d and which has the
component <external> if d has the component <external>; otherwise it has a
<declaration-designator> component that designates d.
Step 1.2. Let fid be the <file-information-designator, in e and let bv be a tbasic-
value*: <file-value,: fid. If e has a <subscript-value-list+, replace the
element of bvl denoted by this <subscript-value-list+ by bv. Otherwise,
replace the single element of bvl by bv.
Step 2. Return bvl.
Chapter 7: Storage and Assignment 223
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 242

Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 243

Chapter 8: Input/Output
8.0 Introduction
This Chapter describes the abstract structure of a <dataset> and the transmission of data
between a <dataset> and the iallocated-storage, of the tmachine-state, directed by PL/I
programs as introduced in Chapter 5. The main Sections are concerned with the following:
8.1 Datasets and the interface between them and the program
8.2 Files
8.3 Conditions applicable to I/O operations
8.4 Evaluation of a <file-option>
8.5 File opening and closing
8.6 Statements performing record transmission
8.7 Statements performing stream transmission, and a description of how data may be
organized in the data stream
8.1 Datasets
A <dataset+ is an abstract model of a physical dataset. Its properties and structure are
those which are necessary for a correct interpretation of a PWI program. The concrete-
representation of a fdataset, is implementation-defined. <alpha+ and fomeqa, are end-
markers for fdataset9.s that have a sequence.
8.1.1 RECORD DATASETS
A <record-dataset, may contain discrete <record%; <record-dataset% without any
frecord+s are permitted.
The "size" (see Section 8.6.6.11) is an implementation-defined function of the
<evaluated-data-description+ of a <record+. It is checked whenever the <record% is
transmitted, and under implementation-defined circumstances may cause the raising of the
<record-condition>.
tkey+s are a means of identifying particular <record%. Within a fdataset,, fkeyss are
unique.
8.1.2 STREAM DATASETS
A <stream-item+ is either a {symbol) or a control item which indicates (in an
implementation-defined way) a line or page break or that the following {symbol)s are to
be sent to the same line as the preceding ones. The <stream-items fpaqemark, and
Gcarriaqe-return, are not allowed in a <stream-item-list> associated with a file open for
stream input and may only appear in a <stream-item-list, associated with a file open for
print stream output.
Chapter 8: Input/Output 225
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 244

8.2 Files
Whenever a PWI program requires to access a <dataset%, it does so by naming a <file-
option>. This <file-option> is evaluated yielding a ffile-value+,fv, which designates a
<file-information+,fi. In order successfully to access a <dataset>, fi must contain
<open+, in which case it also contains a <file-opening+ with a tdataset-designator9,dsd.
dsd designates a fdataset,, which is thus accessed by naming the original <file-option>.
8.2.1 RECORD FILES
The <current-position+ of a <file-opening* containing <record> contains either the
<designator> of a component of a <record-dataset, or <undefined+. The designated node may
be a *record+, a <keyed-recor-, *alpha+, or fomeqa9. One of the actions in executing a
statement may be to update the *current-position>.
The fdelete-flaq+, which may be present in a <file-opening+ when <record> appears, is an
indication that certain actions must not be performed. (See, for example, the operation
delete.)
The fallocated-buffer> in a *file-opening> contains a <generation, allocated by a <read-
statement> or a <locate-statement> with the <pointer-set-option>. In the case of the
<locate-statement> it may also contain a <key, to be associated with the frecord, to be
associated with the <allocated-buffer+.
8.2.2 STREAM FILES
The icurrent-position, of a <file-opening> containing <stream> contains either the
<designator+ of a component of a <stream-dataset, or <undefined>. The designated node
may be a <stream-item+, falpha,, or fomeqas. One of the actions in executing a statement
may be to update the *current-position%.
The <page-number+ component of a *file-opening+ is applicable only when the tfile-
opening, also contains <stream> and <print>.
The <first-comma+ component of a *file-opening> is applicable only when the ffile-
opening9 also contains <stream> and <input>.
8.3 I/O Conditions
In describing the execution of each input/output statement, the circumstances under which
any <io-condition>, except <transmit-condition>, may be raised, are indicated.
8.3.1 RAISE-IO-CONDITION
Operation: raise-io-condition(cond,fv,str,int)
where cond is <endfile-condition>, <endpaqe-condition>, <key-condition>,
<name-condition>, <record-conditio0, <transmit-condition>,
<undefinedfile-condition>, or <conversion-condition>,
fv is a [*file-value,],
str is a [<character-string-value91,
int is an [<integer-value+].
Step 1. If fv is a ffile-value> then let fi be the <file-information, designated by fv.
226
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 245

step 2.
Case 2.1.
Case 2.2.
step 3.
Case 3.1.
Case 3.2.
Case 3.3.
Case 3.4.
cond is not <conversion-condition>.
If fv is <absent+ then perform raise-condition(<error-condition>).
Let eioc be an
*evaluated-io-condition,:
<io-condition>:
cond;
fv.
cond is <conversion-condition>.
Let eioc be <conversion-condition>.
cond is <name-condition>.
Let cbifs be a <condition-bif-value-list+: tcondition-bif-value+: tonfield-
value,: str.
cond is <conversion-condition>.
Let cbifs be a <condition-bif-value-list, simply containing fonsource-
value,: str; and *onchar-value,: int. If fv is a <file-value, then let fn
be the <character-string-value9 in the *filename, in fi and attach an
tonfile-value,: fn; to cbifs.
cond is <transmit-condition>, <record-condition> or <key-condition>, and str
is present.
Let cbifs be a fcondition-bif-value-list+: <condition-bif-value+: fonkey-
value>: str.
(Otherwise).
Let cbifs be <absent>.
Step 4. Perform raise-condition(eioc,cbifs).
step 5.. If cond is <key-condition> or <endfile-condition> then perform exit-from-io(fv1.
8.4 Evaluate-file-option
The evaluation of a <file-option> may be performed during input/output statements. This
operation is also used to evaluate <copy-option%5 and options of <on-statement>s,
<signal-statement>s, and <revert-statement>s containing <named-io-condition%.
Operation: evaluate-file-option
where vr is a <value-reference>.
result: a *Se-value>.
step 1. Perform evaluate-value-referencetvr) to obtain an <aggregate-valueS,av. Return
the *file-values in av.
chapter 8: Input/Output 227
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 246

8.5 File Opening and Closing
Opening a file causes a <file-opening+ to be attached to the <file-informatioti and a
<dataset+ to be associated with the file. Closing a file removes the <file-opening> and
dissociates the *dataset, from the file.
8.5.1 THE OPEN STATEMENT
8.5.1.1 Execute-open-statement
Operation: execute-open-statement(os)
where OS is an <open-statement>.
Step 1. For each <single-opening>,sgo in os, in order, perform execute-single-
opening(sg0).
Step 2. Perform normal-sequence.
8.5.1.2 Execute-sinqle-openinq
Operation: execute-sinqle-openinq(sg0)
where sgo is a <single-opening>.
Step 1. Let efdl be an <evaluated-file-description-list+ with no components.
Step 2. Perform Steps 2.1 to 2.5 in any order.
Step 2.1. Let fo be the <value-reference> in the <file-option> in sgo. Perform
evaluate-file-option to obtain a <file-value+,fv.
Step 2.2. If SF contains a <title-option>,tto, then perform evaluate-title-
option(tto) to obtain an <evaluated-title+,et, and append fevaluated-file-
description+: et; to efdl.
Step 2.3. If sgo contains a <tab-option>,tbo, then perform evaluate-tab-option(tbo) to
obtain an ievaluated-tab-option,reto, and append fevaluated-file-
description+: eto; to efdl.
Step 2.4. If sgo contains a <linesize-option>, then let lzo be its <expression>,
perform evaluate-expression-to-integer(lzo) to obtain an *integer-value+,lz,
which must be greater than zero, and append <evaluated-file-description+:
<evaluated-linesize,: lz;: to efdl.
Step 2.5. If sgo contains a <pagesize-option>, then let pzo be its <expression>,
perform evaluate-expression-to-integertpzo) to obtain an <integer-value*,pz,
which must be greater than zero, and append <evaluated-file-description+:
<evaluated-pagesize,: pz;; to efdl.
Step 3. Let fi be the <file-information+ designated by fv. If fi contains <open+ then
terminate this operation.
step 4. For each immediate component, tn, of SW which is a terminal node, append
*evaluated-file-description>: tn; to efdl.
Step 5. Perform open(fv,efdl) to obtain res.
Step 6. If res is <fail+ then perform raise-io-condition(<undefinedfile-condition>,fv).
228
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 247

8.5.1.3 open
Operation: open(fv,efdl)
Step 1.
Step 2.
Step 3.
Step 4.
Step 5.
Step 6.
Step 7.
where fv is a tfile-value,,
efdl is an <evaluated-file-description-list+.
result: tsucceed, or <fail,.
Let fi be the <file-information+ designated by fv. For each terminal node, tn,
of the <file-description> of fi, append <evaluated-file-description3: tn; to
efdl.
Augment efdl with implied attributes as follows: for each terminal node in efdl
which occurs under "Attribute" in the table below, append to efdl trees
containing the corresponding 'Implied Attributes" categories.
.----------------------------------------
Attribute Implied Attributes
<record> <keyed>
<record>
<stream> <output>
<sequential> <record>
<update> <record> I .----------------------mm-----------------,
Augment efdl with default attributes as follows: when efdl does not contain any
of the "Alternative Attributes" in a line of the table below, append to efdl a
tree containing the corresponding "Default" category.
r-----------------‘------------------------------------------,
Alternative Attributes 1 Default I
i <stream> 1 <record>
1 <a I <output> I <update>
i <stream>
I <input> I L-----------------------------------------------------------~
If efdl contains <record> but does not contain either <sequential> or <direct>,
append <sequential> to efdl.
If the <filename, in fi contains in order precisely the $symbol)s of the word
SYSPRINT, and the <file-directory-entry+ whose <file-information-designator+
designates fi has <external>, and efdl contains <stream> and <output>, then
append <evaluated-file-description+: <print>; to efdl.
If both <stream>, and <record>, or any two of <input>, <output>, and <update>, or
both <direct> and <sequential> are contained in efdl, then return t-9.
If efdl contains an <evaluated-linesizes then it must contain <stream> and
<output>. If efdl contains <stream> and <output> but not an fevaluated-
linesize, then append l evalumile-description,: <evaluated-linesize,: an
implementation-defined <integer-value,:; to efdl.
If efdl contains an <evaluated-pagesize, then it must contain <print>. If efdl
contains <print> but not an <evaluated-pagesize then append tevaluated-file-
description+: *evaluated-pagesize,: an implementation-defined <integer-value+;;
to efdl.
If efdl contains an <evaluated-tab-option3 then it must contain <print>. If
efdl contains <print> but not an <evaluated-tab-option+ then append *evaluated-
file-description+: <evaluated-tab-option+: an implementation-defined tinteger-
value-list+;; to efdl.
If efdl does not contain an <evaluated-title+ then let fn be the *filename> in
fi and perform evaluate-filename to obtain an <evaluated-title3,t. and
append *evaluated-file-description,: t; to efdl.
Chapter 8: Input/Output 229
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 248

Step 8. Attempt to find, among the <datasetH of the <machine-state+ (if any). a
fdataset+,ds, the' <character-string-value+ of the fdataset-name+ of which
matches the *character-string-value of the <evaluated-title, in efdl in an
implementation-defined manner. If the attempt fails, then return tfail>.
If efdl contains <record> then ds must contain a Qrecord-datasets.
If efdl contains <stream> then ds must contain a &stream-datasee.
If efal contains <sequential> and <keyed> then ds must contain a *keyed-
sequential-dataset,.
If efdl contains <sequential> and <record> but not <keyed> then ds must contain
a <sequential-datasets.
If efdl contains <direct> then ds must contain a 4keyed-dataset+.
If efdl contains <stream> and <input> then ds must not contain any (paqemark, or
fcarriaqe-return*.
If efal contains <record> and <keyed> then ds must not contain two distinct
Qceyed-record% whose tkey+s are equal.
Step 9. Attach to fi a
<file-openinqs,fo:
fdataset-designator,
<complete-file-description+:
efal.
The tdataset-designator, designates ds, which has been associated with fv in
Step 8. If efdl contains <record> then attach a <delete-flaq+ to f0. If efdl
contains <print> then attach a <page-number% with 1 to fo. If efdl contains
<stream> and <input> then attach *first-conrae: <z%; to fo.
Step 10.
Case 10.1. efdl contains <direct>.
Attach to fo a <current-position+: <undefined+.
Case 10.2. efdl contains <output> and not <direct>, and ds contains a *record-
list+,rl, a &eyed-record-list%,rl,, or a *stream-item-list+,rl.
Attach to fo a <current-position% designating the last ixunediate
component of rl.
Case 10.3. (Otherwise).
Attach to fo a *current-position% designating the <alpha+ in ds.
Step 11. Set the <open-state+ in fi to contain <open*.
Step 12. Return tsucceed,.
8.5.1.4 Evaluate-tab-option
Operation: evaluate-tab-option(tbo)
where tbo is a <tab-option>.
result: an <evaluated-tab-options.
Step 1. For each <expression>,e in tbo, in any order, perform evaluate-expression-to-
integer(e) to obtain an tinteger-value+. Let il be an *integer-value-list,
containing these <integer-value% in the same order as their original
<expression>s in tbo.
230
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 249

Step 2. The *integer-value% in il must each be greater than zero, and the list must be
in ascending order.
Step 3. Return <evaluated-tab-option>: il.
8.5.1.5 Evaluate-title-option
Operation: evaluate-title-option(t)
where t is a <title-option>.
result: an <evaluated-title>.
Step 1. Let e be the <expression> in t. Perform evaluate-expression(e) to obtain an
<aggregate-value9,av. Let dd be the <data-description> immediately contained in
e.
step 2. Let sdt be the <data-type> in dd and let tdt be a <data-type> containing
<character>, <nonvarvinq> and an implementation-defined <maximum-length>. Let bv
be the *basic-value, in av. Perform convert(tdt,sdt,bv) to obtain a fcharacter-
string-value>,csv.
Step 3. Return *evaluated-title,: csv.
8.5.1.6 Evaluate-filename
Operation: evaluate-filename
where fn is a <filename+.
result: an <evaluated-title>.
Step 1. Let s be the <character-string-value+ in fn and let m be the number of
<character-value% in s. Let n be the implementation-defined maximum length of
the <evaluated-title>.
Step 2.
Case 2.1. n = 0.
Let csv be *character-string-value+: <null-character-strinq>.
Case 2.2. m 2 n > 0.
Let csv be <character-string-value+: l character-value-list,; where the
length of <character-value-list+ is n and where the i'th <character-value>
is the same as the i'th <character-value+ in s, i=l,...,n.
Case 2.3. m < n.
Let csv be <character-string-value9: icharacter-value-list>; where the
length of <character-value-list+ is n and where the i'th <character-value>
is the same as the i'th <character-value+ in s for lsiln, and where the
remaining &character-value% have 13s.
Step 3. Return *evaluated-title,: csv.
Chapter 8: Input/Output 231
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 250

8.5.2 THE CLOSE STATEMENT
8.5.2.1 Execute-close-statement
Operation: execute-close-statement(cs)
where cs is a <close-statement>.
step 1. For each <single-closing>,sc in cs, in order, perform execute-single-
closing(sc).
step 2. Perform normal-sequence.
8.5.2.2 Execute-sinqle-closing
Operation: execute-single-closing(sc)
where SC is a <single-closing>.
Step 1,. Let fo be the <value-reference> in the <file-option> in SC. Perform evaluate-
file-optionffo) to obtain a <file-value+,fv.
Step 2. If the <file-information, designated by fv contains t-9 then perform
close(fv).
8.5.2.3 Close
Operation: close(fv)
where fv is a ffile-value9W
Step 1. Let fi be the <file-information, designated by fv, and let ds be the <dataset+
designated by the fdataset-designator, in fi.
Step 2. If fi contains <output> and an <allocated-buffer+,abuf containing the
fgeneratior-&,g then perform Steps 2.1 through 2.3.
Step 2.1. If abuf contains a 4key9, then let k be a copy of that <key+; otherwise k is
<absent+. Perform construct-record(g,k) to obtain r.
Step 2.2. If k is a they+ then if there is a Frey, in ds equal to k, or if k is
unacceptable to the implementation, then:
Case 2.2.1. This operation is preceded in its *operation-list, by an <operation, for
execute-single-closing.
Perform raise-io-condition(<key-condition>,fv,csv), where csv is the
<character-string-value+ in k.
Case 2.2.2. This operation is preceded in its <operation-list+ by an <operation, for
program-epilogue+
Perform some implementation-defined action and go to step 4.
Step 2.3. Perform insert-record(r,fv).
Step 3,. If abuf is present then perform free(g) and delete abuf.
Step 4. Delete the tfile-opening9 in fi, and set the fopen-state, in fi to contain
*closed*.
232
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 251

8.6 The Record I/O Statements
The record I/O statements perform data transmission to and from <record-datasel9s.
Several of the record I/O statements use common operations. These are described in
Section 8.6.6. Several local variables are used in Section 8.6 in a consistent manner:
abuf
d
ds
edd
eio
eko
epso
efdl
fi
fn
fv
9
int
k
kr
Pas
r
for *allocated-buffers
for <declaration>
for <record-dataset,
for <evaluated-data-description9
for *evaluated-into-option,
for <evaluated-keyto-option,
for <evaluated-pointer-set-option,
for <evaluated-file-description-list+
for <file-information>
for *filename+
for <file-value*
for <generation+
for iinteger-value>
for <key+
for <keyed-record> or <record,
for <current-position,
for <record+
8.6.1 THE REAL STATEMENT
Purpose: The <read-statement> causes a firecord, to be transmitted from a <record-dataset,
to a target <generation* or an <allocated-buffer+.
8.6-1.1 Execute-read-statement
<evaluated-read-statement,::= ffile-value,
Ifevaluated-into-option+ 1
<evaluated-pointer-set-option> 1
*evaluated-ignore-option,)
[<key+ 1 tevaluated-keyto-option,3
Operation: execute-read-statement(rs1
where rs is a <read-statement>.
Step 1. Let ers be an <evaluated-read-statement+ without subnodes.
Step 2. Perform Steps 2.1 through 2.6 in any order.
Step 2.1. Let f be the immediate component of the <file-option> in rs. Perform
evaluate-file-option(f) to obtain a <file-value9,fv. Attach fv to ers.
Step 2.2. If rs contains an <into-option>,ito, perform evaluate-into-option(ito1 to
obtain an *evaluated-into-option,reito and attach eito to ers.
Step 2.3. If rs contains a <pointer-set-option>,pso, perform evaluate-pointer-set-
option(pso) to obtain an <evaluated-pointer-set-option,.epso and attach epso
to ers.
Step 2.4. If rs contains an <ignore-option>,igo, perform evaluate-ignore-option(igo1
to obtain an *evaluated-ignore-option,reigo and attach eigo to ers.
Step 2.5. If rs contains a <key-option>,ko, perform evaluate-key-option to obtain
a <key+,k and attach k to ers.
Step 2.6. If rs contains a <keyto-option>, kto, perform evaluate-keyto-option(kto1 to
obtain an <evaluated-keyto-option,,ekto and attach ekto to ers.
step 3. If the 4file-information+,fi, designated by fv contains t- then go to Step
5.
Chipter 8: Input/Output 233
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 252

step 4.
Step 4.1. Let efdl be an <evaluated-file-description-list, containing <record>.
Step 4.2. If fi does not contain <update> then attach <evaluated-file-description+:
<input>; to efdl.
Step 4.3. Perform open(fv,efdl) to obtain rf. If rf is <fail+ then perform raise-io-
condition(<undefinedfile-condition>,fv). If, on normal return, fi contains
<closed+ then perform raise-condition(<error-condition>).
Step 5. fi must contain:
<record>;
<m or <update>;
if ers contains an <evaluated-ignore-option, or if ers
does not contain a *key+, then <sequential>;
if ers contains a <key> or an <evaluated-keyto-option,. then <keyed>.
step 6. Perform readcers).
Step 7. Perform normal-sequence.
8.6.1.2 Read
Operation: readcers)
where ers is an *evaluated-read-statement9.
Step 1. Let fi be the tfile-information+ designated by the <file-value+,fv, in ers.
must contain t-9. If ers contains a <key+, then let k be this <key+ and 1::
kl be the immediate component of k. Otherwise let k and kl be iabsent,.
Step 2. If fi contains an <allocated-buffer,,abuf, then perform free(g), where g is the
*generation9 in abuf, and delete abuf from fi.
step 3. Perform position-filefers).
Step 4. If ers does not contain an <evaluated-keyto-option,,ekor then go to step 5.
Otherwise let ns be the number of {symbol? subnodes of the fcharacter-string-
value+,csvk in the *key+ in the <keyed-record+ designated by the *current-
position, in fi. Let dd be a <data-description> containing <character>,
<maximum-length> containing ns, and <nonvarvinq>. Let avk be an faggregate-
value, containing csvk. Let ekoet be the <evaluated-target+ in eko.
Step 5.
Case 5.1. ers contains an <evaluated-into-option+,eio.
Let r be the <record, or <keyed-record+ designated by the tcurrent-position9
in fi.
Step 5.1.1. Let eddl be the <evaluated-data-description, in r and edd2 be the
<evaluated-data-description> in eio. Perform evaluate-sizeteddl) to
obtain an <integer-value4,intl, and perform evaluate-size(edd2) to
obtain an *integer-value>,int2.
Step 5.1.2. Let g be the *generation, in eio.
Case 5.1.2.1. intl is equal to int2.
Perform Steps 5.1.2.1.1 and 5.1.2.1.2 in either order.
Step 5.1.2.1.1. eddl and edd2 must be equal. Perform set-storage(g,v), where
v is the <basic-value-list+ in r.
Step 5.1.2.1.2. If ers contains an fevaluated-keyto-option, then perform
assign(ekoet,avk,dd).
234

234
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 253

case 5.1.2.2. (Otherwise).
Step 5.1.2.2.1. Let nsi be the number of <storage-index% in g. Let undef be
a <basic-value-list, containing <basic-value,: fundefined,;
nsi times. Perform Steps 5.1.2.2.1.1 and 5.1.2.2.1.2 in
either order.
step 5.1.2.2.1.1.
step 5.1.2.2.1.2.
Perform set-storage(g,undef).
If ers contains an <evaluated-keyto-option, then
perform assign(ekoet,avk,ddl.
Step 5.1.2.2.2. Perform raise-io-condition(<record-condition>,fv,klJ.
Case 5.2. ers has an <evaluated-pointer-set-option+,epso.
Perform Steps 5.2.1 through 5.2.5 in any order such that Step 5.2.1 precedes
Step 5.2.2, Step 5.2.3, and Step 5.2.4.
Step 5.2.1. Let r be the <record, or fkeyed-record, designated by the fcurrent-
position% in fi. Let edd be the <evaluated-data-description+ in r.
Perform allocate(eddl to obtain a tgeneration9.g.
Step 5.2.2. Let abuf be an <allocated-buffer,: g. Attach abuf to the <file-opening,
in fi.
Step 5.2.3. Let v be the <basic-value-list+ in r. Perform set-storage(g,vl.
Step 5.2.4. Let pdd be a <data-description> simply containing <pointer> and no other
terminal subnodes. Let epsoet be an <evaluated-target* containing the
<generation+ contained in epso. Let agv be an *aggregate-value>
containing the <pointer-value,: g. Perform assign(epsoet,agv,pdd).
Step 5.2.5. If ers contains an <evaluated-keyto-options then perform
assign(ekoet,avk,dd).
Case 5.3. (Otherwise).
if ers contains an <evaluated-keyto-option> then perform
assign(ekoet,avk,ddl.
8.6.2 THE WRITE STATEMENT
Purpose: The <write-statement> causes a <records or <keyed-record+ to be transmitted from
a <generation+ or an <allocated-buffer+ to a <record-dataset,.
8.6.2.1 Execute-write-statement
<evaluated-write-statement,::= ffile-value, <evaluated-from-option>
[<evaluated-keyfrom-option,]
Operation: execute-write-statement(ws)
where ws is a <write-statement>.
Step 1. Let ews be an <evaluated-write-statement+ without subnodes.
Step 2. Perform steps 2.1 through 2.3 in any order.
Step 2.1. Let f be the immediate component of the <file-option> in ws. Perform
evaluate-file-option(f) to obtain a <file-value+,fv. Attach fv to ews.
step 2.2. Let fr be the <from-option> in ws. Perform evaluate-from-option to
obtain an *evaluated-from-option,refo and attach efo to ews.
Step 2.3. If ws contains a <keyfrom-option>,ko, then perform evaluate-keyfrom-
optionfkol to obtain an <evaluated-keyfrom-option>,ekfo and attach ekfo to
ews.
Chapter 8: Input/Output 235
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 254

Step 3. If the <file-information+,fi, designated by fv contains 4Ee_n, then go to Step
5.
step 4.
Step 4.1. Let efdl be an ievaluated-file-description-list+ containing <record>.
step 4.2. If fi does not contain <update> then attach <evaluated-file-descriptioti:
<output>; to efdl.
Step 4.3. Perform open(fv,efdlI to obtain sf. If sf is <fail, then perform raise-io-
condition(<undefinedfile-condition>,fv). If on normal return fi contains
<closed+ then perform raise-condition(<error-condition>) -
Step 5. fi must
Step 6. Perform
Step 7. Perform
8.6.2.2 Write
contain:
(record>:
<output>, or <update> and <direct>:
if and only if ews contains an <evaluated-keyfrom-option+, then <keved>.
writefewsl.
normal-sequence.
Operation: writetewsl
where ews is an *evaluated-write-statements.
Step 1. Let fi be the <file-information> designated by the tfile-value+,fv, in ews. fi
must contain <open,. If ews contains an <evaluated-keyfrom-option,: fcharacter-
string-value,,csv; then let k be a Frey,: csv. Otherwise let k be <absent+.
Let ds be the <record-dataset, designated by the tdataset-designator> in fi.
Step 2.
Case 2.1. fi does not contain an <allocated-buffer+ or does not contain <output>.
Go to Step 3.
Case 2.2. fi contains an fallocated-buffer+,abuf, and fi contains <output>.
Step 2.2.1. Let g be the *generations immediately contained in abuf. If abuf
contains a <key,, let kbu be this <key, and let csvb be the immediate
component of kbu; otherwise let kbu and csvb be <absent+. Perform
construct-record(g,kbu) to obtain kr.
Step 2.2.2. If fi contains <keyed> and if kbu is equal to any <key9 in the
fdataset,,ds, or if kbu is unacceptable to the implementation then
perform raise-io-condition(<key-condition>,fv,csvb).
Step 2.2.3. Perform insert-record(kr,fv) to obtain a tdesignator+.pos.
Step 2.2.4,. Replace the immediate component of the <current-position> in fi with
POS-
Step 3. If fi contains an *allocated-buffer,,abuf, containing a fgeneration+,g, then
perform free(g) and delete abuf from fi.
Step 4. Let g be the <generation+ in the <evaluated-from-option+ in ews. Perform
construct-recordcg ,k) to obtain kr.
Step 5. If fi contains <keyed> then if k is equal to any <key9 in ds or if k is
unacceptable to the implementation then perform raise-io-condition(<key-
condition>,fv,csvl.
step 6. Perform insert-record(kr,fv) to obtain a tdesignator>,pos.
Step 7. Replace the immediate component of the <current-position, in fi by pos.
236
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 255

8.6.3 THE LOCATE STATEMENT
Purpose: The <locate-statement> causes allocation of the specified based variable in an
<allocated-buffer+; it may also cause transmission of a based variable
previously allocated in an <allocated-buffer+.
8.6.3.1 Execute-locate-statement
<evaluated-locate-statement,::= <declaration-designator> ffile-value>
[fevaluated-pointer-set-option,3
[<evaluated-keyfrom-option91
Operation: execute-locate-statement(ls)
where 1s is a <locate-statement>.
Step 1. Let els be an <evaluated-locate-statement+ without subnodes.
Step 2. Perform steps 2.1 through 2.4 in any order.
Step 2.1.
Step 2.2.
Step 2.3.
Step 2.4.
Step 3. If
5.
Step 4.
Step 4.1.
Step 4.2.
Let f be the immediate component of the <file-option> in 1s. Perform
evaluate-file-option(f) to obtain a <file-value+,fv. Attach fv to els.
If 1s contains a <pointer-set-option>,pso, then perform evaluate-pointer-
set-option(pso) to obtain an <evaluated-pointer-set-option,repso and attach
epso to els.
If 1s contains a <keyfrom-option>,kfo, then perform evaluate-keyfrom-
optionfkfo) to obtain an <evaluated-keyfrom-option9,ekfo and attach ekfo to
els. If fi contains <keyed> then let chs be a copy of the immediate
component of ekfo; otherwise let chs be <absent>. If fi contains <keyed>
then let kk be <key+: chs.
Let cdp be a copy of the <declaration-designator> immediately contained in
1s;. Attach cdp to els.
the <file-information>,fi, designated by fv contains TV* then go to Step
Let efdl be an <evaluated-file-description-list+ containing <record> and
<output> I
Perform open(fv,efdl) to obtain sf. If sf is equal to <fail+, then perform
raise-io-condition(<undefinedfile-conditiorD,fv). If, on normal return, fi
contains <closed, then perform raise-condition(<error-condition>).
Step 5. fi must contain:
<record>;
<output> ;
if and only if els contains an <evaluated-keyfrom-option,, then <keyed>.
Step 6. If els does not contain an <evaluated-pointer-set-option+ then the <declaration-
designator> contained in els must designate a <declaration>,d, of the form
<based>:
<value-reference>:
<variable-reference>,v:
<declaration-designator>,p.
p must designate a <declaration> containing <pointer>. Perform evaluate-
variable-reference(v) to obtain g. Let epso be an fevaluated-pointer-set-
options: g; and attach epso to els.
step 7.
Step 7.1. If fi does not contain an <allocated-buffer+,abuf, then go to Step 8.
Chhpter 8: Input/Output 237
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 256

Step 7.2. Let g be the <generation, immediately contained in abuf. If abuf contains a
<key+, let k be this they, and let csvb be its immediate component;
otherwise let k and csvb be <absent+. Perform construct-record(g,k) to
obtain kr.
Step 7.3. If fi contains <keved> then if k is equal to any <key> in the frecord-
dataset,, designated by the tdataset-designator, in fir or if k is
unacceptable to the implementation, then perform raise-io-condition(<e
condition>,fv,csvb).
Step 7.4. Perform insert-record(kr,fvl to obtain pos.
Step 7.5. Replace the immediate component of the <current-position> in fi with pos.
Step 7.6. Perform free(g) and delete abuf from fi.
Step 8. Let dd be the <data-description> immediately contained in the <variable> of the
<declaration> designated by cdp. Perform evaluate-data-description-for-
allocation to obtain eda.
Step 9. Perform evaluate-size(edd) to obtain an <integer-valueF,int. If int is
unacceptable to the implementation then perform raise-io-condition(<record-
condition>,fv,chsl and optionally perform exit-from-io.
Step 10. Perform allocate(edd) to obtain g.
Step 11. Let desc be a <data-description> simply containing <pointer> without other
terminal subnodes. Let epsog be an <evaluated-target, containing the
<generation* in the <evaluated-pointer-set-option, in els. Let agv be an
taggregate-values containing <pointer-value% 9. Perform
assign(epsog,agv.desc).
Step 12. Let d be the <declaration> designated by the <declaration-designator> in els.
Step 12.1. If the <aggregate-type+ of g contains <structure-aggregate-type+ then
perform initialize-refer-options(g).
Step 12.2. Perform initialize-generation(g,d).
Step 13. Let abui be an *allocated-buffers: fgeneration+,g. If fi contains <keyed> then
attach kk to abuf. Attach abuf to the <file-opening> in fi.
Step 14. Perform normal-sequence.
8.6.4 THE REWRITE STATEMENT
Purpose: The <rewrite-statement> causes replacement of an existing *record+ or fkeyed-
record* in a <record-dataset,.
8.6.4.1 Execute-rewrite-statement
*evaluated-rewrite-statement+::= tfile-value, t[tkey,l *evaluated-from-option91
Operation: execute-rewrite-statementtrwsl
where rus is a <rewrite-statement>.
Step 1. Let erws be an <evaluated-rewrite-statement* without subnodes.
Step 2. Perform steps 2.1 through 2.3 in any order.
step 2.1. Let f be the immediate component of the <file-option> in rws. Perform
evaluate-file-option(f) to obtain a 4filevalue9,fv. Attach fv to erws.
step 2.2. If rws contains a <from-option>,fr, then perform evaluate-from-option(f
obtain an *evaluated-from-option+,efo and attach efo to erws.
238
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 257

Step 2.3.
Step 3. If
5.
Step 4.
Step 4.1.
Step 4.2.
If rus contains a <key-option>,ko, then perform evaluate-key-option to
obtain a fkeyS,k and attach k to erws.
the ffile-information+,fi, designated by fv contains tw then go to step
Let efdl be an <evaluated-file-description-list+ containing (record> and
<update>.
Perform open(fv,efdll to obtain sf. If sf is <w+ then perform raise-io-
condition(<undefinedfile-condition>,fv). If on normal return fi contains
<closed, then perform raise-condition(<error-condition>).
Step 5. fi must contain:
<record>;
<update>;
if erws does not contain a &ey+, then <sequential>:
if erws contains a *key+, then <keyed>.
If fi contains <direct> then erws must contain a *key>. If fi contains
<sequential> then erws may contain a they+ and an tevaluated-from-options.
Step 6. Perform rewrite (erws).
Step 7. Perform normal-sequence.
8.6.4.2 Rewrite
Operation: rewriteterws)
where erws is an <evaluated-rewrite-statement+.
Step 1. Let fi be the <file-information, designated by the *file-value,,fv in ems. fi
must contain e-9. If erws contains a flrey*, then let k be this Frey,;
otherwise let k be <absent+. If k is a they, then let csv be its immediate
component; otherwise let csv be <absent,.
step 2.
Case 2.1. k is fabsene.
fi must not contain a <delete-flaq, and the <current-position+ in fi must
not contain <undefined+. If the *current-position+ in fi designates a
<keyed-record%.kr then let k be a copy of the <key9 in kr and let csv be the
immediate component of k.
Case 2.2,. k is a <key,.
Perform position-fileterwsl.
Step 3.
Case 3.1. erws contains an <evaluated-from-option,refo.
Let g be the *generation, in efo.
Case 3.2. (Otherwise).
fi must contain an <allocated-buffer,,abuf. Let g be the &generation> in
abuf.
step 4. Perform construct-record(g,k) to obtain r.
Step 5. Let eddl be the <evaluated-data-description+ in g and edd2 the tevaluated-data-
description9 in the erecord, designated by the <current-position+ in fi.
Perform evaluate-size (eddl) to obtain an iinteger-value9,intl and evaluate-
size(edd2) to obtain an <integer-value+,int2.
Chapter 8: Input/Output 239
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 258

step 6. Let rd be the +record> or <keyed-record, designated by the *current-position> in
fi.
Case 6.1. intl and int2 are equal.
Replace rd by r.
Case 6.2. intl and int2 are not equal.
If rd is a *record+, replace rd by an implementation-defined <record>:
otherwise replace rd by an implementation-defined *keyed-record, with an
equal <keys. Perform raise-io-condition(<record-condition>,fv,csvl, and
optionally perform exit-from-io.
step 7. If fi contains an *allocated-buffer,,abuf, then let g be the <generation+ in
ahuf, perform free(g), and delete abuf from fi.
8.6.5 THE DELETE STATEMENT
Purpose: The <delete-statement> deletes a <record> or *eyed-record, from a trecord-
dataset,.
8.6.5.1 Execute-delete-statement
<evaluated-delete-statement,::= *file-value+ [<key91
operation: execute-delete-statement(d1.s)
where dls is a <delete-statement>.
step 1. Let eds be an *evaluated-delete-statement, without subnodes.
step 2. Perform Steps 2.1 and 2.2 in either order.
Step 2.1.
Step 2.2.
step 3. If
5.
step 4.
Step 4.1.
Step 4.2.
step 5.
step 6.
step 7.
Let f be the immediate component of the <file-option> in dls. Perform
evaluate-file-option(f) to obtain a <file-value9,fv. Attach fv to eds.
If dls contains a <key-option>,ko, then perform evaluate-key-option to
obtain a tkey>,k and attach k to eds.
the ffile-information+,fi, designated by fv contains te then go to Step
Let efdl he an <evaluated-file-description-list, containing <update> and
<record>.
Perform open(fv,efdl) to obtain sf. If sf is tm> then perform raise-io-
condition(<undefinedfile-condition>,fvJ. If on normal return fi contains
<closed, then perform raise-condition(<error-condition>).
fi must contain:
<record>;
<update>;
if eds contains a *key>, then <keyed>;
if eds does not contain a <key+, then <seWX?.ntial>.
If fi contains <direct> then eds must contain a *key+. If fi contains
<sequential> then eds may contain a ikey,.
Perform deleteteds).
Perform normal-sequence.
240
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 259

8.6.5.2 Delete
Operation: deletecedsl
where eds is an <evaluated-delete-statement>.
Step 1. Let fi be the <file-information9 designated by the <file-value+,fv, in eds. fi
must contain 4open+. If eds contains a <key+, then let k be this <key+ and let
csv be the immediate component of k; otherwise let k and csv be Cabsent,.
Step 2.
Case 2.1. k is <absent+.
fi must not contain a <delete-flaq9. The <current-position, in fi must not
contain <undefined,.
Case 2.2. k is a <key+-
Perform position-file(eds1.
Step 3. Let ds be the immediate component of the &record-dataset, designated by the
fdataset-designator> in fi. Let kr be the node designated by the fcurrent-
position+ in fi (kr is a <record+ or a <keyed-record+). Delete kr from ds.
step 4.
Case 4.1. fi contains <direct>.
Replace the immediate subnode of the icurrent-position* in fi by
<undefined,.
Case 4.2. fi contains <sequential>.
Replace the immediate subnode of the <current-position, in fi by a
*designator, designating the predecessor of kr in ds (this may be <alpha+, a
<record, or a <keyed-record,).
Step 5. Attach a <delete-flaq> to the <file-opening> in fi.
Step 6. If fi contains an <allocated-buffer,,abuf, then let g be the ggeneration? in
abuf. perform free(g), and delete abuf from fi.
8.6.6 OPERATIONS APPLICABLE TO RECORD I/O
8.6.6.1 Evaluate-from-option
<evaluated-from-option>::= fgeneration,
Operation: evaluate-from-option(fr1
where fr is a <from-option>.
result: an <evaluated-from-option,.
step 1. Let v be the <variable-reference> immediately contained in fr. Perform
evaluate-variable-reference(v) to obtain a fgenerati0np.g. which must be
connected.
step 2. Return an <evaluated-from-option>: g.
Chapter 8: Input/Output 241
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 260

8.6.6.2 Evaluate-into-option
<evaluated-into-option,::= <generation,
Operation: evaluate-into-option(it0)
where ito is an <into-option>.
result: an <evaluated-into-option+.
Step 1. Let v be the <variable-reference> immediately contained in ito.. Perform
evaluate-variable-reference(v) to obtain a fgeneration+,g, which must be
connected.
step 2. Return an <evaluated-into-option+: g.
8.6.6.3 Evaluate-pointer-set-option
<evaluated-pointer-set-option,::= fgeneration,
Operation: evaluate-pointer-set-option(ps0)
where pso is a <pointer-set-option>.
result: an <evaluated-pointer-set-option+.
step 1. Let v be the <variable-reference> immediately contained in pso. Perform
evaluate-variable-reference(v) to obtain a fgeneration9.g.
Step 2. Return an <evaluated-pointer-set-option,: g.
8.6.6.4 Evaluate-key-option
Operation: evaluate-key-option
where ko is a <key-option>.
result: a <key>.
Step 1. Let e be the <expression> immediately contained in ko. Perform evaluate-
expression(e) to obtain res, containing a <basic-value>,bv.
step 2. Let tt be a <data-type> containing <character>, <nonvaryinq>, and Gnaximum-
length>: <asterisk>. Let rt be the <data-type> of e. Perform convert(tt,rt,bv)
to obtain a fcharacter-string-value,,chs.
Step 3. Return a <keys: chs.
8.6.6.5 Evaluate-keyfrom-option
<evaluated-keyfrom-option,::= *character-string-value>
Operation: evaluate-keyfrom-option0
where kfo is a <keyfrom-option>.
result: an <evaluated-keyfrom-option>.
Step 1. Let e be the <expression> immediately contained in kfo. Perform evaluate-
expression(e) to obtain res, containing a <basic-value>,bv.
Step 2. Let tt be a <data-type> containing <character>, <nonvaryinq>, and Gnaximum-
length>: <asterisk>. Let rt be the <data-type> of e. Perform convert(tt,rt,bv)
to obtain a <character-string-value,,chs.
242
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 261

step 3. Return an <evaluated-keyfrom-option,: chs.
8.6.6.6 Evaluate-iqnore-option
<evaluated-ignore-option,::= <integer-value,
Operation: evaluate-iqnore-option(ig0)
where igo is an <ignore-option>.
result: an *evaluated-ignore-option,.
Step 1. Let e be the <expression> immediately contained in igo. Perform evaluate-
expression-to-integer(e) to obtain an <integer-value+,int, which must not be
negative.
Step 2. Return an <evaluated-ignore-option>: int.
8.6.6.7 Evaluate-kevto-option
<evaluated-keyto-option,::= <evaluated-target+
Operation: evaluate-kevto-ootion(kto)
where kto is a <keyto-option>.
result: an <evaluated-keyto-option,.
Step 1. Let ktotr be the <target-reference> in kto. Perform evaluate-target-
reference(ktotr) to obtain an <evaluated-target+,et.
Step 2. Return an <evaluated-keyto-options: et.
8.6.6.8 Construct-record
Operation: construct-record(g,k)
where g is a connected *generation+,
k is a ttkeysl.
result: a *record, or a <keyed-records.
step 1. Let agv be an <aggregate-value+ which is the value of g. (See Section 7.1.3.7
Let edd be the <evaluated-data-description+ in g. Let bvl be the fbasic-value-
list* in agv. Let r be a <record+: edd bvl.
Step 2.
Case 2.1. k is *absent+.
Return r.
Case 2.2. k is a *key+.
Return *keyed-record*: r k.
Chapter 8: Input/Output 243
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 262

8.6.6.9 Insert-record
Operation: insert-record(kr,fv)
where kr is a <record+ or a <keyed-records,
fv is a <file-value,.
result: a *designators.
Step 1. If kr is a <keyed-records then let k be the <key+ in kr and let csv be the
immediate component of k; otherwise let k and csv be <absent>. Let fi be the
<file-information+ designated by fv. Let rl be the <record-list> or fkeyed-
record-list, in the <dataset+ designated by the fdataset-designator+ in fi. Let
edd be the <evaluated-data-description+ in kr. Perform evaluate-sizefedd) to
obtain an <integer-value+,int. If int is not acceptable to the implementation
then perform Step 1.1.
Step 1.1. If kr is a <record+, let kr be a new implementation-defined <records;
otherwise let kr be an implementation-defined <keyed-record, with an equal
<key+. Optionally perform Step 2. If this operation is preceded in its
immediately containing <operation-list+ by an *operation> for program-
epilogue then perform some implementation-defined action; otherwise perform
raise-io-condition(<record-condition>,fv,csv)- Optionally perform exit-
from-io.
Step 2.
Case 2.1. fi contains <direct>.
Attach kr to rl in a position chosen in an implementation-defined way.
Case 2.2. fi contains <keyed> and <sequential>.
Attach kr to rl in an implementation-defined position which may depend on
the ikey, in kr, on the <record+ in kr and the *current-position, in fi.
Case 2.3. fi contains <sequential> and not <keved>.
Append kr to rl in the position immediately following the immediate
component of rl designated by the <current-position%= in fi.
Step 3. Return a *designator> designating kr in rl.
8.6.6.10 Position-file
Operation: position-file(evst)
where evst is an <evaluated-read-statement+, an +evaluated-rewrite-
statement+ or an <evaluated-delete-statement+.
Step 1. If evst contains a <key> then let k be this <key, and let csv be its immediate
component; otherwise let k and csv both be <absent*. Let fv be the ffile-value,
in evst, let fi be the <file-information+ designated by fv, let ds be the
*record-dataset, in the fdataset, designated by the fdataset-designator+ in fi,
and let pos be the <current-position, in fi.
step 2.
Step 2.1. If k is a <key+ and k is unacceptable to the implementation then perform
raise-io-condition(<key-condition>,fv,csvl.
Step 2.2.
Case 2.2.1. k is <absent>.
Go to Step 3.
244
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 263

Case 2.2.2. k is a <key, and k is not equal to any <key+ in ds.
Step 2.2.2.1. If fi does not contain a <delete-flaq9, then attach a *delete-
flas, to the <file-opening+ in fi.
Step 2.2.2.2. Replace the immediate component of pos by *undefined+.
Step 2.2.2.3. Perform raise-io-condition(<key-condition>,fv,csvl.
Case 2.2.3. k is equal to a <key+ in a <keyed-records,kr, in ds.
Step 2.2.3.1. If fi contains a <delete-flaq>,dfl, then delete dfl from fi.
Step 2.2.3.2. Replace the immediate component of pas by a tdesignator,
designating kr.
Step 3.
Case 3.1. evst is an <evaluated-read-statement+ containing an fevaluated-ignore-
option9,eigo.
Let int be the ginteger-value, in eigo.
Case 3.2. evst is an <evaluated-read-statement> not containing a *keys or an
<evaluated-ignore-option>.
Let int be an *integer-value, with value 1.
Case 3.3. (Otherwise).
Terminate this operation.
Step 4. pos must contain a <designator+,rdes.
Step 4.1. If int is 0, then terminate this operation.
Step 4.2.
Case 4.2.1. rdes designates fomesa3.
Perform raise-io-condition(<endfile-condition>,fv)
Case 4.2.2. rdes designates the last element of the <keyed-record-list, (or trecord-
list+) in ds.
Replace rdes by a <designator+ designating the fomeqa, in ds.
Case 4.2.3. (Otherwise).
Replace rdes by a <designator> designating the next <keyed-record+ (or
<record91 in ds.
Step 4.3. If rdes designates fomeqa, then perform raise-io-condition(<endfile-
condition>,fv).
Step 4.4. If fi contains a <delete-flag+ then delete it from fi.
Step 4.5. Decrement int by 1. Co to Step 4.
8.6.6.11 Evaluate-size
Operation: evaluate-sixe(edd1
where edd is an *evaluated-data-description+.
result: an <integer-value+.
Step 1. Return an implementation-defined <integer-value+, depending on edd.
Chapter 8: Input/Output 245
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 264

8.6.6.12 Exit-from-io
Operation: exit-from-io(fv1
step 1.
Step 2.
Step 3.
Step 4.
where fv is a <file-value%.
Let fi be the ffile-information+ designated by fv. If fi contains an
<allocated-bufferS,abuf, containing a fgenerationS,g. then perform free(g) and
delete abuf from fi.
Perform trim-io-control.
Let eud be the current <executable-unit-designator>. Perform trim-group-
control(eud1.
Replace the immediate component of the current <statement-control> by
<operation-list+:
toperations for advance-execution
<operation, for normal-sequence.
8.6.6.13 Trim-io-control
Operation: trim-io-control
Step 1.
Step 2.
Step 3.
step 4.
Step 5.
step 6.
Step 7.
Let bc be the current <block-control,.
If bc contains a <data-item-control-list,rdiclr then delete dicl from bc.
If bc contains a <current-scalar-item-list+,csil, then delete csil from bc.
If bc contains a fstring-io-control+,sioc, then delete sioc from bc.
If bc contains a <format-control-list,rfcl, then delete fcl from bc.
If bc contains a <remote-block-stateP,rbs, then delete rbs from bc.
If bc contains a <current-file-value,rcfv, then delete cfv from bc.
246
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 265

8.7 The Stream I/O Statements
8.7.1 THE GET STATEMENT
8.7.1.1 Execute-qet-statement
Operation: execute-qet-statementtgs)
where gs is a <get-statement>.
Step 1.
Case 1.1. gs has a <get-file>,gf.
Perform execute-get-file(gf1.
Case 1.2. gs has a <get-string>,gstr.
Perform execute-get-string(gstr).
Step 2. Perform trim-io-control.
Step 3. Perform normal-sequence.
8.7.1.2 Execute-qet-file
Operation: execute-qet-file(gf)
where gf is a <get-file>.
Step 1. Perform steps 1.1, 1.2, and 1.3 in any order.
Step 1.1. Let fo be the <value-reference> in the <file-option> in gf. Perform
evaluate-file-option to obtain a <file-value,,fv.
Step 1.2. If gf contains a <skip-option>, sko, then let e be the <expression> in sko,
and perform evaluate-expression-to-integer(e) to obtain an tinteger-
value+,sk.
Step 1.3. If qf contains a <copy-optiorD,co, then let cv be the <value-reference> in
Step 2.
co* and perform evaluate-file-option to obtain a <file-value+,cf.
Let fi be the <file-information+ designated by fv, and, if a <copy-option> is
present in gf, let cfi be the *file-information+ designated by cf. If fi
contains q-3 then go to Step 4.
step 3.
step 4.
step 5.
step 6.
step 7.
step 8.
Step 9.
Let efdl be an <evaluated-file-description-list+ containing <stream> and
<input>. Perform open(fv,efdl) to obtain rf. If rf is <fail+ then perform
raise-io-condition(<undefinedfile-condition>,fv). If on normal return fi
contains fclosed, then perform raise-condition(<error-condition>).
fi must contain <stream> and <input>.
If gf does not have a <copy-option> then go to step 9. If cfi contains <e>
then go to Step 7.
Let efdll be a <evaluated-file-description-lists containing <stream> and
<output>. Perform open(cf,efdll) to obtain ref. If rcf is <fail, then perform
raise-io-condition(<undefinedfile-condition>,cf). If on normal return fi
contains <closed+ then perform raise-condition(<error-condition>).
cfi must contain <stream> and <output>.
Attach a <copy-file+: cf; to the current <block-state*.
If gf has a <skip-option> then perform skip(sk,fv) -
Chapter 8: Input/Output 247
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 266

Step 10. If gf has an <input-specification> then:
Case 10.1. gf has a <list-directed-input>,ldi.
Perform get-list(ldi,fv).
Case 10.2. gf has a <data-directed-input>,ddi.
Perform get-data(ddi,fv).
Case 10.3. gf has an <edit-directed-input>,edi.
Perform get-edit(edi,fv).
Step 11. If gf has a <copy-option> then delete the current fcopy-file+.
Step 12. If gf has a <list-directed-input> then terminate this operation. Otherwise set
the <first-comma+ in fi to contain <on+. -
8.7.1.3 Execute-qet-strinq
Operation: execute-qet-strinqcgstrl
where gstr is a <get-string>.
Step 1. Perform Steps 1.1 and 1.2 in either order.
Step 1.1.
Step 1.2.
Let e be the <expression> immediately contained in gstr. Perform evaluate-
expression(e), to obtain an <aggregate-value>,av. Let sdt be the <data-
type> in the <data-description> immediate component of e. Let sv be the
<basic-value, in av. Let tdt be a <data-type> simply containing
<character>, <nonvaryinq>, and <maximum-length>: <asterisk>. Perform
convert(tdt.sdt,sv) to obtain a <character-strinq-value+,csv. Attach to the
current fblbck-control, a fstring-io-control%: csv <first-comma+: TV+.
If gstr has a <copy-option>,co, then perform Steps 1.2.1 through 1.2.4.
Step 1.2.1.
Step 1.2.2.
Step 1.2.3.
Step 1.2.4.
Step 2.
Let cv be the <value-reference> in co. Perform evaluate-file-option
to obtain a <file-value+,cf. Let cfi be the <file-information+
designated by cf. If cfi contains t-3 then go to Step 1.2.3.
Let efdl be an <evaluated-file-description-list+ containing <stream> and
<output>. Perform open(cf,efdll to obtain a result ref. If rcf is
&ail+ then perform raise-io-condition(<undefinedfile-condition>,cf).
If on normal return fi contains <closed+ then perform raise-
condition(<error-condition>).
cfi must contain <stream> and <output>.
Attach a <copy-file*: cf; to the current <block-state,.
Case 2.1. gstr has a <list-directed-input>,ldi.
Perform get-list(ldi1.
Case 2.2. gstr has a <data-directed-input>,ddi.
Perform get-datacddi).
Case 2.3. gstr has an <edit-directed-input>,edi.
Perform get-editfedil.
step 3. If gstr has a <copy-option> then delete the current <copy-file+.
248
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 267

8.7.1.U Get-list
Operation: get-list(ldi,fv)
Step 1.
Step 2.
Step 3.
Step 4.
where ldi is a <list-directed-input>,
fv is a [*file-value+l.
Attach to the current *block-control+ a
.?data-item-control-list+:
gdata-item-control+:
<data-list-indicator+ designating the <input-target-list> of ldi
<data-item-indicator,:
4undefined3.
Perform establish-next-data-item to obtain a l current-scalar-item.,ndi or
fnone+,ndi. If ndi is <none, then terminate this operation.
Perform parse-list-input to obtain a <character-string-value>,csv.
If fv is a <file-value+ then attach to the current <block-control+ a fcurrent-
file-value+: fv.
Case 4.1.
Case 4.2.
Case 4.3.
Case 4.4.
Case 4.5.
csv contains just one terminal which is a C,) or a fnull-character-string+.
Go to Step 6.
The terminal nodes of csv can be parsed as 'Cnon-blank-comma-quote) [{non-
blank-comma-list)]".
Let v be csv. Let st be <character>.
The terminal nodes of csv can be parsed as "{simple-character-string-
constant)'.
Perform basic-character-value(csv) to obtain a <character-string-value3,v.
Let st be <character>.
The terminal nodes of csv can be parsed as "$simple-bit-string-constantg".
Perform basic-bit-value(csv) to obtain a <bit-string-value+.v. Let st be
<bit>.
(Otherwise).
Let intg be the smallest integer such that the <character-string-value> of
length intg containing the first intg <character-value% of csv does not
have a continuation conforming, and does not itself conform, to either ot
the syntaxes: {simple-character-string-constant* or $simple-bit-string-
constant).
Perform raise-io-condition(<conversion-condition>,fv,csv,intg). On normal
return let csv be the immediate component of the current freturned-onsource-
value,; csv must not contain only blanks; go to step 4.
Step 5. Let et be the <evaluated-target, in ndi. Let agv be
Chhpter 8: Input/Output 249
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 268

*aggregate-value+:
*aggregate-type,:
*scalar>;
*basic-value-list+:
<basic-value+:
V.
Let dd be
<data-description>:
<item-data-description>:
<data-type>:
<non-computational-type>:
<string>:
<string-type>: st;
Unaximum-length>:
<asterisk>;
<nonvaryinq>.
Perform assign(et,agv,ddl.
Step 6. If fv is a <file-value, then delete the current <current-file-values. GO to
Step 2.
8.7.1.4..1 Parse-list-input
Operation: parse-list-innut
where fv is a defile-value,].
result: a <character-string-value+.
Step 1.
Case 1.1. fv is a <file-value,.
Let fi be the <file-information+ designated by fv, and let ds be the
fdataset3 designated by the *dataset-designator3 in fi. Let cp be the
*current-position3 in fi.
Case 1.1.1. There is no <stream-item-list+ in ds or cp designates the last fstream-
item3 in the <stream-item-list+ in ds.
Perform raise-io-condition(<endfile-condition>,fv).
Case l-1.2. (Otherwise).
Let sl be a *stream-item-list, containing the f&ream-item% in the
<stream-item-list3 in ds, following the <stream-item+ designated by cp.
Case 1.2. fv is *absent%.
Let sl be the <character-string-value+ in the current <string-io-control>.
If sl contains no $symbol)s then perform raise-condition(<error-condition>).
step 2.
Case 2.1. sl can be parsed as "[{leading-delimiter-list*1 , [<stream-item-list31".
Let Id be that sequence in sl which satisfies "[(leading-delimiter-list)3 ,I
in this parse, and let its number of terminal nodes be In. Perform input-
stream-item In times. If the current <first-comma+ exists and contains
<on> or the *first-comma3 in the <file-information3,fi exists and contains
ton, then return <character-string-values: <character-value-list3:
+character-value+: Csymbol): C.). Otherwise set the current *first-comma3
(respectively, the <first-comma* in fi) to contain tz+, perform parse-list-
input to obtain csv, and return CSV.
250
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 269

Case 2.2. sl can be parsed as "*leading-delimiter-list)'.
Let In be the number of terminal nodes in sl. Perform input-stream-item
In times. If the current <first-comma, exists and contains <on+, or the
*first-comma, in fi exists and contains <on+ then return <character-string-
value,: <null-character-strinqs. Otherwisego to Step 1.
Case 2.3. sl can be parsed as "[{leading-delimiter-list)] * tbstring-symbol-or-
linemark-list,l".
Let In be the number of terminal nodes in sl. Perform input-stream-item
In times. Perform raise-condition(<error-condition>).
Case 2.4. sl can be parsed as "[Cleading-delimiter-list21 *putative-list-constant?
Cfbl.1 [<stream-item-list+ll".
Let In be the number of terminal nodes in sl preceding that part which
satisfies "[<stream-item-list,]' in this parse. Perform input-stream-
item In times. Let csv be a <character-string-value+ whose terminal
nodes are those of the part of sl which satisfies '$putative-list-constant)
Cbl,l' except those terminals that are <linemark%. Let fc be the current
<first-comma,, if that exists; otherwise let fc be the *first-comma, in fi.
If the last {symbol+ of csv is a $,p then set fc to contain ton+; otherwise
set fc to contain <off+. Delete from csv the last {symbol), if that
+symbol) is a 13 or a C,). Return csv.
8.7.1.4.2 Parsinq Cateqories for List Directed Input
Some of the categories used in parsing input streams for list directed input are
categories of the Concrete Syntax or the Machine-state Syntax. Others are defined as
follows:
{leading-delimiter)::= LI 1 <linemark>
*putative-list-constant*::= WCtstring-symbol-or-linemark-list,l'
C+non-blank-comma-quote*
CCnon-blank-comma-list$+ll 1
{{data-symbol? 1 {;p 1 = 1
CCnon-blank-comma-list)3
{data-symbol)::= {letter) I {digit) I I + I -l(l~l-I:l~141~1~1 > J < I / 1 * I $ I X-1 Cextralingual-character)
Note: E; $s;bnodes of {data-symbol) are those of +Isymbol) except for b, C,). f;?. {'p
= .
*non-blank-comma-quote)::= {data-symbol) I ; I = I tlinemark,
<non-blank-comma)::= (non-blank-comma-quote? I '
&string-symbol-or-linemark+::= {string-or-picture-symbol+ I <linemark+
Chapter 8: Input/Output 251
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 270

8.7.1.5 Get-data
$data-basic-reference)::= Cfdata-structure-reference)]
[*lb-list)) {identifier) [$&list)1 [<data-subscripts)1
{data-structure-reference)::= {data-basic-reference).
*data-subscripts)::= (<data-subscript-commalist)) C(ld-list)1
Cdata-subscript)::= [$&list)1 [+I-1 +integer) t$k+list)l
<scalar-facts,::= <identifier-list> <declaration-designator>
[<data-description>1 tinteger-value>
Operation: qet-data(ddi,fv)
where ddi is a <data-directed-input>,
fv is a [<file-value93.
step 1. Perform parse-data-input-name(fv) to obtain a <character-string-value+,nf.
step 2.
Case 2.1.
Case 2.2
Case 2.3.
Case 2.4.
nf contains a fnull-character-strinq9 or has just one +%.ymbol) which
contains a C,).
Go to Step 1.
nf has just one $symbol+ which contains a C;).
Terminate this operation.
The last Csymbol) of nf contains an $=).
Let nnf be a {symbol-list) containing, in order, all the $symbol)s of nf
except its terminal $symbol): $=o. If nnf is an empty list then let nnf be
a fnull-character-strinqs.
(Otherwise).
Let vf be <character-string-value+: <null-character-strinqs. If the last
{symbol) of nf contains a C;) then let li=l; otherwise let li=O. Go to Step
9.
Step 3. Perform parse-data-input-value(fv) to obtain a <character-string-value,,vf. If
vf contains a {symbol) and its last $symbol) contains a C;) then let li=l;
otherwise let li=O.
step 4..
Step 4.1.
Step 4.2,.
If nnf conforms to the syntax for {data-basic-reference) then let dbr be a
{data-basic-reference* whose terminal nodes are pairwise equal to the
terminal nodes of nnf, both sets being taken in order. Otherwise go to Step
9.
Let vr be a <variable-reference> without subnodes.
Step 4.2.1. Let id11 be an {identifier-list) containing, in order, the $identifier)s
in dbr. Let id12 be an <identifier-list> without subnodes. For each
$identifiera,idc, in id11 let ida be the corresponding <identifier> and
append ida to id12. If id12 contains more than one element, then let
id13 be a copy of id12, delete the first element of idl3, and attach
id13 to vr.
252
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 271

Step 4.2.2. If dbr does not contain any (data-subscripts)s then let ssl be tabsen-
and go to Step 4.2.3.
Let dsl be a $data-subscript-list) containing all the Cdata-subscript)s
in all the Cdata-subscripts) in dbr, in order. Let ssl be a <subscript-
list> without subnodes.
For each {data-subscript+.dss, in dsl, in order, perform Step 4.2.2.1.
Attach ssl to vr.
Step 4.2.2.1. Let csdss be the <character-string-value, corresponding to dss.
Perform basic-numeric-value(csdss) to obtain a <value-and-type+:
<real-value*,rv <data-type>,,dt. Let ss be
<subscript>:
<expression>:
<constant>:
<basic-value,:
rv;
dt;
<data-description>:
<item-data-description>:
dt.
Step 4.2.3. Let blo be the <begin-block> or <procedure> that simply contains the
<executable-unit> designated by the current cexecutable-unit-
designator>.
Step 4.2.3.1. If blo contains a <declaration-list>,dl, then go to Step 4.2.3.3.
Step 4.2.3.2. If blo is not contained in a <procedure> then go to Step 9. Let
blo be the <begin-block> or <procedure> containing blo which
contains no other <begin-block> or <procedure> containing blo.
Go to Step 4.2.3.1.
Step 4.2.3.3. Let sfl be a <scalar-facts-list, containing all of the distinct
<scalar-facts% which can be obtained by performing Steps
4.2.3.3.1 and 4.2.3.3.2.
Step 4.2.3.3.1. Let d be a <declaration> in dl. Let sfx be a <scalar-facts>
containing a <declaration-designator> designating d, an
<identifier-list>,idl: id; where id is the <identifier>
immediately contained in d, and an <integer-value+: 0. If d
contains <variable> which immediately contains a <data-
description>,dd, then perform Steps 4.2.3.3.1.1 and
4.2.3.3.1.2.
step 4.2.3.3.1.1. If dd immediately contains a cdimensioned-data-
description>,ddd. then let nbp be the number of
<bound-pair>s in the <bound-pair-list> of ddd. add
nbp to the value of the <integer-value, in sfx,
replace that <integer-value* with the sum so
obtained, and let dd be the <element-data-
description> in ddd.
step 4.2.3.3.1.2. If dd simply contains a <structure-data-
description>,sdd, which immediately contains an
<identifier-list>,idlx, then choose an
<identifier>,,idx., from idlx, append idx to idl,
let dd be the <data-description> in the <member-
description> corresponding to idx in the <member-
description-list> of sdd, and go to Step
4.2.3.3.1.1.
Step 4.2.3.3.2. If d contains a <variable> then let idd be the <item-data-
description> simply contained in dd and attach a <data-
description>: idd; to sfx.
Chapter 8: Input/Output 253
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 272

Step 4.2.3.4. In this step a list will be said to be an ordered sublist ot
another list if the two lists are equal or if a list equal to the
first list can be obtained by deleting one or more elements from
the second.
If id12 is not an ordered sublist of the <identifier-list> of any
fscalar-facts, in sfl then go to step 4.2.3.2. If id12 is an
ordered sublist of the <identifier-list% of more than one
<scalar-facts+ in sfl but is not equal to one of them, then go to
Step 9. If id12 is equal to the <identifier-list> of a Gcalar-
factss,sfx, then let id14 be that <identifier-list>. If id12 is
an ordered sublist of the <identifier-list>,idl4, of exactly one
tscalar-facts*,sfx in sfl. then id12 must equal id14.
If ssl is a <subscript-list> then let nssl be the number ot
<subscript% in ssl. Otherwise let nssl be 0. It nssl is not
equal to the value of the tinteger-value, in sfx then go to Step
9.
Let d be the <declaration> designated by the <declaration-
designator> in sfx. d must contain <declaration-type>:
<variable>; and must not contain <based> without a subnode. lhe
<data-description>,dd, in sfx must contain an <item-data-
description> containing a <data-type>: <computational-type>.
Attach the <declaration-designator> in sfx to vr. Attach the
<data-description> in sfx to vr.
Step 4.2.3.5. In this step a list is said to be an initial sublist ot another
list if the two lists are equal or if a list equal to the first
can be obtained by deleting the last element of the second list
one or more times.
If ddi does not contain a <data-target-list> then go to Step 4.3.
Let vrl be a <variable-reference-list> consisting of each
<variable-reference> in a <data-target> in the <data-target-list>
of ddi whose <declaration-designator> equals that of vr and whose
<identifier-list>, if present, has fewer elements than id14. If
vrl is empty then go to step 9. If any <variable-reference> in
vrl is without an <identifier-list> then go to step 4.3.
Otherwise, go to step 9 unless some <variable-reference> in vrl
has an <identifier-list> which is an initial sublist of the
<identifier-list> obtained by deleting the first <identitier>
from ia14.
Step 4.3. Perform evaluate-variable-reference0 to obtain a tgeneration+,g.
step 5. If fv is a <file-value+ then attach <current-file-value+: fv; to the current
<block-control+. If vf contains a fnull-character-strinq3 then go to Step 8.
If vf contains the single $symbol): C.); then go to step 8. If vf contains the
single $symbol): $;); then go to Step 8. If vf terminates in the {symbol): C;);
then remove this {symbols.
Step 6.
Case 6.1. The terminal nodes of vf can be parsed as "{non-blank-comma-quote) [<non-
blank-comma-1ist)l..
Let v be vf. Let st be <character>.
case 6.2. The terminal nodes of vf can be parsed as *<simple-character-string-
constant)".
Perform basic-character-value(vf1 to obtain a *character-string-value+,v.
Let st be <character>.
Case 6.3. The terminal nodes of vf can be parsed as '<simple-bit-string-constant)'.
Perform basic-bit-valuetvf) to obtain a fbit-string-valueS,v. Let st be
<bit>.
254
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 273

Case 6.4. (Otherwise).
Let intg be the smallest integer such that the <character-string-value> of
length intg containing the first intg <character-value% of csv does not
have a continuation conforming, and does not itself conform, to either of
the syntaxes: $simple-character-string-constant) or Csimple-bit-string-
constant).
Perform raise-io-condition(<conversion-condition>,fv,vf,intg). On normal
return let vf be the immediate component of the current freturned-onsource-
values; vf must not contain only blanks; go to Step 6.
Step 7. Let etg be <evaluated-target,: g. Let agv be
<aggregate-value*:
faggregate-type,:
*scalar+;
<basic-value-list+:
<basic-value+:
V.
Let dd be
<data-description>:
<item-data-description>:
<data-type>:
<non-computational-type>:
<string>:
<string-type>: st;
<maximum-length>:
<asterisk>;
<nonvarvinq>.
Perform assign(etg,agv,dd).
Step 8. If fv is a <file-value+ then remove the current <current-file-value,. If li=l
then terminate this operation; otherwise go to Step 1.
Step 9. Perform raise-io-condition(<name-condition>,fv,str), where str is a l character-
string-value+ containing, in order, the {symbolps of nf and the $symbol)s of vf
(excepting a final $symbol): Q;+;, if any); On-normal return if li=O then go to
Step 1; otherwise terminate this operation.
8.7.1.5.1 Parse-data-input-name
Operation: parse-data-input-name(fv)
where fv is a [*file-value,].
Step 1.
result: a <character-string-value,.
Case 1.1. fv is a <file-value,.
Let fi be the <file-information> designated by fv, and let ds be the
<dataset+ designated by the fdataset-designator3 in fi. Let cp be the
<current-position3 in fi.
Case 1.1.1. There is no <stream-item-list* in ds, or cp designates the last fstream-
item3 in the <stream-item-list+ in ds.
Perform raise-io-condition(<endfile-condition>,fv).
Case 1.1.2. (Otherwise).
Let sl be a fstream-item-lists containing, in the same order, fstream-
item% equal to the <stream-item% in the <stream-item-list3 in ds which
follow the <stream-item3 designated by cp.
Chapter 8: Input/Output 255
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 274

Case 1.2. fv is <absent,.
Let sl be the <character-string-value, in the current *string-io-control>.
If sl contains no $symbol)s then perform raise-condition(<error-condition>).
Step 2.
Case 2.1. sl can be parsed as "CCleading-delimiter-list+] f, 1 ; 1 =3 [fstream-item-
list%l".
Let Id be that sequence in sl which satisties "C$leading-delimiter-list)3 1,
I ; 1 =3" in this parse, and let the number of its terminal nodes be In.
Perform input-stream-item(fv3 In times. Let endld be the last terminal node
in Id. Return <character-string-value+: <character-value-list+: tcharacter-
value>: {symbol): endld.
Case 2.2. sl can be parsed as "{leading-delimiter-list)'.
Let In be the number of terminal symbols in sl. Perform input-stream-
item In times. Return *character-string-value>: +null-character-
strinqs.
Case 2.3. sl can be parsed as "[{leading-delimiter-list)3 Cputative-name-field+ C (; 1
=3 [*stream-item-list911".
Let In be the number of terminal nodes in sl before that part which
satisfies "[<stream-item-list91" in this parse. Perform input-stream-
item In times. Let csv be a <character-string-value+ whose terminal
nodes are those preceding the part of sl which satisfies "Cfstream-item-
list+]" in this parse, excluding the part of sl satisfying "ITleading-
delimiter-1ist)l" and those terminals that are flinemark9s. Return csv.
8.7.1.5.2 Parse-data-input-value
Operation: parse-data-input-value(fv3
where fv is a Ctfile-value>].
Step 1.
result: a <character-string-value,.
Case 1.1. fv is a *file-value+.
Let fi be the tfile-information+ designated by fv, and let ds be the
*dataset+ designated by the fdataset-designator9 in fi. Let cp be the
<current-position+ in fi.
Case 1.1.1. There is no <stream-item-list9 in ds, or cp designates the last +stream-
item9 in the <stream-item-list9 in ds.
Perform raise-io-condition(<endfile-condition>,fv).
Case 1.1.2. (Otherwise).
Let sl be a (stream-item-list, containing, in the same order, 4stream-
itees equal to the 4stream-items in the fstream-item-list3 in ds which
follow the *stream-item+ designated by cp.
Case 1.2. fv is fabsents.
Let sl be the <character-string-value+ in the current <string-io-control>.
If sl contains no @ymbol)s then perform raise-condition(<error-condition>].
256
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 275

Step 2.
Case 2.1. sl can be parsed as "[{leading-delimiter-list)] {,I;3 Ifstream-item-list+l,l‘.
Let Id be that sequence in sl which satisfies 'CQeading-delimiter-list)1
I,(;3" in this parse, and let the number of its terminal nodes be In.
Perform input-stream-item(fv3 In times. Return a fcharacter-string-value+
containing the last (symbol? in Id.
Case 2.2. sl can be parsed as "~leading-delimiter-list~n.
Let In be the number of terminal nodes in sl. Perform input-stream-item
In times. Return *character-string-value,: fnull-character-strinq%
Case 2.3. sl can be parsed as '[Cleading-delimiter-list+1 ' C@tring-or-picture-
symbol-list)]".
Let In be the number of terminal nodes in sl. Perform input-stream-item
In times. Perform raise-condition(<error-condition>).
Case 2.4. sl can be parsed as '[<leading-delimiter-list)3 Cputative-data-constant)
[CbJ,(;3 [*stream-item-list913'.
Let In be the number of terminal nodes in sl preceding that part which
satisfies '[<stream-item-list93' in this parse. Perform input-stream-
item (fv3 In times. Let csv be a <character-string-value+ whose terminal
nodes are those of the part of sl which satisfies '(putative-data-constant)
[bI,J;l" except those terminals that are <linemark%. Delete from csv the
last $symbol), if that +aymbol) is a ki or a $,). Return csv.
8.7.1.5.3 Parsinq Cateqories for Data Directed Input
Some of the categories used in parsing input streams for data directed input are
categories of the Concrete Syntax or Machine state Syntax. Some of the categories are
defined in Section 8.7-1.4.2 "Parsing categories for list directed input'. The remainder
are defined as follows:
Cputative-name-field)::= {$data-symbol) 1 '3 [{field-element-l-list)3
Cfield-element-l)::= {data-symbol)1 * I,lMlUinemark+
{putative-data-constant?::= {simple-character-string-constant)
[{{data-symbol) I flinemark, I =3
[{field-element-2-list)13 1
Eldata-symbol) I =3
[{field-element-2-list)]
<field-element-2)::= {data-symbol)l<linemark,t=l'
8.7.1.6 Get-edit
Operation: get-edit(edi,fv)
where edi is an <edit-directed-input>,
fv is a Itfile-value*).
Step 1. Attach to the current fblock-control, a
fdata-item-control-list+:
fdata-item-control+:
<data-list-indicator. designating the first (input-target-list>
of edi
*data-item-indicator>:
*undefined,.
Chapter 85 Input/Output 257
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 276

Step 2. Attach to the current *block-control+ a
<format-control-list+:
*format-control>:
<format-specification-list-designator> designating the first <format-
specification-list> of edi
<format-list-index+:
<integer-value+:
0.
Step 3. Perform establish-next-data-item to obtain a *current-scalar-item,rndi:
<evaluated-target+,et; or tnone+,ndi. If ndi is *none% then go to Step 6.
Step 4. Perform establish-next-format-item to obtain a <format-itenD,efi.
Step 5.
Case 5.1. efi contains a <control-format>,ecf.
Perform execute-input-control-formattecf,fv). Go to Step 4.
Case 5.2. efi contains a <data-format>,edf.
Perform execute-input-data-format(et,edf,fW. Go to Step 3.
Step 6. Let dw be the first <data-item-control+ of the current <data-item-control-
list,.
Case 6.1. The <data-list-indicator, of dpp designates the last <input-target-list> of
edi.
Terminate this operation.
Case 6.2. (Otherwise).
Replace dpp by
<data-item-control+:
<data-list-indicator, designating the next <input-target-list> of edi
<data-item-indicator>:
<undefined+.
Replace the current <format-control-list, by
<format-control-list>:
*format-control>:
<format-specification-list-designator> designating the next
<format-specification-list> of edi
<format-list-indee:
<integer-value+:
0.
Go to Step 3.
8.7.1.6.1 Execute-input-control-format
Operation: execute-input-control-format(ecfrfv)
where ecf is a <control-format>,
fv is a Ctfile-value91.
Case 1. fv is a <file-value,.
Step 1.1. Let fi be the <file-information+ designated by fv. ecf must not have a
<paqe> , <line-format>, or <tab-format>,. Let ds be the <dataset+ designated
by the fdataset-designator+ in fi.
258
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 277

Step 1.2.
Case 1.2.1. ecf has a <space-format>: <integer-value9.w.
w must not be negative. If w=O then terminate this operation; otherwise
perform input-stream-item-for-edit(fv,i), for i=l,...,w.
Case 1.2.2. ecf has a <skip-format>: <integer-value9.w.
w must be greater than zero. Perform skip(w,fv).
Case 1.2.3. ecf has a <column-format>: <integer-value3.w.
Step 1.2.3.1. w must not be negative. Perform evaluate-current-column(fv) to
obtain cc. If w = 0 then let w be 1. Let n be the number of
fsymbolqs in ds which follow the <stream-item+ designated by the
<current-position9 in fi, up to the next <stream-item+ which has
a <linemark> or tomeqa,.
Step 1.2.3.2.
Case 1.2.3.2.1. cc < (w-1) and n 2 (w-cc-l).
Perform input-stream-item (w-cc-l) times.
Case 1.2.3.2.2. cc = (w-1).
Terminate this operation.
Case 1.2.3.2.3. cc > (w-l), or cc < (w-1) and n < (w-cc-l).
Perform skipcfinteger-value+:l;,fvl. Let nl be the number
of {symbolps in ds which follow the *stream-item+ designated
by the <current-position+ in fi, up to the next fstream-
item+ which has a <linemark+ or fomeqa,. If nl 2 (w-1) > 0
then perform input-stream-item(f v) (w-1) times.
Case 2. fv is <absent+.
ecf must have a <space-format>: <integer-value+,w. w must not be negative.
w=O then terminate this operation; otherwise perform input-stream item-foE ..-
edit(fv,i), for i=l,...,w.
8.7.1.6.2 Execute-input-data-format
Operation: execute-input-data-format(etredf,fv)
where et is an <evaluated-target>,
edf is a <data-format>,
fv is a [<file-value,].
Step 1.
Case 1.1. edf immediately contains a <real-format> or a <string-format>.
Let w be the first <integer-value+ in edf; w must be present.
Case 1.2. edf immediately contains a <picture-format>,pf.
Let w be the associated character-string length of pf. (see Section 9.5.2.1
Case 1.3. edf immediately contains a <complex-format>,ecf.
Step 1.3.1. If the first immediate component of ecf is a <real-format> then let wl
be the first *integer-value* in ecf; otherwise let wl be the associated
character-string length of the first component of ecf.
Chapter 8: Input/Output 259
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 278

step 1.3.2. If there is a second immediate component, SC, of ecf and SC is a <real-
format> then let w2 be the first <integer-value+ in SC. If there is an
SC and SC is a <picture-format> then let w2 be the associated character-
string length of SC. If there is no second immediate component of ecf
then let w2 be wl. Both wl and w2 must be non-negative.
step 1.3.3. Let w = wl+w2.
step 2. w must not be negative- If w=O then let v be a *basic-value+ : <character-
string-value+ : <null-character-strinq9;; and go to Step 6. Otherwise perform
input-stream-item-for-edit(fv,i), for i=l,...,w, to obtain w <stream-item%,
sicil each containing a +aymbol). If fv is a <file-value+ then attach to the
current <block-control+ a <current-file-value+: fv. Let csv be a fcharacter-
string-value, containing, in order, the w <character-value% cvlil, i=l,...,w,
where cvCi1 contains the $s.ymbol) sitil, for i=l,...,w.
Step 3. If edf has a <complex-format>,ecf, then perform Steps 3.1 through 3.7.
Step 3.1. If wl=O then let csvl be a *character-string-value+:
<null-character-strinq9. Otherwise let csvl be a <character-string-value+
obtained by deletinq the last w2 <character-value% from a copy of csv. If
w2=0 then let csv2 be a <character-string-value,: *null-character-strinqs.
Otherwise let csv2 be a icharacter-strins-value, obtained bv deleting the
first wl *character-value% from a copy of csv.
Step 3.2. Let cl be the first component of ecf. If cl is a <real-format> then let dfl
be its immediate component; otherwise let dfl be cl.
Step 3.3. If ecf has no second component then let df2 be dfl. Otherwise let c2 be the
second component of ecf; if c2 is a <real-f ormat> then let df2 be its
immediate component; otherwise let df2 be c2.
Step 3-Q. Perform validate-input-format(dfl,csvl) to obtain a <value-and-type+,rl or
finvalid+,rl. If rl is <invalid+ then let intg be its immediate component.
and perform raise-io-condition(<conversion-condition>,fv,csvl,intg); on
normal return let csvl be the immediate component of the current <returned-
onsource-value, and go to Step 3.4.
Step 3.5. Perform validate-input-format(df2,csv2) to obtain a <value-and-type+,r2 or
finvalid+,r2. If r2 is <invalid+ then let intg be its immediate component
and perform raise-io-condition(<conversion-condition>,fv,csv2,intg); on
normal return let csv2 be the immediate component of the current treturned-
onsource-value, and go to Step 3.5.
Step 3.6. Let vl and v2 be the first components of rl and r2, respectively. Let dtl
and dt2 be the second components of rl and r2, respectively. Let adt be a
<data-type> containing <real> and <decimal>, and <scale> and <precision>
defined as follows:
Case 3.6.1. dtl has <fixed>. and <precision>: r s; and dt2 has <fixed>, and
<precision>: t u.
adt has <scale>: <fixed>; <precision>: p q; where:
p = min(N,max(r-s,t-ul+max(s,u))
q = max(s,u)
N = maximum <number-of-digits> for <fixed> and <decimal>.
Case 3.6.2. (Otherwise).
Let r and t be the <number-of-digits> of dtl and dt2, respectively. adt
has <scale>: <float>; <number-of-digits>: min(N,max(r,t));, where N is
the maximum <number-of-digits> for <float> and <decimal>.
Step 3.7. Perform convert(adt,dtl,vl) to obtain a <real-value> containing the freal-
number~,cvl, and perform convert(adt,dt2,v2) to obtain a *real-value>
containing the <real-number+,cv2. Let bv be a fbasic-value, containing a
<complex-values: cvl cv2. Let edd be an *evaluated-data-description9
containing an <item-data-description> containing a <data-type> with <mode>:
<complex>; but otherwise as adt. Go to Step 7.
260
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 279

Step 4. If edf immediately contains a <real-format>,f or a <string-format>,f, then
perform steps 4.1 through 4.3.
Step 4.1. Let df be the immediate component of f.
Step 4.2. Perform validate-input-f ormat (df,csv) to obtain a <value-and-type,,v, or a
<character-string-values,v, or finvalid+,v. If v is *invalid+ then let intg
be its immediate component and perform raise-io-condition (<conversion-
condition>,fv,csv,intg); on normal return let csv be the immediate component
of the current <returned-onsource-value, and go to Step 4.2.
Step 4.3. Go to Step 6.
Step 5. If edf immediately contains a <picture-format>,pf, then perform Step 5.1.
Step 5.1. Perform validate-input-format(pf,csv) to obtain a <value-and-type9,v. or a
<character-string-value+,v, or finvalid9.v. If v is *invalid+ then let intg
be its immediate component and perform raise-io-condition (<conversion-
condition>,fv,csv,intg); on normal return let csv be the immediate component
fo the current *returned-onsource-value+ and go to Step 5.1.
Step 6.
Case 6.1. v is a <character-string-value, or a <bit-string-value+.
Let edd be an <evaluated-data-description+ containing an <item-data-
description> of <character> or <bit> (respectively) and <maximum-length>:
<asterisk>. Let bv be a <basic-value,: v.
Case 6.2. v is a <value-and-type*: val adt.
Let edd be an <evaluated-data-description, containing an <item-data-
description> containing adt. Let bv be a *basic-value>: val.
Step 7. Let agv be an
*aggregate-value>:
<aggregate-type>:
*scalar>;
<basic-value-list,:
bv.
Let dd be the <data-description> immediate component of ead. Perform
assign(et,agv,dd). If fv is a <file-value+ then delete the current tcurrent-
file-value+.
8.7.1.6.2.1 Validate-input-format
<invalid,::= <integer-values
Operation: validate-input-format(df,csv)
where df is an immediate component of a <real-format> or a <string-format>,
or is a <picture-format>,
csv is a <character-string-value,.
result: <invalid, or <character-string-value+ or &it-string-value+ or
<value-and-type,.
Case 1. af is a <picture-format>.
Case 1.1. df contains <pictured-numeric>,.
Perform validate-numeric-pictured-value(df,csv) to obtain a fpicture-
validity9rpv. If pv has <picture-invalid*: val; then return <invalid+: val.
If pv has <picture-valid+: val; then return a *value-and-type,: val aadt;
where aadt is the associated arithmetic data-type of df.
Chapter 8: Input/Output 261
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 280

Case 1.2. df contains <pictured-character>.
Perform validate-character-pictured-value(df,csv) to obtain a tpicture-
validity+,pv. If pv has <picture-invalid,: val; then return <invalid+: val.
Otherwise return csv.
Case 2. df is a <character-format>.
Return csv.
Case 3. df is a <bit-format>.
Case 3.1. csv conforms to the syntax 'C$kGlist)l Cbs-list1 C$J+list)l- where bs is one
of the $symbol)s in the first column of Table 4.2 corresponding to the
<radix-factor> in df.
Let bc be that part of csv which satisfies 'Cbs-list]'. Let bcx be the
<character-string-value+ obtained by concatenating bc with a $I*) on the
left, and, on the right, with a 'El, 'B2. 'B3 or 'B4, according to the
<radix-factor> of df. Perform basic-bit-value (bcx) to obtain a tbit-string-
value%,bsv. Return bsv.
Case 3.2. (Otherwise).
Let intg be the smallest value such that the first intg $symbol)s in csv do
not have a valid continuation according to this syntax. Return <invalid>:
intg.
Case 4. df is a <fixed-point-format>.
Step 4.1. Let w, d and s be respectively the <integer-value% in df. If either d or s
(or both) is <absent+ then 0 is assumed. d must not be negative.
Step 4.2.
Case 4.2.1. csv conforms to the syntax "{la-list+* or is a <character-string-value,:
<null-character-strinq+.
Return Cvalue-and-type+: <real-value+: 0; adt; where adt is a <data-
type> containing <real>, <fixed>, <decimal>, <number-of-digits>: 1; and - - <scale-factor>: 0.
Case 4.2.2. csv conforms to the syntax "C$&list*l [+(-I Cdecimal-number) [$b-
1ist)l".
Perform basic-numeric-value(csv) to obtain a <value-and-types: v dt. If
csv contains a C-2 then let s.=s i otherwise, let s*=s-d. Return a
<value-and-type+: seal-value,: (v*lOts'): adt; where adt is a <data-
type> equal to dt except that <scale-factor> is decremented by s’.
Case 4.2.3. (Otherwise).
Let intg be the smallest value such that the first intg $symbol$s in csv
do not have a valid continuation according to the syntax in Case 4.2.2.
Return *invalid+: intg.
Case 5. df is a <floating-point-format>.
Step 5.1. Let W, d, and s be respectively the <integer-value% in df. If s is
present, it is ignored. If d is <absent* then let d be 0. d must not be
negative.
Step 5.2.
Case 5.2.1. csv conforms to the syntax 'Sk+list)" or is a 4character-string-value*:
fnull-character-strinqs.
Return <value-and-type+: <real-value*: 0; adt; where adt is a <data-
type> containing <real>, <fixed>, <decimal>. <number-of-digits>: l;, and
<scale-factor>: O.-
262
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 281

Case 5.2.2. csv conforms to the syntax "[$&list)1 [+1-l {decimal-number)
C{CElC+I-lIE1 $integer)l [$a-1ist)l".
If csv contains a sign, but not an E, after a substring conforming to
{decimal-number*, then replace that substring with the concatenation of
that substring with E. Perform basic-numeric-value(csv) to obtain a
<value-and-types: v dt. If csv contains a C.2 then let d'=O; otherwise
let d'=d. Return a <value-and-type%: <real-value+: (v*lOt-d'); adt;
where adt is a <data-type> equal to dt except that its <scale-factor>
(if any) is incremented by d'.
Case 5.2.3. (Otherwise).
Let intg be the smallest value such that the first intg $symbolps in csv
do not have a valid continuation according to the syntax in Case 5.2.2.
Return *invalid+: intg.
8.7.1.7 Input-stream-item
Operation: input-stream-item(fv1
where fv is a [<file-value,].
Step 1.
result: a <stream-item* or fomecra%
Case 1.1. fv is a *file-value+.
Step 1.1.1. Let fi be the *file-information9 designated by fv; let cp be the
*current-position> in fi, and let ds be the <dataset> designated by the
fdataset-designator> in fi.
Step 1.1.2.
Case 1.1.2.1. cp designates <alpha+.
Replace the immediate component of cp by a <designators to the
first <stream-item9 in ds, or to the fomeqa, in ds if there are
no <stream-item% in ds.
Case 1.1.2.2. cp designates a <stream-item9.
Replace the immediate component of cp by a <designator> to the
next <stream-item+ in ds , or to the fomeqa, in ds if there are no
further <stream-item% in ds.
Case 1.1.2.3. cp designates fomeqa,.
Go to Step 1.1.3.
Step 1.X.3. Let si be the node designated by cp.
Case 1.2. fv is tabsent9.
Let sioc be the current <string-io-control,. Let csv be the fcharacter-
string-value> in sioc.. If csv contains a fnull-character-strinqs then
perform raise-condition(<error-condition>). Otherwise let si be a fstream-
item,: v;, where v is the first (Symbols in CSV. If csv contains only one
Gvm~lt then replace csv by <character-string-value+:
fnull-character-strinq9. Otherwise delete the first &character-value> in
csv.
step 2. If there is a <copy-file, in the current <block-state, then let cf be its
immediate component; otherwise go to step 3. If si is not fomecra9 or <linemark+
then perform output-string-item(symrcf), where sym is a <stream-item+ containing
the {symbol) in si. If si is flinemark, then perform skipttinteger-
value+:l;,cf).
step 3. Return si.
Chapter 8: Input/Output 263
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 282

8.7.1.8 Basic-character-value
Operation: basic-character-value(csvin)
where csvin is a *character-string-value*.
result: a <character-string-value+.
step 1. The +symbol+s of csvin must conform to the syntax of $simple-character-string-
constant).
step 2. Let csvo be a copy of csvin. Remove the first and last *character-value% of
csvo I If no <character-value% remain then return *character-string-value*:
<null-character-strinq,.
Replace all adjacent pairs of <character-value,: $symbol): $'); originally in
csvo with a single *character-values: $symbol): $').
step 3. Return csvo.
8.7.1.9 Basic-bit-value
Operation: basic-bit-value(csvin)
where csvin is a <character-string-value*.
result: a &bit-string-value+.
Step 1. csvin must conform to the syntax of Tsimple-bit-string-constant?.
Step 2. Let m be 1, 1, 2, 3, or 4 according as the Cradix-factor? in csvin is B, Bl, B2,
83, or B4. Remove the portion of csvin corresponding to the syntax for {radix-
factor) and remove the first and last of the remaining *character-value?s. Let
n be the number of fcharacter-value% remaining, if any. If n=O then return
ibit-string-value,: <null-bit-strinq,.
Step 3. Let syiil, for i=l,...,n, be the $symbol)s, in order, of the remaining
<character-value% in csvin. Each syiil must have an entry in Table 4.2 which
is valid for the value of m. Let bsv be a <bit-string-value+, containing m*n
<bit-value%, such that <bit-value% (i*m+l-m) through (i+m) are obtained from
Table 4.2 as a function of m and syiil, for i=l,...,n.
Step 4. Return bsv.
8.7.1.10 Input-stream-item-for-edit
Operation: input-stream-item-for-edit(fv,i)
where fv is a Cffile-value,],
i is an *integer-value*.
result: a *stream-item+.
Step 1. Perform input-stream-item to obtain a <stream-item+,si or an fomega>,si.
Step 2.
Case 2.1. si is 4omeqa3.
If i=l then perform raise-io-condition(<endfile-condition>,fv); otherwise
perform raise-condition(<error-condition>).
Case 2.2. (Otherwise).
If si contains <linemark+ then go to Step 1. Otherwise return si.
264

264
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 283

8.7m.2 THE PUT STATEMENT
8.7.2.1 Execute-put-statement
Operation: execute-put-statement(ps1
where ps is a <put-statement>.
Step 1.
Case 1.1. ps has a <put-file>,pf.
Perform execute-put-filetpf).
Case 1.2. ps has a <put-string>,pstr.
Perform execute-put-string(pstr).
step 2. Perform trim-io-control.
Step 3. Perform normal-sequence.
8.7.2.2 Execute-put-file
Operation: execute-put-file(pf)
where pf is a <put-file>.
Step 1. Perform Steps 1.1, 1.2, and 1.3 in any order.
Step 1.1. Let fo be the <value-reference> of the <file-option> in pf. Perform
evaluate-file-option( to obtain a <file-value>.fv.
Step 1.2. If pf has a <skip-option>,sko, then let e be the <expression> in sko, and
perform evaluate-expression-to-integer(e), to obtain an *integer-value>,sk.
Step 1.3. If pf has a <line-option>,lopt, then let el be the <expression> in lopt, and
perform evaluate-expression-to-integercell, to obtain an <integer-value+,l.
Step 2. Let fi be the <file-information, designated by fv. If fi contains t-9 then
go to Step 4; otherwise go to Step 3.
Step 3. Let efal be an *evaluated-file-description-list* containing <stream> and
<output>. Perform open(fv,efdl) to obtain a result rf. If rf is - ffail, then
perform raise-io-condition(<undefinedfile-condition>,fv). If on normal return
fi contains <closed, then perform raise-condition(<error-condition>).
Step 4. fi must contain <stream> and <output>.
Step 5. If pf immediately contains <e> then perform put-page(fv).
step 6. If pf has a <line-option> then perform put-line(l,fv).
Step 7. If pf has a <skip-option> then perform skip(sk,fv).
step 8. If pf has an <output-specification> then:
Case 8.1. pf has a <list-directed-output>,ldo.
Perform put-list(ldo.fv).
Case 8.2. pf has a <data-directed-output>,ddo.
Perform put-data(ddo,fv).
Case 8.3. pf has an <edit-directed-output>,edo.
Perform put-edlt(edo,fv).
Chapter 8: Input/Output 265
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 284

8.7.2.3 Execute-put-string
Operation: execute-put-strinq(pstr1
where pstr is a <put-string>.
step 1. Let tr be the <target-reference> in pstr. Perform evaluate-target-referencectr)
to obtain an <evaluated-target+,et.
Case 1.1. et has a <generation+.
Let n be the <maximum-length> contained in et.
Case 1.2. et has an <evaluated-pseudo-variable-reference+.
Case 1.2.1. et contains <onchar-pv>.
There must be a current fonchar-values. Let n=l.
Case 1.2.2. et contains <onsource-pv>.
There must be a current fonsource-values. Let n be the length of the
current fonsource-value,.
Case 1.2.3. et contains <substr-pv>.
If et contains four elements, let n be the fourth of its elements.
Otherwise let n be lng-st+l, where lng is the length of the value of the
<generation+ contained in et and st is the third element of et.
Step 2. Attach to the current tblock-control, a
<string-io-control+:
<character-string-value+:
<null-character-strinq,;
*string-limit>:
<integer-value+:
n.
Step 3.
Case 3.1. pstr has a <list-directed-output>,ldo.
Perform put-list(ldo).
Case 3.2. pstr has a <data-directed-output>,ddo.
Perform put-datafddo).
Case 3.3. pstr has an <edit-directed-output>,edo.
Perform put-edit(edo).
Step 4. Let OS be the <character-string-value+ of the current <string-io-control9. Let
m be the number of $symbol)s in OS. Perform assign(et,agv,ddl where agv is an
<aggregate-value+ containing OS, and dd is a <data-description> containing
<character>, <nonvarvinq>, and <maximum-length> with m.
Step 5. Delete the <string-io-control, from the current <block-control>.
266
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 285

8.7,.2.4 Put-list
Operation: put-list(ldo,fvl
where ldo is a <list-directed-output>,
fv is a Ctfile-value,].
step 1. Attach to the current *block-control+ a
<data-item-control-list%:
fdata-item-control,:
<data-list-indicator+ designating the <output-source-list> of ldo
fdata-item-indicator+:
<undefined%.
Step 2. If fv is a <file-value+ then let fi be the <file-information, designated by fv.
step 3. Let t be an <integer-value,.
Case 3.1,. fv is a <file-value+ and fi contains <print>.
Step 3.1.1. Perform evaluate-current-column(fv) to obtain cc.
Step 3.1.2. If the devaluated-tab-option+ of fi contains an <integer-value+,iv, such
that cc=iv-1, then let t=O; otherwise let t=l.
Case 3.2. (Otherwise).
Let t=o.
Step 4,. Perform establish-next-data-item to obtain a <current-scalar-item.,ndi or
fnone*,ndi. If ndi is <none, then
item,:~symbol):LI;:r
perform
fv) and tanate this operation.
output-string-item(fstream-
Step 5. Let sdt be the <data-type> in ndi. and let tdt be a <data-type> simply
containing <character>, <nonvarvinq>, and <maximum-length>: <asterisk>. Perform
convert(tdt.sdt. bv). where bv is the basic-value9 in ndi, to obtain a
<character-string-value+,csv.
Step 6.
Case 6.1. sdt contains <bit>-
Let cv be a <stream-item-list, containing, in order, the $symbol~s
the $symbol)s of csv, if any,
Case 6.2. sdt contains <character> or <pictured-character>, and either fv is <absent,
or fv is a <file-value+ and fi does not contain <print>.
Let cv be a <stream-item-list+ containing, in order, the $symbol+s
the $symbol+s of csv, if any, but with each {symbol): $*I; replaced
by two occurrences of {symbol): $*a;
C'?
Case 6.3. (Otherwise)-
Let cv be a <stream-item-list9 containing, in order, the $symbol)s of csv.
Step 7.
Case 7.1. fv is a <file-value+ and fi contains <print>.
Perform tabft,fvl.
Chapter 8: Input/Output 267
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 286

Case 7.2.
Step 8. Let
Step 9.
Case 9.1.
Case 9.2.
(Otherwise).
then perform
gymbz;: Ib;;;,fv).
output-string(<stream-item-list*: fstream-item,:
t=1.
fv is a *file-value%
Perform evaluate-current-column(fv) to obtain ccol. Let lsz be the
<evaluated-linesize, in fi. Let lcv be the number of {symbolps in cv. If
lcv > (Is77cc011 and ccol+O then perform skipcfinteger-value~:l;rfv).
Perform output-string(cv,fv).
fv is *absent,.
Perform output-string(cv,fv).
Step 10. Go to Step 4.
8.7.2.5 Put-data
Operation: put-data(ddo,fv)
where ddo is a <data-directed-output>,
fv is a Ctfile-value,].
Step 1.
Case 1.1. ddo contains a <data-source-list>,dsl.
Go to Step 2.
Case 1.2. ddo does not contain a <data-source-list>.
Let dsl be a <data-source-list> which contains an implementation-defined
list of trees of the form
<data-source>,t:
<variable-reference>:
<declaration-designator>,dd
[<identifier-list>]
<data-description>,ddesc;;
where dd designates a <declaration> which contains <variable> but does not
contain <based> without a subnode and ddesc does not contain a <non-
computational-type>.
Step 2. Attach to the current fblock-control+ a
<data-item-control-list+:
<data-item-control+:
<data-list-indicator+ designating dsl
*data-item-indicator.:
<undefined>.
Step 3. If fv is a <file-value+ then let fi be the <file-information, designated by fv.
Step 4. Let t be an <integer-value+.
Case 4.1. fv is a <file-value+ and fi contains <print>.
Step 4.1.1. Perform evaluate-current-column(fv) to obtain cc.
Step 4.1.2. If the <evaluated-tab-option, of fi contains an <integer-value+,iv, such
that cc=iv-1 then let t=O; otherwise let t=l.
268
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 287

Case 4.2. (Otherwise).
Let t=O.
Step 5. Perform establish-next-data-item to obtain a <current-scalar-iten,ndi or
fnone+,ndi. If ndi is <nones then perform output-string-item(sc,fv) where SC is
a <stream-item+: <symbol): C;);; and terminate this operation. Otherwise ndi is
a
<current-scalar-item+:
*basic-value+,bv
<data-type>,sdt
<data-name-field,:
{symbol-list),n.
sdt must contain <computational-type>-
Step 6. Let n be a <stream-item-list> containing, in order. the Csymbolqs of the fdata-
name-field+ in ndi.
Step 7. Let tdt be a <data-type> simply containing <character>, <nonvaryinq>, and
<maximum-length>: <asterisk>. Perform convert(tdt,sdt,bv) to obtain a
<character-string-value+,csv.
Step 8.
Case 8.1. sdt contains <bit>.
Let cv be a <stream-item-list+ containing, in order, the {symbolqs
S‘? the +symbol)s of csv, if any,
i'?
Case 8.2. sdt contains <character> or <pictured-character>.
Let cv be a <stream-item-list+ containing, in order, the $symbol)s
C'? the +symbol)s of csv. if any, but with each #%ymbol): C'); replaced by
two occurrences of Csymbol): $'p;
c-a
Case 8.3. (Otherwise).
Let cv be a <stream-item-list, containing, in order, the $symbolJ+s of csv.
Step 9.
Case 9.1. fv is a <file-value, and fi contains <print>.
Perform tab(t,fv).
Case 9.2. (Otherwise).
then perform output-stringffstream-item-list+:
;:y*:i;: b;;;.fv).
<stream-item,:
Step 10. Let t=l.
Step 11.
Case ll.l.fv is a <file-value+.
step 11.1.1. Perform evaluate-current-column(fv) to obtain ccol. Let lsz be the
<evaluated-linesize, in fi. Let lcv be the number of (symbol)s in cv,
and let In be the number of $symbol)s in n.
Step 11.1,.2. If (ln+l) > (lsz-cc011 and cc01 + 0 then perform skipcfinteger-
value+:l;,fv).
Ch'apter 8: Input/Output 269
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 288

Step 11.1.3. Perform output-string(n,fvl. Perform
item: jsymbol): $=*;;,fvl.
output-string-item(fstream-
Step 11.1.4. Perform evaluate-current-column(fv) to obtain ccol. If 1cv > (lsz-
ccol) and cc01 + 0 then perform skiptcinteger-value,:l;,fvJ.
Step 11.1.5. Perform output-string(cv,fvl.
Case 11.2.fv is <absent+.
Perform output-string(n,fv); perform output-string-itemffstream-item,:
{symbol): $=);;,fv); perform output-string(cv,fv).
Step 12. Go to Step 5.
8.7.2.6 Put-edit
Operation: put-edit(edo,fv)
where edo is an <edit-directed-output>,
fv is a [<file-value,].
Step 1. Attach to the current <block-control3 a
fdata-item-control-list>:
<data-item-control,:
<data-list-indicator+ designating the first <output-source-list>
of edo
<data-item-indicator,:
<undefined,.
Step 2. Attach to the current fblock-control, a
<format-control-list*:
*format-control>:
<format-specification-list-designator> designating the first <format-
specification-list> of edo;
*format-list-index,:
tinteger-values:
0.
Step 3. Perform establish-next-data-item to obtain a <current-scalar-item.rndir or
&one>,ndi. If ndi is t-9 then go to Step 6.
Step 4. Perform establish-next-format-item to obtain a <format-item>,efi.
Step 5..
Case 5.1. efi contains a <control-format>,ecf.
Perform execute-output-control-format(ecf,fv). Go to step 4.
Case 5.2. efi contains a <data-format>,edf.
Perform execute-output-data-format(ndi,edfrfv). Go to step 3.
step 6. Let dpp be the first <data-item-control+ of the current *data-item-control-
list+.
Case 6.1. The <data-list-indicator+ of dpp designates the last <output-source-list> of
edo.
Terminate this operation.
270
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 289

Case 6.2. (Otherwise).
Replace dpp by
<data-item-control+:
<data-list-indicator+ designating the next <output-source-list> of
edo
<data-item-indicator+:
fundefine-.
Replace the current tformat-control-list+ by
<format-control-list+:
*format-control+:
<format-specification-list-designator> designating the next
<format-specification-list> of edo;
<format-list-index+:
*integer-value%
0.
Go to Step 3.
8.7.2.6.1 Execute-output-control-format
Operation: execute-output-control-format(ecf,fv)
where ecf is a <control-format>,
fv is a [<file-value91.
Step 1. If fv is a <file-value+ then let fi be the <file-information+ designated by fv.
If fv is 4absent+ then ecf must have a <space-format>.
Step 2.
Case 2.1. ecf has a <space-format>: <integer-value9,w.
W must not be negative. If w=O then terminate this operation. Perform
output-string(str,fv) where str is a *stream-item-list, whose w terminal
nodes are all bs.
Case 2.2. ecf has a <skip-format>: <integer-value+,w.
w must not be negative. Perform skip(w,fv).
Case 2.3. ecf has a <line-format>: <integer-value+,w.
Perform put-line(w,fv).
Case 2.4. ecf has <w>.
Perform put-page(fv).
Case 2.5. ecf has a <tab-format>: iinteger-value+,w.
w must not be negative. Perform tab(w,fv).
Case 2.6. ecf has a <column-format>: *integer-value3.w.
Step 2.6.1. Perform evaluate-current-column0 to obtain cc. W must not be
negative. If w = 0 or w exceeds the <evaluated-linesize, in fi then let
w be 1.
Step 2.6.2.
Case 2.6.2-l. cc < w-l.
Perform output-string(str,fv), where str is a <stream-item-list,
containing (w-cc-l) terminal nodes, each of which is a b.
Chapter 8: Input/Output 271
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 290

Case 2.6.2.2. cc = w-l.
Terminate this operation.
Case 2.6.2.3. cc > w-l.
Perform skipffinteger-value>:l;,fv). If w>l then perform
output-string(str,fv), where str is a <stream-item-list>
containing (w-1) terminal nodes, each of which is a L1.
8.7.2.6.2 Execute-output-data-format
Operation: execute-output-data-format(csi,edf,fv)
where csi is a <current-scalar-item+,
edf is a <data-format>,
fv is a [*file-value+l.
Case 1. edf contains a <bit-format>,ebf.
Let m be the value of the <radix-factor> in ebf. Perform convert(tdt,sdt,bv) to
obtain a <bit-string-value+,bsv, where tdt is a <data-type> containing <bit>,
<nonvarvinq>, and <maximum-length>: <asterisk>:, sdt is the <data-type> of csi,
and bv is the <basic-value+ immediate component of csi. Let n be the number or
immediate components of bsv. Let nn=n. If n is not a multiple of m, then let
nn be the next higher multiple of m and insert (nn-n) <bit-value>: fzero-biti;s
at the end of the fbit-value-list, of bsv. Let n=nn. Let k=n/m. If ebf
contains an <integer-value*,w, and >
letn k=w.
w*m then perform raise-
condition(<strinqsize-condition>) and If ebf has no finteger-value,
then let w be k. If w-0 then terminate this operation. Let csv be a fstream-
item-list, containing w $symbol)s as follows:
The first k contain, in order, the (symbol+ in Table 4.2 corresponding to
successive groups of m <bit-value% in bsv.
The remaining w-k contain Is.
Perform output-string(csv,fv).
Case 2. edf contains a <character-format>,ecf.
Let tdt be a <data-type> containing <character> and Unaximum-length>: the
finteger-value, in ecf (or <asterisk> if there is no <integer-value, in ecf);
let sdt be the <data-type> and v the <basic-value> of csi. Perform
convert(tdt,sdt,bvl to obtain a <character-string-value9,csv. Let sil be a
<stream-item-list> containing, in order, the ~symbol)s of csv. Perform output-
string(sil,fv).
Case 3. edf immediately contains a <picture-format> with a <pictured> node, pit.
Let sdt be the <data-type> and bv the <basic-value> of csi. Perform
convert(pdt,sdt,bv), where pdt is a <data-type> containing pit, to obtain a
<character-string-value+,csv. Let sil be a <stream-item-list3 containing, in
order, the $symbol)s of csv. Perform output-string(sil,fv).
Case 4. edf contains a <complex-format>,ecf.
Step 4.1. Let sdt be the <data-type> of csi. Let bv be the <basic-value, of csi.
Case 4.1.1,. sdt contains <character>, <pictured-character>, or <bit>.
Perform convert-to-arithmetic(sdt, bv) to obtain a <value-and-type+
either with components rp and rat, or with immediate components rp, rat,
ip, and idt. If ip and iat do not exist, let ip be a freal-value,: 0;
and let idt be rdt.
272
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 291

Case 4.I.2. sdt contains <real> (including <pictured-numeric>).
Let rp be bv. Let rdt and idt be sdt. Perform convert(idt,dt,v) to
obtain ip, where v is a <real-value+: 0; and dt is a
containing
<data-type>
<real>, <fixed>, <decimal>, <number-of-digits>: I;, and
<scale-factor>: 0.
Case 4.1.3. sdt contains <complex> (including <pictured-numeric>).
Step 4.2. Let
Let rdt and idt be <data-type>s containing (real> but otherwise as sdt.
If sdt does not contain <pictured-numeric>, let rp and ip be fbasic-
value% which respectively contain the two <real-number% in csi. If
sdt has <pictured-numeric>, let rp and ip be *character-string-valuess
containing, respectively, the first n and last n <character-value+s in
csi, where n is the associated character-string length of rdt and idt.
Cl be the immediate component of the first immediate component of ecf.
If ecf has a second immediate component, SC, then let c2 be the immediate
component of sc; otherwise let c2 be cl.
Step 4.3. If cl is <pictured>, then perform convert(pl,rdt,rp) to obtain a fcharacter-
string-value>,csvl, where pl is a <data-type> containing cl. Otherwise
perform edit-numeric-output(rp,rdt,cl) to obtain csvl.
Step 4.4. If c2 is <pictured>, then perform convert(p2,rdt,ip) to obtain a <character-
string-value,,csv2, where p2 is a <data-type> containing c2. Otherwise
perform edit-numeric-output(ip,idt,c2> to obtain csv2.
Step 4.5. Let sil be a fstream-item-list+ containing, in order, the fsymbol)s of csvl
and csv2. If sil contains one or more +symbol)s then perform output-
stringfsil,fv).
Case 5. (Otherwise).
Let fpf be the <fixed-point-format> or <floating-point-format> in edf. Perform
edit-numeric-output(bv,dt,fpf) to obtain a <character-string-value,,csv, where
bv is the fbasic-value, in csi and dt is the <data-type> in csi. Let sil be a
4stream-item-list+ containing, in order, the $symbol)s of csv,. If sil contains
one or more <symbol)s then perform output-string(sil,fv).
8.7.2.6.3 Edit-numeric-output
Operation: edit-numeric-output(bv,sdt,fpf)
where bv is a <basic-value+,
sdt is a <data-type>,
step 1.
fpf is a <fixed-point-format> or a <floating-point-format>.
result: a <character-string-value+.
Case 1.1. bv has a freal-value+,cv.
Let cdt be sdt.
Case 1.2. bv has a tcomplex-value,.
Let cv be a <real-value, containing the real part of bv; let cot be a <data-
type> containing <real> but otherwise as sdt.
Case 1.3. bv has a <character-string-value9 or a fbit-string-value,.
Perform convert-to-arithmetic(sdt,v) to obtain a <value-and-type+ with first
two components cv and dt, where v is the immediate component of bv. Let cdt
be a <data-type> containing dt.
Step 2. Let w, d, and s be (respectively) the <integer-value% in fpf, if they exist.
Chapter 8: Input/Output 273
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 292

Step 3.
Case 3.1. fpf is a <fixed-point-format>.
Step 3.1.1. If the second <integer-value+ is missing from fpf, let d=O; if the third
is missing, let s=O. d must not be negative.
Step 3.1.2. Let tdt be a <data-type> containing <real>, <fixed>, <decimal>, <scale-
factor>:d. N, used below, is implementation-defined.
Case 3.1.2.1. d = 0 and cv b 0.
W must satisfy w>O. Let n=w, p=min(n,N), i=w-p, and j=p-1. Let
pit be '(i)B(j)Z9'.
Case 3.1.2.2. d = 0 and cv < 0.
W must satisfy w>l. Let n=w-1, p=min(n,Nl, i=w-p-l. and j=p.
Let pit be '(i)B(j)-9'.
Case 3.1.2.3. d > 0 and cv 2 0.
w must satisfy w>d+l. Let n=w-1, p=min(n,N). i=w-p-l, and j=p-d-
1. Let pit be '(i)B(j)Z9V.(d)9'.
Case 3.1.2.4. d > 0 and cv < 0.
Step 3.1.3.
Case 3.2. fpf is a <floating-point-f ormat>.
Step 3.2.1. If the second and third <integer-value+s are missing from fpf, let s be
the converted <number-of-digits> of sdt for a target <data-type> of
<real>, <float>, <decimal>, and let d=s-1.
value> is missing, let s=d+l.
If only the third finteger-
d must not be less than 0 and s must not
be less than d.
Step 3.2.2.
Step 3.2.3.
Step 3.2.4.
w must satisfy w>d+2. Let n=w-2, p=min(n,N), i=w-p-2, and j=p-d.
Let pit be '(i)B(j)-9V.(d)9'.
Attach <number-of-digits>: P; to tdt. Remove from pit any repetition
factor equal to 0 and the character following it. Let pdt be the <data-
type> corresponding to the {picture* with concrete-representation pit,
defined above. (For a fuller treatment, see Section 4.4.6). Let x be
the <real-value+ calculated as x=cv*lOts+0.5*sign(cv)*lO1-d. Perform
convert(tdt,cdt.x) to obtain cv3 and then perform edit-numeric-
picture(cv3,pdt) to obtain a <character-string-value3,csv. Return CSV.
Let tdt be a <data-type> which has <a>, <float>, <decimal>, <number-
of-digits>: s. Perform convert-to-float-decimal(tdt,cdt,cvJ to obtain a
freal-value>,cv3.
There exists a unique representation of cv3 in the form vm*lOtve, where
ve is an integer and either vm=ve=O or lOt(s-d-l)labs(vm)<lOt(s-d>.
If vm>O, let i=O; otherwise, let i=l. Let j=max(s-d.1). If d=O, let
k=O; otherwise let k=l. Let n be the implementation-defined size of the
exponent field for <floating-point-format>s. Let m=w-i- j-k-d-2-n.
(Informally, i indicates the need for a sign position in the mantissa
field; j is the number of digit positions before the decimal point; k
indicates the need for a decimal point in the mantissa field; m is the
excess of the field-size over the requirements of the specification for
the particular value.1 If m<O or abs(ve)>lOtn, perform raise-
condition(<size-condition>).
274
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 293

Step 3.2.5. Let picx be the <picture* with concrete-representation '(m)B(i)-
(j)9V(k).(d)9*.. Remove from picx any repetition factor whose value is
zero and the character following it. Let picm be the <data-type>
corresponding to picx. Perform edit-numeric-picture(rvm,picml to obtain
a <character-string-value>,cvm, where rvm is a <real-value+ containing
vm. Let pice be the <data-type> corresponding to the $picture> with
concrete-representation *S(n)9'. Perform edit-numeric-picture(rve,pice)
to obtain a <character-string-value3,cve, where rve is a *real-value+
containing ve. Return a <character-string-value+ containing the
concatenation of cvm, the icharacter-value,: $symboll: E;;, and eve.
8.7.2.7 Output-strinq
Operation: output-strinq(sil,fvl
where sil is a <stream-item-list+,
fv is a [<file-value+].
step 1. For each <stream-item+,si, in sil in order, perform output-string-item(si,fvl.
8.7.2.8 Output-strinq-item
Operation: output-strinq-item(si,fvJ
where si is a <stream-item>,
fv is a [*file-value>].
step 1. If fv is a tfile-value, then let fi be the <file-information+ designated by fv.
step 2. If fv is a <file-value+ then perform Steps 2.1 and 2.2.
Step 2.1. Perform evaluate-current-column(fv) to obtain an <integer-value,.cc, and
perform evaluate-current-line(fv) to obtain an <integer-value+,cl. Let lsz
be the <evaluated-linesize+ in fi, and psz be the <evaluated-pagesize+ in
fi, where they exist.
Step 2.2.
Case 2.2.1. fi contains <print>, cc = lsz and cl = psz.
Perform output-stream-item(fstream-item~:~linemark~;,fv). Perform
raise-io-condition(<endpaqe-condition>,fv), and on normal return go to
step 2.1.
Case 2.2.2. cc = 1s~. but Case 2.2.1 does not apply.
Perform output-stream-itemcfstream-item+: flinemark,;,fv).
Case 2.2.3. (Otherwise).
Go to step 3.
step 3. Perform output-stream-item(sirfv).
Chapter 8: Input/Output 275
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 294

8.7.2.9 Output-stream-item
Operation: output-stream-item(si,fv)
Case 1.
Case 2.
where si is a <stream-item,,
fv is a Icfile-value+l.
fv is a <file-value+.
Let fi be the <file-information+ designated by iv. fi must contain <stream>.
Let ds be the <dataset+ designated by the fdataset-designator, in fi, and let cp
be the <current-position, in fi.
If si has fpaqemark, or fcarriaqe-return, then fi must contain <print>. Append
si to the <stream-item-list+ in ds. Replace the immediate component of cp by a
<designator, to the last <stream-item+ in ds. If fi contains <print> and si is
a fpaqemark, then add 1 to the <integer-value> in the fpage-numbers in fi.
fv is <absent>.
si must contain a {symbolI+. Let csv be the <character-string-value+ in the
current <string-io-control, and let m be the <integer-value+ of the tstring-
limit, of the current <string-io-control+. It csv contains m {symbol+ or it
m=O then perform raise-condition(<error-condition>).
Let s be the Csymbol) in si. If csv contains fnull-character-strinq, then
replace the immediate component of csv with <character-value-list,: fcharacter-
value,: s. Otherwise, append <character-value,: s; to the fcharacter-value-
list> of csv.
8.7.2.10 Tab
Operation: tab(w, fv)
where w is an <integer-value>,
fv is a <file-value+.
Step 1. Let eto be the <evaluated-tab-option+ in the <file-information+,fi designated by
fv. w must be non-negative. If w=O then terminate this operation.
Step 2. Perform evaluate-current-column0 to obtain an <integer-value>,cc.
Step 3.
Case 3.1. There are at least w <integer-value%, t in et0 satisfying cc < t-l < lsz,
where lsz is the <evaluated-linesize, in ti.
Perform output-tab(fv) w times.
Case 3.2. (Otherwise).
Step 3.2.1. Perform skiptfinteger-value+:l;,fv).
Step 3.2.2. If the first <integer-value* in et0 is not 1 then perform output-
tabffv).
276
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 295

8.7.2.10.1 Output-tab
Operation: output-tabffv)
where fv is a <file-value+.
Step 1. Perform evaluate-current-columnffvl to obtain an <integer-value>,cc.
Step 2. Let tv be the smallest <integer-value+ in the <evaluated-tab-option+ of the
ffile-information> designated by fv, which is greater than cc+l.
Step 3. Let bs be a <stream-item-list*, the ttv-cc-l) <stream-items of which all
contain bs. Perform output-string(bs,fv).
8.7.2.11 Put-line
Operation: put-line(w,fvl
where w is an <integer-value+,
fv is a <file-value+.
step 1. Perform evaluate-current-line(fv) to obtain an *integer-value9,cl. Let ep be
the <evaluated-pagesize, and let cp be the <current-position+ in the tfile-
information+ designated by fv.
Step 2.
Case 2.1. w = cl and the node designated by cp contains a fpaqemark,. a flinemark, or
a fcarriaqe-return,.
Terminate this operation.
Case 2.2. cl < w 5 ep.
Perform skipffinteger-value,:(w-cl);,fvJ.
Case 2.3. ep 2 cl 2 w and Case 2.1 does not apply, or w > ep 2 cl.
Perform output-stream-itemcfstream-item+:<linemark+;,fv) (ep-cl+11 times.
Perform raise-io-condition(<endpage-condition>,fv).
Case 2.4. (Otherwise).
Perform put-page(fv).
8.7.2.12 Put-paqe
Operation: put-paqe(fv)
where fv is a <file-value+.
Step 1. Perform output-stream-itemffstream-item,: fpaqemark+;,fv).
Chapter 8: Input/Output 277
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 296

8.7.3 OPERATIONS APPLICABLE TO STREAM I/O
8.7.3.1 Skip
Operation: skip(w,fv)
where w is an <integer-value+ which must not be negative,
fv is a <file-value%
Step 1. Let fi be the <file-information9 designated by fv.
Step 2.
Case 2.1. The value of w is zero.
Perform output-stream-itemcfstream-item,: fcarriaqe-return*;,fv).
Case 2.2. fi contains <input>.
Perform repeatedly input-stream-item until either w flinemark+s have
been returned or an fomeqa+ is returned. In the latter case perform raise-
io-condition(<endfile-condition>,fv).
Case 2.3. fi contains <output> but not <print>.
Perform output-stream-itemtfstream-item+:4linemark+;,fv) w times.
Case 2.4. fi contains <print>, and w + 0.
Perform evaluate-current-line(fv) to obtain an *integer-value+,cl. Let ep
be the <evaluated-pagesize, in the <file-information+ designated by fv. If
cl > ep or if ep L cl+w then perform output-stream-itemtfstream-item,:
<linemark,; ,f v) w times. Otherwise perform output-stream-item(tstream-
item+:flinemark>;,fv) (ep-cl+11 times and then perform raise-io-
condition(<endpaqe-condition>,fv).
8.7.3.2 Evaluate-current-column
Operation: evaluate-current-column0
where fv is a l file-value,.
result: an <integer-value>.
Step 1. Let fi be the <file-information> designated by fv, and ds the &lataset+
designated by the fdataset-designator+ in fi. Let cp be the node in as
designated by the <current-position+ in fi.
Step 2.
Case 2.1. cp contains an <alpha,, a *linemark+, a fpaqemark,, or a fcarriaqe-return>.
Return finteger-value9: O-
Case 2.2. cp contains a +aymbol* or an <omega*.
Let p be that *alpha+, *linemark>, fpaqemark,, or fcarriaqe-return, in ds
preceding cp and such that any <stream-itees in ds between p and cp contain
{symbolps- Let n be the number of +aymbol)s between p and cp, inclusive.
Return einteger-value>: n.
278
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 297

8.7.3.3 Evaluate-current-line
Operation: evaluate-current-linetfv)
where fv is a <file-value+.
result: an <integer-value>.
Step 1. Let fi be the <file-information+ designated by fv, and ds the *dataset,
designated by the fdataset-designator+ in fi. Let cp be the node in ds
designated by the <current-position+ in fi.
Step 2.
Case 2.1. cp contains an <alpha+ or a fpaqemark,.
Return <integer-value+: 1.
Case 2.2. (Otherwise).
Let p be that *alpha+ or <paqemark+ in ds preceding cp such that all
<stream-item% in ds between p and cp contain $symbol)s, flinemartis, or
fcarriaqe-return>s. Let n be the number of flinemark>s between p and cp,
inclusive. Return <integer-value+: n+l.
8.7.3.4 Establish-next-data-item
Operation: establish-next-data-item
result: a <current-scalar-item> or <none+.
Step 1. In the current *block-control9 let cdld be the last fdata-list-indicator>, and
let cdid be the last <data-item-indicator..
Step 2.
Case 2.1. cdid is tundefined>.
Set cdid to designate the first component of the list designated by cdld.
Case 2.2. cdid designates an immediate component of the list designated by cdld, but
not the last component.
Set cdid to designate the next immediate component of the list designated by
cdld.
Case 2.3. cdid designates the last immediate component of the list designated by cdld.
If cdid designates a <current-scalar-item+ then delete the current fcurrent-
scalar-item-list> and go to Step 4. If the current <data-item-control-list+
contains precisely one <data-item-control+ then return <none,. Otherwise
perform test-termination-of-controlled-group to obtain t. If t is __ <true3
then go to Step 4. Otherwise, let cdid designate the first immediate
component of the list designated by cdld.
Chapter 8: Input/Output 279
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 298

step 3.
Case 3.1. The node designated by cdid immediately contains an <expression>,e.
Perform evaluate-expression(e) to obtain an <aggregate-value+,av, having n
<basic-value9.s. Let csil be a *current-scalar-item-list+ having n elements
whose i'th element is a <current-scalar-item containing the i'th fbasic-
value, in av, bvfil, and the <data-type> corresponding to bviil in the
<data-description> immediately contained in e. Attach csil to the current
<block-control>. Append to the current <block-control+ a
fdata-item-control+:
<data-list-indicator+ designating csil
<data-item-indicator+ designating the first element of csil.
Return the first element of csil.
Case 3.2. The node designated by cdid immediately contains a <target-reference>,tr.
Perform evaluate-target-reference(U) to obtain an <evaluated-target+,et.
Step 3.2.1.
Case 3.2.1.1. et immediately contains a tgeneration9,g.
Perform expand-generation(g) to obtain a <generation-list+,ggl,
having n elements. Let csil be a <current-scalar-item-list>
having n elements whose i'th element is <current-scalar-item+:
<evaluated-target,: gglil;;, where gglil is the i'th element of
ml-
Case 3.2.1.2. et immediately contains an <evaluated-pseudo-variable-reference>,
epvr.
Case 3.2.1.2.1. epvr contains an <onchar-pv>, <onsource-pv>, <paqeno-pv>,
<strinq-pv>, or <unspec-pv>.
Let csil be a <current-scalar-item-list+: fcurrent-scalar-
item,: et.
Case 3.2.1.2.2. epvr contains an <imaq-pv> or <real-pv>.
Let g be the <generation+ in epvr. Perform expand-
generation(g) to obtain a <generation-list>,gg. Let n be
the number of fgeneration%i in gg. Let csil be a fcurrent-
scalar-item-list* containing n copies of fcurrent-scalar-
item,: et. For i=l,..-,n, replace the <generation> in the
i'th <current-scalar-item+ in csil by the i'th *generation>
in gg.
Case 3.2.1.2.3. epvr contains a <substr-pv>.
Let g be the <generation+ in epvr. Perform expand-
generation(g) to obtain a <generation-list+,ggl- Let n be
the number of elements in ggl and let ggfil denote the i'th
element in ggl. Let avl be the first <aggregate-value+ in
epvr and let eavlIi1 be the element of the tbasic-value-
list> in avl which corresponds to ggCi1. If epvr has two
<aggregate-value%, then let av2 be the second and let
eav2Cil be the element in its *basic-value-list+ which
corresponds to gglil. Let csil be a fcurrent-scalar-item-
list> containing n copies of <current-scalar-item,: et. For
i=l s...rn. replace the subtrees of the i'th <evaluated-
pseudo-variable-reference+ in csil as follows:
replace the <generation+ by gglilr
replace the first <aggregate-value+ by an faggregate-
value, containing eavlLi1,
and. if a second <aggregate-value+ is present, replace
it by an <aggregate-value+ containing eav2Ci.l.
280
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 299

Step 3.2.2. Attach csil to the current <block-state+. Append to the fdata-item-
control-list+ in the current <block-control> a
<data-item-control+:
<data-list-indicator+ designating csil
<data-item-indicator+ designating the first element of csil.
Return the first element of csil.
Case 3.3. The node designated by cdid immediately contains a <variable-reference>,vr.
Step 3.3.1. Perform evaluate-variable-referencecvr) to obtain a fgeneration9.g.
Perform make-name-and-subscript-list(vr,g) to obtain a tname-and-
subscript-list+,nasl. Let edd be the *evaluated-data-description%= in g.
Perf arm expand-eddtedd) to obtain an fevaluated-data-description-
list+,eddl having n elements. Let csil be a <current-scalar-item-list,
having n elements whose i'th element contains:
the j'th *basic-value+ in the <allocation-unit+ designated by the
fallocation-unit-designator+ in g, where j is the i'th <storage-index>
in g:
the <data-type> from the i'th <evaluated-data-description> of eddl;
a <data-name-field, containing a $symbol-list),dnfsl obtained from
nas[il, the i’th <name-and-subscript+ of nasl, by performing Step
3.3.1.x.
Step 3.3.1-l. Let dnfsl be the $symbol-list? obtained from the <dot-name-list>
of nas[il by concatenating the {symbol-1ist)s of the <dot-name%
their
Efsl.
natural order. Delete the final $symbol+: $.+ from
If nas[il has a <comma-subscript-list,rcsl, then replace dnfsl by
the {symbol-list) obtained by concatenating dnfsl. {symbol-list):
{symbol#: $();;r and the {symbol-list)s in the <comma-subscriptss
of csl in their natural order, and replacing the final +z.ymbo1p:
f,) by {symbol): $1).
Step 3.3.2. Attach csil to the current <block-control,. Append to the current fdata-
item-control-list> a
<data-item-control+:
<data-list-indicator. designating csil
<data-item-indicator+ designating the first element of csil.
Return the first element of csil.
Case 3.4. The node designated by cdid immediately contains a tree of the form <do-
spec>,dsp.
Let dldl be the first immediate component of the node designated by cdid.
Let did1 be the first immediate component ot dldl. Append to the current
<data-item-control-list9 a
idata-item-control>:
fdata-list-indicator+:
<designator+ designating dldl;
<data-item-indicator>:
<designator+ designating didl.
Let cdid be the current <data-item-indicator>. Perform establish-
controlled-group(dsp1 to obtain t- If t is ftrue, then go to step 3.
Case 3.5. The node designated by cdid is a <current-scalar-item+,csi.
Return csi.
Step 4. If the current fdata-item-control-list+ contains more than one tdata-item-
control, then delete the last of them and go to Step 1. Otherwise return
<none+.
Chapter 8: Input/Output 281
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 300

8.7.3.4-l Expand-edd ,
Operation: expand-edd(edd)
Case 1.
Case 2.
Case 3.
where edd is an <evaluated-data-description,.
result: an *evaluated-data-description-list+.
edd immediately contains <data-description>
<dimensioned-data-description>,idd.
immediately containing a
Let bpl be the <bound-pair-list> in ddd. let lblil and ubiil be respectively the
values of the i'th <lower-bound> and <upper-bound> in bpl and let ne be the
number of elements in bpl. Let n be the integer
ne
n (ubfil-lbIil+l).
i=l
Let eddl be an <evaluated-data-description,: <data-description>: the immediate
subtree of the <element-data-description> in ddd. Perform expand-eddceddl) to
obtain an <evaluated-data-description-list+,eddI. Let eddln be an fevaluated-
data-description-list+ with, in order, n replications of the subtrees of eddl.
Return eddln.
edd immediately contains a <data-description> immediately containing a
<structure-data-description>,sdd.
For each <data-description>,dd[il in sdd perform expand-edd(fevaluated-data-
descriptions: ddfil) to obtain an <evaluated-data-description-list+,eddlkil.
Let eddln be an <evaluated-data-description-list, with, in order, the subtrees
of these eddlfils. Return eddln.
edd immediately contains a <data-description> immediately containing an <item-
data-description>.
Return *evaluated-data-description-list+: edd.
8.7.3.4..2 Expand-qeneration
Operation: expand-qenerationtg)
where g is a <generation,.
result: a fgeneration-list,.
Step 1. Let aud be the <allocation-unit-designator+ in g. Let edd be the fevaluated-
data-description, in g. Let sil be the <storage-index-list+ in g. Let n be the
number of elements in sil. Perform expand-edd(edd) to obtain an fevaluated-
data-description-list,reddl. Let gg be a <generation-list+ without subnodes. For
i=l,.-.,nr let si[il be a copy of the i'th element of sil, let edd[iJ be a copy
of the i'th element of eddl, and attach to gg a <generatio*: eddlil aud
<storage-index-list,: silil.
282
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 301

8.7.3.4.3 Make-name-and-subscript-list
<name-and-subscript+::= [<dot-name-list91 ifcomma-subscript-list91
<dot-name>::= {symbol-list)
<comma-subscript+::= +qmbol-list)
Operation: make-name-and-subscriot-list(vr,g)
where vr is a <variable-reference>,
g is a <generation+.
result: a <name-and-subscript-list+.
Step 1. Let id be the <identifier> in the <declaration> designated by the <declaration-
designator> of vr. Perform identifier-to-dotname to obtain a fdot-name>,dn.
Let dnl be a <dot-name-list+: dn. If vr has an <identifier-list>,idl, then for
each <identifier>,xid in idl, in order, perform identifier-to-dotname(xid) to
obtain a <dot-name+,xdn and append xdn to dnl. If vr has a <subscript-
list>,ssl, then create a <comma-subscript-list,ricsl from ssl by performing Step
1.1.
Step 1.1. Let icsl be a <comma-subscript-list+ without subnodes. Let nss be the
number of immediate subtrees of ssl, and let sslil, i=l ,...,nss, be these
subtrees. For i=l,...,nss, append to icsl a fsymbol-list),sl obtained from
ss[il as follows: if ss[il is a <subscript>: <asterisk>; then sl is +symbol-
list*: {symbol): +*+- Otherwise, let ex be the <expression> in sslil,
perform evaluate-expression-to-integercex) to obtain an finteger-value>,iv,
and perform subscript-to-comma-subscript to obtain a {symbol-list),sl.
Step 2. g contains an <evaluated-data-description9: <data-description>,dd. Let xdd be
the immediate subnode of dd. Perform expand-name-and-subscript(xdd) to obtain a
<name-and-subscript-list+,nasll.
Step 3. Perform Step 3-l for each l name-and-subscript>,xnas, of nasll.
Step 3.1. If xnas contains a <dot-name-list+,xdnl, then replace xdnl by the result of
concatenating a copy of dnl with xdnl. Otherwise attach a copy of dnl to
xnas.
Step 4. If vr has a <subscript-list>,ssl, then perform Step 4.1 for each fname-and-
subscript+,xnas of nasll.
Step 4.1. Let cicsl be a copy of icsl. Let nast be the number of trees Csymbol-list?:
{symbol): {+);; in cicsl.
Case 4.1.1. nast = 0.
If xnas contains a <comma-subscript-list>,xcsl, then replace xcsl by the
result of concatenating ciccl with xcsl; otherwise attach cicsl to xnas.
Case 4.1.2. (Otherwise).
For i=l,...,nast, replace the i'th <comma-subscript+: Csymbol-list?:
* -- in cicsl by a copy of the i'th <comma-subscript+ in
:h:**
xnas. Delete
first nast fcomma-subscript% from the <comma-subscript-list+,xcsl
of xnas- If xcsl is now without subnodes then replace it with cicsl.
Otherwise replace xcsl by the result of concatenating cicsl and xcsl.
Step 5. Return nasll.
Chapter 8: Input/Output 283
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 302

8.7.3.4.4 Expand-name-and-subscript
Operation: expand-name-and-subscripk(cld)
where dd is an <item-data-description>,, or a <dimensioned-data-description>,
or a <structure-data-description>.
result: a <name-and-subscript-list+.
Case 1. ad is an <item-data-description>.
Let xnas be a <name-and-subscript+ without subnodes. Return fname-and-subscript-
list,: xnas.
Case 2,. dd is a <dimensioned-data-description>.
Step 2.1. Let xdd be the <structure-data-description> or <item-data-description> in
the <element-data-description> in dd. Perform expand-name-and-subscript(xdd)
to obtain a <name-and-subscript-list+,nasll.
Step 2.2. For each <bound-pair>,bp in the <bound-pair-list> of dd, taken in order from
left-to-right perform Steps 2.2.1 and 2.2.2.
Step 2.2.1. Let lb and ub be <integer-value% equal to the <integer-value>s in the
<lower-bound> and <upper-bound> of bp, respectively. Let n be ub-lb+l.
Let nas12Cil. i=l,...,n, be n copies of nasll. For each i, i=l,...,n,
let nas13 be nas12[il, let iv be an <integer-value,: lb+i-I:, perform
subscript-to-comma-subscript to obtain a <comma-subscript+,xcs, and
perform Step 2.2.1.1.
Step 2.2.1.1. For each <name-and-subscript,,xnas in nas13 perform Step
2.2.1.1.1.
Step 2.2.1.1.1. If xnas has a <comma-subscript-list,rxcslr then insert xcs
before the first component of xcsl. Otherwise attach to xnas
a <comma-subscript-list,: xcs.
Step 2.2.2. Let nasll be the <name-and-subscript-list+ obtained by concatenating the
n nas12Cil in the order i=l,...,n.
Step 2.3. Return nasll.
Case 3. dd is a <structure-data-description>.
Step 3.1. Let n be the number of <identifier>s in the <identifier-list>,idl in dd.
Let nasllfil, i=l,...,n, be n <name-and-subscript-list% without subnodes.
For i=l,...,n, let nas12 be naslllil, let id1 be the i'th <identifier> in
idl, let ddl be the <data-description> in the i'th <member-description> in
dd, let xdd be the immediate subtree of ddl, and perform step 3.1.1.
Step 3.1.1. Perform identifier-to-dotname(id1) to obtain a fdot-name+,dn. Perform
expand-name-and-subscript(xdd1 to obtain a fname-and-subscript-
list*,nasl2. Perform Step 3.1.1.1 for each fname-and-subscript,,xnas in
nas12.
Step 3.1.1.1. If xnas has a <dot-name-list>,xdnl, then insert dn before the
first subtree of dnl. Otherwise attach to xnas a <dot-name-list+:
an.
Step 3.2. Let nas13 be the <name-and-subscript-list, obtained by concatenating the n
naslllil, in the order i=l,...,n. Return nas13.
284
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 303

8.7.3.4.5 Subscript-to-comma-subscript
Operation: subscript-to-comma-subscript
where ss is an *integer-value,.
result: a *comma-subscript>.
Step 1. If ss=O then let n be 1 and let ssdfll be the {symbol): {O). Otherwise let n be
the number of digits required for the decimal representation of the absolute
value of ss and let ssdlil, i=l,.-.,n, be the +symbol)s representing the digits
of that representation.
Step 2. Let cssl be a {symbol-list) containing, in order, the ssdli3, i=l,....n,
followed by the +symbolt: $,)- If ss<O then replace cssl by the result ot
concatenating a Csymbol-list): {symbol?: T-T;; with cSS~.
Step 3. Return <comma-subscripts: cssl.
8.7.3.4.6 Identifier-to-dotname
Operation: identifier-to-dotname
where id is an <identifier>.
result: a <dot-name,.
Step 1. Let dnsl be a fsymbol-list) obtained by concatenating the $symbol-list? of id
with a {symbol-list?: +symbol): C.).
Step 2. Return <dot-name,: dnsl.
8.7.3.5 Establish-next-format-item
Operation: establish-next-format-item
result: a <format-item>.
Step 1. Let fc be the last *format-control, in the current *format-control-list+. In
fc, let flp be the <format-specification-list-designator>, let fli be the
immediate subnode of the <format-list-index,, and, where they exist, let fiv be
the immediate subnode of the <format-iteration-value*, and let fii be the
immediate subnode of the <format-iteration-index+. Let fsl be the <format-
specification-list> designated by flp, and let n be the number of immediate
components of fsl.
Step 2.
Case 2.1. fli is lass than n.
Add 1 to fli, giving a value, m.
Case 2.1.1. The m'th element of fsl is a <format-iteti,fi.
Let cfi be a copy of fi. Perform evaluate-format-itemtcfi) to obtain a
<format-item>,efi. Return efi.
Chapter 8: Input/Output 285
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 304

Case 2.1.2. The m'th element of fsl is a <format-iteration>,fi.
Let e be the <expression> in the <format-iteration-factor> immediately
contained in fi. Perform evaluate-format-expression(e) to obtain an
<integer-value+,intg. intg must not be negative. If intg > 0 then
append to the current <format-control-list> a
<format-control+:
<format-specification-list-designator> designating the
<format-specification-list> in the m’tb
element of the list designated by flp
<format-list-index+:
<integer-value*:
0;;
<format-iteration-value>:
intq;
<format-iteration-index+:
<integer-value+:
1.
Go to Step 1.
Case 2.2. fli equals n, and fc is the only <format-control, in the current fformat-
control-list+.
Set fli to 0 and go to Step 1.
Case 2.3. fli equals n, and there are at least two fformat-control% in the current
<format-control-list+.
Case 2.3.1. fii is less than fiv.
Add 1 to fii, set fli to 0 and go to Step 1.
Case 2.3.2. fii equals fiv.
Delete fc, and go to Step 1.
8.7.3.6 Evaluate-format-item
When a <format-item> contains a <data-format> or a <control-format>, all <expression>s
contained in it are evaluated, and a tree having the same structure as the <format-item>
(but with <integer-value+s replacing the <expression%) is returned. When a <format-
item> contains a <remote-format>, this <remote-format> specifies a <format-statement>;
designators of the <format-specification-list> of the statement, and of the <format-
statement> itself, are placed in the current *format-control-list,. (The latter
designator allows the correct environment to be established for the evaluation of
<expression>s.) The operation establish-next-format-item is then re-invoked to scan the
<format-specification-list>.
Operation: evaluate-format-item
where fi is a <format-item>.
result: a <format-item>.
Case 1. fi contains a <control-format> or a <data-tormat>.
Let cfi be a copy of fi. For each <expression>,e, in cfi, chosen in any order.
perform evaluate-format-expression(e9 to obtain an <integer-vaiue+,iu and
replace e by iv. Return cfi.
Case 2. fi contains a <remote-format>srf.
Step 2.1. If there is in the current <format-control-list+ a <remote-block-state+.
then let rbs be the last such <remote-block-state+. Attach a copy of rbs to
the current <block-control,.
286
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 305

Case 2.1.1. rf contains a <variable-reference>,vr.
Perform evaluate-variable-referencecvr) to obtain a <generation+ whose
value (see Section 7.1.3) contains a single *format-value>,fv.
Case 2.1.2. rf contains a <named-constant-reference>,ncr.
Perform evaluate-named-constant-referencetncr) to obtain an faggregate-
value, containing a single *format-value*,fv.
Step 2.2. If there exists a <remote-block-state+.rbs in the current <block-control+
then delete rbs. fv contains a fblock-state-designator,,bsd and a <format-
statement-designator>,fsd. The current <format-control-list9 must not
contain a fformat-control, with a <format-statement-designator> equal to
fsd. The current fblock-state-list, must contain a <block-state+ designated
by bsd. If rf immediately contains a <variable-reference> whose <data-type>
has <local> then bsd must designate the current <block-state>.
Step 2.3. Let fsl designate the <format-specification-list> in the node designated by
fsd. Append to the current <format-control-list+ a
<format-control+:
<format-specification-list-designator>: fsl;
<format-list-index+:
l integer-value,:
0;:
fsd
<remote-block-state+:
bsd.
Step 2.4. Perform establish-next-format-item to obtain a <format-iteti,efi. Return
efi.
8.7.3.6.1 Evaluate-format-expression
An <expression> which appears in a <format-specification-list> is evaluated and converted
to an <integer-value+. However, when such an <expression> occurs in a <format-statement>
(and is accessed via a <remote-format>), the necessary environment must be Set Up for its
evaluation. This is achieved by temporarily placing a <remote-block-state> in the
current <block-control+.
Operation: evaluate-format-expression(e)
Case 1.
Case 2.
where e is an <expression>.
result: an <integer-value+.
There exists in the current <format-control-list+ a <remote-block-state>.
Let rbs be the last such <remote-block-state+. Attach a copy of rbs to the
current <block-control+. Perform evaluate-expression-to-integer(e) to obtain an
<integer-value>,intg. Delete the fremote-block-state+ from the current fblock-
control>. Return intg.
(Otherwise).
Perform evaluate-expression-to-integer(e) to obtain an <integer-value+,intg.
Return intg.
Chapter 8: Input/Output 287
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 306

Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 307

Chapter 9: Expressions and Conversion
9.0 Introduction
This chapter defines operations that are involved in the evaluation of expressions.
Included are definitions for builtin functions and the rules for data conversion. The
main Sections are:
9.1 Aggregate Expressions
9.2 Prefix Operators
9.3 Infix Operators
9.4 Builtin-functions
9.5 Conversion
9.1 Aggregate Expressions
9.1.1 SCALAR AND AGGREGATF TYPES
9.1.1.1 Aqqregate Type of a Data Description - --
An *aggregate-type+ is an alternative way of specifying the part of a <data-description>
rJhich describes the structuring of elements into an aggregate. For each <data-
description> there is an associated <aggregate-type, determined as follows:
Ignoring all <identifier-list> immediate components of <structure-data-description>s and
all components of <item-data-description>s, the shapes of the trees for the <data-
description> and the *aggregate-type+ are identical, and categories correspond as
follows:
<data-description>
<dimensioned-data-description>
<element-data-description>
<bound-pair-list> and all its
components
<structure-data-description>
<member-description>
<item-data-description>
<aggregate-type+
<dimensioned-aggregate-type>
<element-aggregate-type+
<bound-pair-list> and equal components if
<integer-value% otherwise <asterisk>
components
<structure-aggregate-type+
<member-aggregate-type+
<scalar+
In many contexts, properties of <data-description% are stated in terms of properties of
their associated iaggregate-type% without explicitly building the corresponding tree or
even using the term “associated”.
For example: "The result has <aggregate-type+: <scalar+;" is equivalent to “the result
<data-description> immediately contains an <item-data-description>".
9.1.1.2 Scalar Elements
The scalar-elements of an faggregate-value, are the components of its <basic-value-list+;
the scalar-elements of a *generation> are the components of its <storage-index-list+.
The number-of-scalar-elements is defined for a <data-description> by the operation
scalar-elements-of-data-description (Section 7.1.11, and applies equally to a <data-
description>, a <generation+ containing tP.at <data-description>, the associated
+aggregate-type3 of that <data-description>, and an <aggregate-value+ containing that
4aggregate-type+.
Chapter 9: Expressions and Conversion 289
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 308

9.1.1.3 Treatment of Scalars
To facilitate the systematic treatment of expression evaluation and assignment,
operations such as evaluate-expression and evaluate-builtin-function-reference are
defined so that they return or accept <aggregate-value?s in all cases. There are, of
course, many contexts in which a value's <aggregate-type+ is known, a priori, to
immediately contain Fscalar+. but these are treated as degenerate cases of the general
mechanism.
9.1.1.4 Compatibility
In general, the <aggregate-value% involved in operations, such as infix-add. are not
required to have equal <aggregate-type+s but only to have <aggregate-type>s that are
compatible. The result of such an operation has an <aggregate-type, that is the common
<aggregate-type+ of the operand <aggregate-type%. These terms are defined as follows:
Let t111 *---, tlnl be a set of <aggregate-type%. If n=l, the set is compatible, and
its common <aggregate-type+ is t113. If n>2, the set is compatible if and only if
t12l,....t[nl are compatible and till is compatible with the common faggregate-
type+,ct, of tt2l,...,tlnl. In this case the common <aggregate-type> of tlll,...,t~nl
equals the common *aggregate-type+ of t[ll and ct. Finally, if n=2. compatibility and
the common <aggregate-type+ are determined as follows:
Case 1. At least one of till, tl21 immediately contains fscalar,.
Assume t111 immediately contains <scalar+. Then the set tc11, t121 is
compatible, and its common <aggregate-type+ is tl23.
Case 2. Both till and tI23 immediately contain <structure-aggregate-type%.
The set till, t121 is compatible if and only if
11) t111 and tl21 have the same number of components, m, in their Unember-
aggregate-type-list%;
(2) for lliim, the i'th member-aggregate-type+ of till is compatible with
the i'th <member-aggregate-type+ of t-121.
When tt11 and t121 are compatible, their common <aggregate-type% immediately
contains a <structure-aggregate-type3 with m &ember-aggregate-type9.55, the i'th
-ember-aggregate-type> having the common <aggregate-type+ of the i'th fmember-
aggregate-type%? of till and tl21.
Case 3. One of t[ll, t121 immediately contains a <dimensioned-aggregate-type+ and the
other does not.
Assume till immediately contains a <dimensioned-aggregate-type+. The set till,
tl21 is compatible if and only if the <element-aggregate-type> of t[ll is
compatible with t121.
In this case the common <aggregate-type% of till and tl21 immediately contains a
<dimensioned-aggregate-type+ with a <bound-pair-list> equal to that of till and
<element-aggregate-type+ containing the same immediate components as the common
<aggregate-type> of tI21 and the <element-aggregate-type+ of till.
Case 4. Both t111 and tC21 have <dimensioned-aggregate-type+s.
The set till and tl21 is compatible if and only if
(1) the <bound-pair-list>s of till and tl21 are equal except that where a
<bound-pair>: <asterisk>: occurs in one, the other may have a <bound-
pair> containing a <lower-bound> and an <upper-bound>;
(21 the <element-aggregate-type?% of till and tl21 are compatible.
When till and tl2J are compatible, their common <aggregate-type>,ct, immediately
contains a <dimensioned-aggregate-type+ whose felement-aggregate-type, contains
290
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 309

the same immediate components as the common <aggregate-type, of the telement-
aggregate-type% of tC11 and t[Zl. The <bound-pair-list> of ct equals that of
tc11 except that, when a <bound-pair> in t[ll has <asterisk>, the corresponding
<bound-pair> of t[21 is used.
9.1.1.5 Correspondence
The following two sections extend the notion of node correspondence (see Section 1.3.1.1)
to apply to scalar-elements of <aggregate-value% and fgeneration>s whose (associated)
faggregate-type% are compatible, and <data-type>s in <data-description>s whose
associated <aggregate-type% are compatible. The extended notion of correspondence is
used extensively in defining aggregate operations and in defining the result <data-
description>s of <expression%.
9-I-1.5.1 Correspondence of Scalar Elements
Let atx and aty be compatible *aggregate-type%, with number-of-scalar-elements nx and
nyr respectively. Let xc11 ,...,x[nxI and yC11 ,...,yCnyl be sequences of scalar-elements.
(For example, xc11 ,....xCnxl might be the scalar-elements of an *aggregate-value+ whose
<aggregate-type+ is atx.) The *aggregate-type% atx and aty determine a correspondence
the xc11 ,...,xCnxl and the y[ll,...,yCnyl as follows: between
Case 1. At least one of atx and aty immediately contains <scalar+.
Case 2.
Case 3.
Case 4.
Assume atx immediately contains <scalar+. Then nx=l, and xc11 corresponds to
all of the yC1l,yC2l,...,yCnyl.
Both atx and aty immediately contain <structure-aggregate-type+s.
Let p be the number of <member-aggregate-type%3 in atx (also atyl, and let mxIi1
(respectively, myIil1, lsilp, be the number-of-scalar-elements of the i'th
member-aggregate-type+ in atx (respectively, atyl. Let sxlil and sy[il be 0,
when i=l. Let sxtil be the sum of mxijl, llj<i, where 25iIp, and let syCi1 be
the sum of myCj1, llj<i. where 21idp. Then the correspondence between the
xCll,.~.,x[nxl and the yIll,...,yCnyl is such that the x[sxiil+jl, lbjQnxti.1,
where l<ilp, correspond to the ytsy[il+kl, llksmy[il, where llilp. (Informally,
the correspondence matches elements of the i'th member of x with those of the
i'th member of y-1 Between these groups the correspondence is determined by the
<aggregate-typess of the i'th *ember-aggregate-type% of atx and aty.
One of atx and aty immediately contains a *dimensioned-aggregate-type+ and the
other does not.
Assume atx has the <dimensioned-aggregate-type+. Let m be the number-of-scalar-
elements of the <element-aggregate-type+ of atx. Then the correspondence
between the xCll,...,xInxl and the y[ll ,...,yCnyl is such that the xim+i+jl,
l<jSm, i=O,l,..., correspond to yCl1, y[2l,...,y[nyl. (Informally, the
correspondence matches the scalar-elements of y with each of the elements of the
array x.1 Between these groups, the correspondence is determined by the
<element-aggregate-type of atx and by aty.
Both atx and aty have *dimensioned-aggregate-typess.
Let mx (respectively, my) be the number-of-scalar-elements of atx (respectively,
atyl. The correspondence between the xCll,...,xCnxl and the y[ll,...,ylnyl is
such that the x[mx*i+jl, II jlmx, correspond to the y[my*i+kl, 1lkSmy. where
2=0,X,... . (Informally, the i*th element of the array x corresponds to the i'th
element of the array y.1 Between these groups, the correspondence is determined
by the <element-aggregate-type+s of atx and aty.
Chapter 9: Expressions and Conversion 291
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 310

9.1.1.5.2 Corresnondence of Data Tvnes
Let x be an <aggregate-value, or a <generation>, and let dd be a <data-description> whose
associated *aggregate-type, equals the <aggregate-type+ of x. Let xC13, xt23,... be the
scalar-elements of x. Then the <data-type>s in dd correspond to the xC13, x121,... as
follows:
Case 1. dd immediately contains an <item-data-description>.
The single <data-type> simply contained in dd corresponds to the single scalar-
element xCl1.
Case 2. dd immediately contains a <structure-data-description>.
Let the <member-description>s simply contained in dd be md1ll,...,md[p3. Let
nCi1, lGAp, be the number-of-scalar-elements of mdCi1. Let sCi1 be 0 when i=l.
Let s[il be the sum of ntjl, lrj<i for Zli<p. The <data-type% in mdlil
correspond to the scalar-elements x[s[il+jl, 11jdnCi1, for lsilp, the
correspondence being determined by the <aggregate-type+ of mdCi.1.
Case 3. dd immediately contains a <dimensioned-data-description>.
Let ed be the <element-data-description> simply contained in dd, and let n be
the number-of-scalar-elements of the <aggregate-type+ of ed. Then for i=O,l,...
the <data-type>s in ed correspond to the scalar-elements xCn*i+jl, lljsn, the
correspondence being determined by the <aggregate-type+ of ed.
Now the correspondence is easily extended to cover cases where the &aggregate-typess of
dd and x are merely compatible. Suppose that all <bound-pair>s in dd contain finteger-
value+s, and let y be any <aggregate-value3 whose <aggregate-type+ is the same as that of
dd. Then a <data-type> in dd corresponds to a scalar-element in x if and only if they
both correspond to some scalar-element in y. If some <bound-pair>s in dd have
<asterisk>s, consider instead a <data-description> that is equal to dd except for <bound-
pair>s and whose associated <aggregate-type+ is the same as that of x.
Finally the correspondence can be established between <data-type% of two <data-
description>s, ddl and dd2, as follows. Let x be an <aggregate-value> whose taggregate-
type+ equals the common faggregate-type, of ddl and dd2 except that all <bound-pair>s
have <lower-bound> equal to 1 and <upper-bound> equal to 1. Then a <data-type> in ddl
corresponds to a <data-type> in dd2 if and only if both correspond to some scalar-element
in x.
292
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 311

9.1.1.6 Generate-aqqreqate-result
Most operations that return an <aggregate-value+ work in the same general way:
The operands are evaluated, bounds of any arrays are checked for compatibility, and
the <aggregate-type+ of the result is determined. Then each scalar-element of the
result is determined by performing a sequence of steps involving only scalar-elements
of the operand values.
The various aggregate operations differ only in the details of this sequence of scalar
steps. To simplify the description of these aggregate operations a special macro
operation, generate-aggregate-result, is used.
Macro Operation: qenerate-aqqreqate-result
A reference to this macro operation always occurs as the first statement in the body ot
an operation, f. The remainder of the body will be a sequence of Steps numbered
1 I-.-, nstep, where nstepzl. To perform f, substitute these Steps into the followinq
macro body immediately after Step 3.2, renumber them and their corresponding Steps and
Cases as Step 3.2.1, Step 3.2.2, . . . . Step 3.2.nstep, and, in Step 3.2. replace "nstep"
by its actual value. Renumber references to these Steps and Cases correspondingly. Then
interpret the expanded macro body as an operation body in the normal way.
Macro Body:
Step 1. For each operand, optj+ll, j=l,...,(number of operands)-1, taken in any order,
perform evaluate-expression(opfj+ll) to obtain an <aggregate-value+,val[jl.
Step 2. If two or more of the valIj1 have <aggregate-type,s containing corresponding
<bound-pair-list>s, then corresponding <bound-pair>s must be equal.
step 3. Let atp be the common <aggregate-type+ of the valljl. Let nse be the number-of-
scalar-elements of atp. For i=l ,...,nse, taken in any order, perform Steps 3.1
and 3.2.
Step 3.1. Let scalar-result-type be that <data-type> of rdd that corresponds to
scalar-element. i, in an <aggregate-value> whose *aggregate-type> is
atp. For each operand, opij+lJ, let the scalar <data-type> of op[j+lJ
be the corresponding <data-type> of opCj+ll, and let the scalar-value of
opCj+ll be the corresponding scalar-element in vallj3.
Step 3.2. Perform Steps 3.2.1 through 3.2.nstep to obtain scalar-result. Let
avalIi1 equal scalar-result.
[Replace this line with the Steps following the macro reference, renumbered as above.1
Step 9. Return an *aggregate-value* whose <aggregate-type, is atp, and whose fbasic-
value-list+ contains avallll, aval[21r...,aval[nse3.
Note: (1) An operation referencing generate-aggregate-result always has a first
operand, raa, which is a <data-description>. Its other operands will be
<expression>s or <argument%. To refer to the scalar-elements of the
operands, the result, and their respective <data-type>s, it uses the four
special terms defined in Step 3. namely: scalar-value, scalar <data-type>,
scalar-result, and scalar-result-type. In the expanded macro body, scalar-
value, scalar-result, and scalar-result-type are local variables, together
with nse, atp, avalCi1, and vallj+ll.
(2) The concepts of scalar-value, scalar <data-type>, scalar-result, and scalar-
result-type are also used in an extended sense in the section on
"Attributes" and "Constraints", where they refer to the parts of a <data-
description> being constructed or checked by the Translator, rather than to
parts of rdd.
Chapter 9: Expressions and Conversion 293
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 312

DECLARE 1 A(101,
2 X FLOAT BINARY,
2 Y CHAR(5).
B(10) FIXED BINARY,
C(9) FLOAT DECIMAL,
1 D. 2 X, 2 Y, 2 2;
A = A(ll+D; A = B+C; A = B+C(l);
Consider the <infix-expression> "A(ll+D" in the first <assignment-statement>. The
<aggregate-type+ of "A(1)" immediately contains a <structure-aggregate-type> whose
imember-aggregate-type-list, has two components, both of which have faggregate-
type,: <scalar+. The <aggregate-type+ of D also immediately contains a fstructure-
aggregate-type+, but its gmember-aggregate-type-list+ has three components. Thus
the two <aggregate-type% are not compatible (Section 9.1.1.4, Case 2). Because the
two <aggregate-type% are not compatible, "A(ll+D" does not satisfy the constraints
for <infix-expression>s. No <data-description> is defined for 'A(l)+D", and the
<program> is rejected by the Translator.
Consider next the <infix-expression> "B+C" in the second <assignment-statement>. B
and C have the same <aggregate-type,,, t. This immediately contains a tdimensioned-
aggregate-type3 whose <bound-pair-list> contains a single <asterisk>, and with
*element-aggregate-type%: <scalar+. Thus B and C have compatible taggregate-
type+s, and the <aggregate-type+ of 'B+C" is their common *aggregate-types, i-e.,
t. Once the <aggregate-type+ of "B+C" is known, determination of the result <data-
description>, d. reduces to the determination of the <data-type> components of d
(i.e., the result-type) from the corresponding components of the <data-
description>s of B and C. In this rule there is only a single result-type. It has
<arithmetic>, with the common derived tiode> of B and C, which has <real>, the
common derived <base> of B and C, which has <binary>, etc.
Evaluation of "B+C" is by the general procedure described in Section 9.1.1.6. First
the expressions "B" and "C" are evaluated. Next the <aggregate-type>s of B and C
are checked for compatibility. Because of the general properties of expressions,
the <aggregate-type%, t(B) and t(C), of the values of B and C will be the same as
the <aggregate-type% of the expressions themselves, except that in t(B) and t(C)
any <bound-pair> will have a pair of finteqer-value,s as its components instead of
an- <asterisk>. Thus it is really only necessary to check that <bound-pair>s match.
In this case. the <bound-oair> in t(B) has comwnents 1 and 10. while that in t (Cl
has components 1 and '9. Thus the <aggregate-type% are not compatible, and the
program fails to satisfy the test in Step 2 of generate-aggregate-result.
Now consider the <assignment-statement> "A=B+C(ll;", which is actually correct!
Evaluation of B yields an <aggregate-value> whose <aggregate-type> is t(B) (see
above), and which has 10 scalar-elements, b[ll I---, b1101. The value of "C(1)" is an
<aggregate-value, with <aggregate-type+: <scalar+; and which has a single scalar-
element, c. The <aggregate-value+ of "B+C(l)" is generated by the general
procedure in Section 9.1.1.6. The i'th scalar-element of the result is generated
by "addition" of bCi1 and c. Here the precise result of the "addition" depends on
the result-type and the <data-type>s of B and C as well as on the value b[il and c.
Evaluation of the <variable-reference> "A" yields a <generation>, ga, whose
<aggregate-type+ imn?ediately contains a <dimensioned-aggregate-type+, t(A),
containing a single <bound-pair> with components 1 and 10. The eelement-aggregate-
type3 of t(A) has a <structure-aggregate-type+ with two member-aggregate-type%,
both with <aggregate-type*: <scalar+. Hence ga has 20 scalar-elements,
a[ll,...,aC201. The *aggregate-type+ of the value of "B+C(l)" is t(B), which is
promotable to the <aggregate-type+, t(A) (Section 7.5.3.1). Therefore the
assignment can be carried out. The assignment involves corresponding scalar-
elements. The first element in the value of B+C(l) is assigned to the components
of storage designated by a[11 and a121. The second element is assigned to at31 and
a141, etc. The details of the assignment depend on the corresponding <data-type>s
in the <data-description> of ga. Assignment of aC11, a[31,... is controlled by the
<data-type> of A-X, which has <float> and <binary>; assignment to af21, aC41,... is
controlled by the <data-type> of A.Y. which has <character>.
Example 9.1. An Example of Scalar and Aggregate Types-
294
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 313

9.1.2 INTEGER TYPE
The term integer-type means a <data-type> which has <arithmetic> containing <real>
<fixed> <binary> <number-of-digits>: n; and <scale-factor>: 0. n is
defined and depends on the particular context in which the term is used.
implementation-
9.1.2.1 Evaluate-expression-to-inteqer
Operation: evaluate-expression-to-inteqerte)
where e is an <expression>.
result: an <integer-value+.
Step 1. Perform evaluate-expression(e) to obtain an *aggregate-value,,av. Let bv be the
*basic-value, in av. Let sdt be the <data-type> of e.
Step 2. Let tdt be a <data-type> which is integer-type (Section 9.1.2). Perform
convert(tdt.sdt, bv) to obtain a freal-value+.rv.
Step 3. Return an <integer-value% with the same component as rv.
9.1.3 DERIVED DATA TYPES
In general, a scalar <data-type> of an expression or a target <data-type> for a
conversion operation is derived from other <data-type>s. Special terminology is
introduced for the most common cases.
9.1.3.1 Derived Base. Scale, and Mode
Let S be a set of one or more <data-type>s which have <computational-type>.
The derived <base> of S has <binary> if any element of S has <arithmetic> with <base>:
<binary>; or if any element of S has <string> with <string-type>: <bit>. Otherwise the
derived <base> of S has <decimal>.
The derived <scale> of S has <float> if any element of S has <arithmetic> (including
<arithmetic> in <pictured-numeric>) with <scale>: <float>. Otherwise the derived <scale>
has <fixed>,.
The derived <mode> of S has <complex> if any element of S has <arithmetic> (including
<arithmetic> in <pictured-numeric>) with <mode>: <complex>. Otherwise the derived <mode>
has <&>.
When the set S contains at least two <data-type%, the terminology derived common <base>,
derived common <scale>, and derived common <mode> will generally be used.
Chapter 9: Expressions and Conversion 295
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 314

..--1 --.“-- -~_/--~.‘--L __._
9.1.3.2 Converted Precision
Given a source <data-type> which has <computational-type>, a target <base>, and a target
<scale>, the source <data-type> has associated with it a converted <precision>, with a
converted <number-of-digits> and a converted <scale-factor>,. For a source <data-type>
which has <arithmetic> (including <arithmetic> in <pictured-numeric>), the converted
<precision>, converted <number-of-digits> and converted <scale-factor> are defined in
Table 9.1.
For a source <data-type> which has <string> with <string-type>: <bit>; the converted
<precision> is the same as for a source <data-type> which has <arithmetic> with <base>:
<binary>;. <scale>: <fixed>;, <scale-factor>: 0; and <number-of-digits>: N;, where N is
the maximum <number-of-digits> allowed for <binary> and <fixed>.
For a source <data-type> which has <string> with <string-type>: <character>;, or has
<pictured-character>, the converted <precision> is the same as for a source <data-type>
which has <arithmetic> with <base>: <decimal>;, <scale>: <fixed>;, <scale-factor>: 0; and
<number-of-digits>: N;, where N is the maximum <number-of-digits> allowed for <decimal>
and <fixed>.
Table 9.1. Table of Converted Precisions as a Function of Target and Source Attributes.
r---------------------------------------------------------------------------------------,
1 Target 1
1 <base> 1 Source <base> and <scale>
I and I------------------------------------------------------------------------------,
1 <scale>1 Binary Fixed I Decimal Fixed I Binary Float I Decimal Float I I--------+------------ --------t---------------------+------------------+---------------,
I
i B'
/ P'=P
I I p'=min(ceil I
I (p+3.32) +l,N) I I lnary 1
I Fixed I q'=q / q’*= ceil(q*3.32) t
I
I
I I I I ,--------+---------------------+---------------------+------------------+---------------~
I mintceil I P'"P
(p/3.32) +l,N)
i i q'=q
I-
[ q'=ceil(q/3.32)
I -------+---------------------+--------------------- +------------------+---------------I
; B'
I I I lnary I p'=min(p,N) I p'=min(ceil I P'=P I p'=min(ceil
I Float I I (p+3.32) ,N) (p*3.32),N) I
I I --------+---------------------+---------------------+------------------+---------------
i Decimal; p'=min(ceil
(p/3.32) ,N)
1 the <scale-factor>, q', of the converted <preciHion> as functions of the <number- I
I of-digits>, p, and <scale-factor>, q, of the source <precision>. Those table entries 1
I left blank are for cases which never arise in the language definition. N is the I
I maximum <number-of-digits> allowed for the target <base> and <scale>.
I
In many cases. evaluation of an expression requires conversion of the values of its
subexpressions to target <data-type% which have <arithmetic> and whose <base>, <scale>,
and <mode> are all the same, but whose <precision>s are determined individually using the
rules for converted <precision>.
296
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 315

Consider the <infix-expression> "l.OEO + '1O'B". The rules for evaluating <infix-
expression>s specify that, before the addition is performed, the values of the two
expressions l.OEO and l lO*B are to be converted to the derived common <base>,
<scale>, and -ode> of the expressions. According to Section 9.1.3.1, this means
conversion to a target <data-type> that has <binary>, <float>, and <m>. The
<precision> of each target <data-type> is the converted <precision>.
For the <constant> 'l.OEO", the source <base> has <decimal>, and th3 source
<number-of-digits> is 2. Therefore, according to Table 9.1, the converted <-rumber-
of-digits> is 7. For the <bit> <constant> *lO'B, the source <base> has (binary>,
the source <scale> has <fixed>,and the value p in Table 9.1 is taken to be the
maximum <number-of-digits>, Nl, allowed for <fixed> and <binary>- Therefore the
converted <number-of-digits> is min(Nl,NZ), where N2 is the maximum <number-of-
digits> allowed for <float> and <binary>.
Example 9.2. An Example of Converted Precision.
9.1.3.3 Derived String Type
Let S be a set of one or more <data-type>s which have <computational-type>.
The derived <string-type> of S has <bit> if all the types in S have <string> with
<string-type>: <bit>. Otherwise the derivedstring-type> has <character>.
When the set S contains at least two <data-type>s, the terminology derived common
<string-type> will generally be used.
9.L3.0 Further Definitions for Character and Bit Strinqs
In the definitions of operations whose result <data-type> is either <character> or <m>,
the
(1)
(2)
(3)
(4)
following definitions are used:
The lenqth of a <character-string-value+ or of a <bit-string-value+ is zero if it
contains a <null-character-strinqs or a inull-bit-strinq>; otherwise it is equal to
the number of components in its <character-value-list+ or <bit-value-listi.
A null-strinq is a *character-string-value+ containing *null-character-strinq> when
the result <string-type> has <character> or a fbit-string-value+ containing a *null-
bit-strinq+ when the result <string-type> has <bit>-
A strinq is a <character-string-value, when the result <string-type> has
<character>, or a fbit-string-value+ when the result <string-type> has <m>.
A substrinq, c, of a string, St, is a string of the same type as st. It either
contains a null-string, or it contains a contiguous set of icharacter-value>s or
<bit-value,s from st in the same order as in st.
Chapter 9: Expre&ions and Conversion 297
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 316

9.1.4 ARITHMETIC RESULTS
In the definitions of the evaluation for many <infix-expression% and <builtin-function-
reference% the final step in determining a scalar-result is to take an arithmetic value
computed in some natural way (e.g. by addition of two numbers) and adjust it in a way
that represents the special properties of arithmetic in PL/I. This adjustment is made by
the operation arithmetic-result, which is defined below. This operation may cause
certain conditions to be raised. Note, however, that there are other points in a
<program> execution where the same conditions can be raised.
operation: arithmetic-result(v,rt)
where v is a freal-value> or <complex-value,,
rt is a <data-type> that has <arithmetic>.
result: a *real-value, or a <complex-value, (as v).
Let b be the numerical-base of rt, m be the <number-of-digits> of rt, n be the <scale-
factor> of rt (if rt has <fixed>), and N be the maximum <number-of-digits> allowed for
the <base> and <scale> of rt. If v is a <complex-value+, let x and y be its real and
imaginary parts, respectively.
Case 1. rt has <fixed> and <real>.
Determine a freal-values,v' as follows:
v' = (bt-nl*floor(v+btn), if v L 0,
V' = (bt-n)*ceil(v+btnl, if v <O.
If abs(v.1 2 bt (N-n), then optionally perform raise-
conditionl<fixedoverflow-condition>); otherwise, return v'.
Case 2. rt has <fixed> and <complex>.
Determine values x' and y' as follows:
x' = (bt-n)*floor(lbtn)+x>, if x L 0,
X' = (bt-n)*ceil((btn)+x), if x < 0,
y' = (b+-n)+floor((btn)*y)r if y 2 0,
y' = (bt-n)+ceil((btn)*y), if y < 0.
If abs(x')>bt(N-n), or if abs(y’)Lbt (N-n), then optionally perform raise-
condition(<fixedoverflow-condition>); otherwise, return a <complex-value+:
x’+i.*y’.
Case 3. rt has <float> and <real>.
Step 3.1. If this operation was invoked by infix-add, infix-subtract, infix-multiply
or infix-divide then perform Step 3.1.1.
Step 3.1-l. Let opl and op2 be the freal-value% denoted by x' and y' in Step 1 of
the operation that invoked this operation. If op1, op2 and v are all
integers, and if abs(opl), abs(op2) and abs(v) are less than btm, then
return v.
Step 3.2. Optionally perform raise-condition(<underflow-condition>) and return a
*real-value>: 0.
Step 3.3. Optionally perform raise-condition(<overflow-condition>).
Step 3.4. Return an implementation-dependent approximation to v.
Case 4. rt has <float> and <complex>.
Step 4.1. Optionally perform raise-condition(<underflow-condition>), let x' = 0, and
go to Step 4.4.
Step 4.2. Optionally perform raise-condition(<overflow-condition>).
Step 4.3. Let X* be an implementation-dependent approximation to x.
298
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 317

Step 4.4. Optionally perform raise-condition(<underflow-condition>), let y' = 0, and
go to step a-7.
Step 4.5. Optionally perform raise-condition(<overflow-condition>).
Step 4.6. Let y' be an implementation-dependent approximation to y.
Step 4.7. Return a <complex-values: x'+i*y*.
9.1.4-l Conditions in Expressions
Certain properties are common to a number of <expression% that yield arithmetic results.
If a condition may occur at the point where a result is normally obtained, then the
operation arithmetic-result is invoked,. If a condition may occur during the course of a
more complicated evaluation, then the operation conditions-in-arithmetic-expression is
invoked.
Operation: conditions-in-arithmetic-expression(v)
where v is a <data-type>.
Case 1. v contains <float>.
Step 1.1. Optionally perform raise-condition(<underflow-condition>).
Step 1.2. Optionally perform raise-condition(<overtlow-condition>).
Case 2. v contains <fixed>.
Optionally perform raise-condition(<fixedoverflow-condition>).
9.1.5 EXPRESSIONS
Operation: evaluate-expression(e)
where e is an <expression> or <argument>.
result: an <aggregate-values.
Step 1. If e is an <argument>. let x be the first immediate component of the
<expression> immediately contained in e; otherwise, let x be the first immediate
component of e. Let f be the type of x (e.g. prefix-expression). Perform
evaluate-f(x) to obtain v. Return v. (See Section 9.1.8 for the case where x is
<isub>.)
9.1.6 VALUE REFERENCES
Operation: evaluate-value-referencetvr)
where vr is a <value-reference>.
result: an <aggregate-value+.
Case 1. vr immediately contains a <builtin-function-reference>,x.
Perform evaluate-builtin-function-reference(x) to obtain v. Return v.
Case 2. vr immediately contains a <named-constant-reference>,x.
Perform evaluate-named-constant-reference(x) to obtain v. Return v.
Chapter 9: Expressions and Conversion 299
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 318

.-
case 3. vr immediately contains a <variable-reference>,x.
Perform evaluate-variable-reference(x) to obtain a tgeneration,,g. Perform
value-of-generation(g) to obtain an <aggregate-value,,v. v must not contain
fundefined,. Return v.
Case 4. vr immediately contains a <procedure-function-reference>,x.
Step 4.1. Perform evaluate-entry-reference(x) to obtain an fevaluated-entry-
reference,,eer.
Step 4.2. Perform activate-procedure(eer).
Step 4.3. Let v be a copy of the <aggregate-value, in the freturned-value, of the
current <block-state,. Return v.
9.1.7 CONSTANTS
Operation: evaluate-constant(c)
where c is a <constant>.
result: an <aggregate-value+.
Step 1. Return an faggregate-values containing the <basic-value+ in c.
9.1.8 ISUES
Note: There is no operation evaluate-isub. Before an <expression> containing an
<isub> is evaluated, the <isub> is replaced by an <expression> without <isub>s.
See Section 7.6.12.
9.1.9 PARENTHESIZED EXPRESSIONS
Operation: evaluate-parenthesized-expression(e)
where e is a <parenthesized-expression>.
result: an <aggregate-value+.
Step 1. Let x be the <expression> immediately contained in e. Perform evaluate-
expression(x) to obtain v. Return v.
9.1.10 ARGUMENTS
Note: There is no operation evaluate-argument. See evaluate-expression in Section
9.1.5.
300
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 319

9.2 Prefix Operators
This section describes the prefix operators available in PL/I. Section 9.2.1 gives the
general rules for evaluating a <prefix-expression>, and Section 9.2.2 presents the
details for each <prefix-operator> in alphabetical order.
9.2.1 PREFIX EXPRESSIONS
Operation: evaluate-prefix-expression(e)
where e is a <prefix-expression>.
result: an *aggregate-value+.
Step 1. Let x be the <expression> immediately contained in e. Let rdd be the <data-
description> immediately contained in e. Perform prefix-f(rdd,x) to obtain v,
where f is the name of the <prefix-operator> immediately contained in e. Return
V.
9.2.2 DEFINITION OF THE PREFIX OPERATORS
The descriptions of the <prefix-operator>s are given in the following sections in
alphabetical order.
Under Constraints and Attributes, x denotes the <expression> immediately contained in the
<prefix-expression>. Note that all the operations use the macro operation generate-
aggregate-result.
9.2!.2.1 Prefix-minus
Constraints: Each scalar <data-type> of x must have <computational-type>.
Attributes: Each scalar-result-type has <arithmetic> with the derived <base>, <scale>,
and <mode> and the converted <precision> of the corresponding scalar <data-
type> of x.
The result <aggregate-type+ is the same as the <aggregate-type, of x.
Operation: prefix-minus(rdd,x)
Perform generate-aggregate-result.
Step 1. Convert the scalar-value of x to the scalar-result-type to obtain y. The
scalar-result is -y.
Chapter 9: Expressions and Conversion 301
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 320

9.2.2.2 Prefix-not
Constraints: Each scalar <data-type> of x must have <computational-type>.
Attributes: Each scalar-result-type has ab>.
The result <aggregate-type+ is the same as the <aggregate-type> of x.
Operation: prefix-not(rdd,x)
Step 1.
Step 2.
Perform generate-aggregate-result.
Convert the scalar-value of x to <bit> to obtain y.
If Y contains the <null-bit-strinq,, then the scalar-result is fbit-string-
value,: <null-bit-strinq3. Otherwise the scalar-result is a <bit-string-value+
whose length is the same as the length of y- The i'th <bit-value> of the result
has *zero-bit+ if the i'th *bit-value, of y has *one-bit>; it has *one-bit3 if
the i'th <bit-value+ of y has <zero-bit*.
9.2,.2.3 Prefix-plus
Constraints: Each scalar <data-type> of x must have <computational-type>.
Attributes: Each scalar-result-type has <arithmetic> with the derived <base>, <scale>,
and <mode> and the converted <precision> of the corresponding scalar <data-
type> of x.
The result <aggregate-type+ is the same as the <aggregate-type> of x.
Operation: prefix-plus(rdd,x)
Perform generate-aggregate-result.
Step 1. Convert the scalar-value of x to the scalar-result-type to obtain the scalar-
result.
302
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 321

9.3 Infix Operators
This Section describes the <infix-operator>s available in PL/I. Section 9.3.1 gives the
general rules for evaluating an <infix-expression>, and Section 9.3.2 presents the
details for each <infix-operator> in alphabetical order.
9.3.1 INFIX EXPRESSIONS
Operation: evaluate-infix-expression(e)
where e is an <infix-expression>.
result: an <aggregate-value*.
Step 1. Let x and y be, in order, the <expression>s immediately contained in e. Let rdd
be the <data-description> immediately contained in e. Perform infix-f(rdd,x,yl
to obtain v, where f is the name of the <infix-operator> immediately contained
in e. Return v.
9.3.2 DEFINITION OF THE INFIX OPERATORS
The descriptions of the <infix-operator% are given in the following sections in
alphabetical order.
Under Constraints and Attributes, x and y denote, in order, the <expression>s immediately
contained in the <infix-expression>. Further, for the <infix-operator% <add>,
<subtract>, <multiply>, <divide>. and <power>, certain local variables are used with
special meanings as follows. The local variables m, p, and r denote the <number-of-
digits> components of, respectively, the scalar-result-type, the converted <precision> of
the corresponding <data-type> of x, and the converted <precision> of the corresponding
scalar <data-type> of y. The local variables n, g, and s denote the <scale-factor>
components of, respectively, the scalar-result-type,, the converted <precision> of the
corresponding scalar <data-type> of x, and the converted <precision> of the corresponding
scalar <data-type> of y. Unless otherwise stated, the target <base> and <scale> for
determining the converted <precision> are the <base> and <scale> of the scalar-result-
%=- The letter N denotes the maximum <number-of-digits> allowed for data of the <base>
and <scale> of the scalar-result-type. The local variable b denotes the numerical-base
of the scalar-result-type.
Note that all operations use the macro operation generate-aggregate-result (Section
9.1.1.6).
Chapter 9: Expressions and Conversion 303
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 322

9.3.2.1 Infix-add
Constraints: All scalar <data-type>s of x and y must have <computational-type>.
The <aggregate-type% of x and y must be compatible.
Attributes: Each scalar-result-type has <arithmetic> with the derived common <base>,
<scale>, and <mode> of the corresponding scalar <data-type>s of x and y. If
the result <scale> has <float>, then m = max(p,r). If the result <scale> has
<fixed>. then
m = min(N,max(p-q,r-s)+max(qrs)+l)
n = max(q,sl.
The result <aggregate-type> is the common *aggregate-type% of x and y.
Operation: infix-add(rdd,x,y)
Perform generate-aggregate-result.
step 1. Convert the scalar-values of x and y to target scalar <data-type>s which have
<arithmetic> with the derived common <base>, <scale>, and <mode> of the scalar
<data-type>s of x and y. The target <precision> for the conversion of the
scalar-value of x (respectively, y) is the converted <precision> of the scalar
<data-type> of x (respectively, y). Let x' and y' be the converted values.
step 2. Perform arithmetic-result(x'+y',scalar-result-type), to obtain the scalar-
result.
9.3.2.2 Infix-and
Constraints: All scalar <data-type% of x and y must have <computational-type>.
The <aggregate-type+s of x and y must be compatible.
Attributes: Each scalar-result-type has <bit>.
The result <aggregate-type+ is the common <aggregate-type> of x and y-
Operation: infix-and(rdd,x,yl
Perform generate-aggregate-result.
step 1. Convert the scalar-values of x and y to <bit>. Let n be the maximum of the
lengths of the converted values.
Step 2. If the length of one converted value is less than n, convert it to <bit> of
specified length n. Let x' and y' be the final converted values.
Step 3. If n = 0, the scalar-result is a <bit-string-value,: <null-bit-strinqs.
Otherwise the scalar-result is a <bit-string-value, of length n. The i'th fbit-
value, of the result is *one-bit+ if the i'th <bit-value% of both x' and y' are
tone-bit+; otherwise the i'th <bit-value+ of the result is <zero-bit,.
304
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 323

9.3.2.3 Infix-cat
Constraints: All scalar <data-type>s of x and y must have <computational-type>.
The faggregate-type% of x and y must be compatible.
Attributes: Each scalar-result-type has the derived common <string-type> of the
corresponding scalar <data-type>s of x and y.
The result <aggregate-type+ is the common <aggregate-type> of x and y.
Operation: infix-cat(rdd,x,y)
Perform generate-aggregate-result.
Step 1. Convert the scalar-values of x and y to the derived common <string-type> of the
scalar <data-type% of x and y. Let the converted values be x' and y'.
Step 2. Perform concatenate(x',y') to obtain the scalar-result.
9.3.2.3.1 Concatenation of Strinq Values
Operation: concatenate(sl,s2)
where sl and s2 have the same <string-type>.
result: a string of the same <string-type> as sl and s2.
Case 1. sl and s2 are both null-strings.
Return a null-string.
Case 2. (Otherwise).
Return a string containing the <character-value% or <bit-value% of sl, if any,
in order, followed by the <character-value% or <bit-value% of ~2, if any, in
order.
9.3.2.4 Infix-divide
Constraints: All scalar <data-type>s of x and y must have <computational-type>.
The <aggregate-typess of x and y must be compatible.
Attributes: Each scalar-result-type has <arithmetic> with the derived common <base>,
<scale>, and <mode> of the corresponding scalar <data-type>s of x and y. If
the result <scale> has <float>, then m = max(p,r). If the result <scale> has
<fixed>. then:
m = N,
n = N-p+q-s.
The result l aggregate-type, is the common <aggregate-type, of x and y.
Operation: infix-divide(rdd,x,y)
Perform generate-aggregate-result.
Step 1. Convert the scalar-values of x and y to target scalar <data-type>s which have
<arithmetic> with the derived common <base>, <scale>, and -ode> of the scalar
<data-type>s of x and y. The target <precision> for the conversion of the
scalar-value of x (respectively, y) is the converted <precision> of the scalar
<data-type> of x (respectively, y). Let x' and y' be the converted values.
step 2. If y' = 0, perform raise-condition(<zerodivide-condition>); otherwise perform
arithmetic-result(x'/y*,scalar-result-type), to obtain the scalar-result.
Chapter 9: Expressions and conversion 305
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 324

9.3.2.5 Infix-eq
Constraints: Corresponding scalar <data-type>s of x and y must:
(11 both have <computational-type>, or
(2) both have <locator>, or
(3) have <non-computational-type>, with the immediate subnodes of the
<non-computational-type>s belonging to the same category other
than <locator> or <area>.
Further, if one scalar <data-type> has <offset> and the other has <pointer>,
then the <offset> must contain a <variable-reference>.
The <aggregate-type% of x and y must be compatible.
Attributes: Each scalar-result-type has <bit>.
The result <aggregate-type+ is the common <aggregate-type+ of x and y.
Operation: infix-eq(rdd,x,y)
Perform generate-aggregate-result.
Step 1. Perform compare(x',y',t[ll,t[23) to obtain camp, where x' is the scalar-value of
XI y' is the scalar-value of y. tC13 is the scalar <data-type> of x, and t(21 is
the scalar <data-type> of y.
Step 2. If camp is <equal+, the scalar-result is a fbit-string-value> containing <one-
&,. Otherwise the scalar-result is a <bit-string-value> containing fzero-
&&,.
9.3.2.5.1 Compare
<comparison-result+::= <equal+ 1 *not-equal+
<not-equal,::= [<less-than, 1 <greater-than>]
Operation: compare(v[ll,v~2l,t[ll,tC2l)
where vC11 is a <basic-value+.
~(21 is a <basic-value3,
t[ll is a scalar <data-type>,
t[21 is a scalar <data-type>.
result: a subtree of <comparison-result+.
Case 1. At least one of t[ll and t(21 has <arithmetic> or <pictured-numeric>.
Convert ~(11 and v[21 to target scalar <data-type>s which have <arithmetic> with
the derived common <base>, <scale>, and <mode> of till and tI21. The <precision>
of the target type for v[il is the converted <precision> of t[il. Let ~(11' and
v(21' be the converted values.
Case 1.1. ~(11' = ~(21'.
Return <equal+.
Case 1.2. The derived common <mode> is <complex>, and vCl1' + ~(21'.
Return <not-equal> without any subnode.
Case 1.3. The derived common <mode> is <real>, and vCl1' < vi21'.
Return <not-equal>: <less-than,.
306
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 325

Case 1.4. The derived common <mode> is <a>, and ~111' > ~121'.
Return *not-equal%: fqreater-than,.
Case 2. Each of t[ll and tf21 has <string> or <pictured-character>.
Convert ~111 and ~121 to the derived common <string-type> of tJ11 and t[Zl. Let
n be the maximum of the lengths of the converted values. If the length of one
converted value is less than n, convert it to the derived common <string-type>
with specified length n. Let vJ11' and ~121' be the final converted values.
Case 2.1. vtll' = vC21*-
Return <equal>.
Case 2.2. vtll' and vJ21' are <bit-string-value%, and i is the smallest integer suc'l
that the i'th <bit-value% of v[ll' and vJ2J' differ.
If the i'th <bit-value+ of v[lJ' is *zero-bit, (while the i'th *bit-value>
of v[21' is <one-bit+), return <not-equal+: fless-thar9. Otherwise return
<not-equal,: fqreater-than,.
Case 2.3. v[ll* and vC21' are <character-string-valueH, and i is the smallest integer
such that the i'th <character-value, of v[lJ' and vJ21' differ.
If the $symbol) in the i'th <character-value> of vtll' precedes the +X.ymbol)
in the i'th <character-value> of vJ2J' in the result of performing collate-
bif, return <not-equal,: <less-than+. Otherwise return <not-equal>:
fqreater-than+.
Case 3. t[ll and tC21 both have <non-computational-type>-
If t[iJ has <pointer> and t[jl has <offset>, perform convert(tlil,tljl,v[j3) to
obtain x, and let vCjl* be a <basic-value9: x, and let vlil' = vfil. Otherwise
let vCl1' = ~111, and vC21' = vC21.
Case 3.1. vtll' and vC21' do not contain <pointer-value% or foftset-value%.
If vCll* and vl21' are identical, return <equal+; otherwise return fnot-
equal, with no subnode.
Case 3.2. vfll' and vC21' contain <pointer-value%.
Step 3.2.X. Any <allocation-unit-designator, contained in v[ll' or vf21' must
designate an <allocation-unit+ that exists.
Step 3.2.2. Let eddl and edd2 be the <evaluated-data-description+ of v[ll' and ~~21'
respectively.
Step 3.2.3.
Case 3.2.3.1. VCll' and ~121' are equal.
Return <equal+.
Case 3.2.3.2. Either vCll* or v[21', but not both, has 4-9.
Return fnot-equals.
Case 3.2.3.3. The tallocation-unit-designator% of v111' and vf21' are
different and eddl and edd2 both contain a <data-type> that has
neither a <maximum-length> with 0 nor an <area-size> with 0.
Return tnot-equal+.
Chapter 9: Expressions and Conversion 307
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 326

Case 3.2.3.4. (Otherwise).
The <allocation-unit-designator9s of vill' and vI23' must be
equal. The first elements, sill11 and si2[11, respectively, of
the <storage-index-list% of vlll' and vI21' must be different.
Suppose silt11 < si2Cll. Let k be such that silCk1 = si2!11, if
such a value exists; otherwise let k be m+l, where m is the
number of elements in sil. There must exist an n, lln<k, such
that the <item-data-description>,idd obtained by performing find-
item-data-description(dd,nJ, where dd is the <data-description>
of eddl, has a <data-type> that contains neither a Qnaximum-
length> with 0 nor an <area-size> with 0.
Return <not-equal>.
Case 3.3. vIl1' and vC21* contain <offset-value%.
Case 3.3.1.
Case 3.3.2.
Case 3.3.3.
v[ll' and vC21' are equal, or differ only in their <occupancy+
components.
Return <equal+.
VCll' or vC21', but not both, has t-3.
Return <not-equal>.
(Otherwise).
The *significant-allocation-IistW in v[ll' and v[21' each have nl and
n2 components, respectively, and nl must not equal n2 (say nl < n21.
Let eddlCi1, i=l,...,nl, and edd2[il, i=l ,...,n2, be respectively the
<evaluated-data-description% in the isignificant-allocation-list+s of
VIll' and vI21'. eddlCi1 must equal edd2Iil for i=l,...,nl. and for
some j, nl<j<n2, edd2Cjl must have a <data-type> which does not contain
a <maximum-length> with 0 nor an <area-size> with 0.
Return <not-equal>.
3.3.2.6 Infix-qe
Constraints: AL1 scalar <data-type% of x and y must have <computational-type>. Further,
each such <data-type> must not have <arithmetic> or <pictured-numeric> with
<mode>: <complex>.
The <aggregate-type+s of x and y must be compatible.
Attributes: Each scalar-result-type has <bit>.
The result <aggregate-type> is the common <aggregate-type+ of x and y.
Operation: infix-qe(rdd,x,yl
Perform generate-aggregate-result.
Step 1. Perform compare(x',y',t~ll,t[23) to obtain camp, where x' is the scalar-value Of
XI y' is the scalar-value of y. tC11 is the scalar <data-type> of x, and t[21 is
the scalar <data-type> of y-
Step 2. If camp is <not-equal,: fqreater-than,; or <equal,, the scalar-result is a fbit-
string-value, containing <one-bit+; otherwise the scalar-result is a fbit-
string-value, containing <zero-bit,.
308
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 327

9.3.2.7 Infix-qt
Constraints: All scalar <data-type% of x and y must have <computational-type>. Further,
each such <data-type> must not have <arithmetic> or <pictured-numeric> with
<mode>: <complex>.
The <aggregate-typeka of x and y must be compatible.
Attributes: Each scalar-result-type has <@>.
The result *aggregate-type+ is the common <aggregate-type+ of x and y-
Operation: infix-qt(rdd,x,y)
Perform generate-aggregate-result.
Step 1. Perform compare(x*,y'.tCll,t[23) to obtain camp, where x' is the scalar-value of
X* y' is the scalar-value of y, till is the scalar <data-type> of x, and ti21 is
the scalar <data-type> of y.
Step 2. If camp is <not-equal+: <greater-than+; then the scalar-result is a -it-string-
value, containing tone-bit+; otherwise the scalar-result is a tbit-string-value+
containing <zero-bit%.
9.3.2.8 Infix-le
Constraints: All scalar <data-type>s of x and y must have <computational-type>. Further,
each such <data-type> must not have <arithmetic> or <pictured-numeric> with
<mode>: <complexJT
The <aggregate-type+s of x and y must be compatible.
Attributes: Each scalar-result-type has <bit>.
The result <aggregate-type+ is the common <aggregate-type>
Operation: infix-le(rdd.x.yl
Perform generate-aggregate-result.
of x and y.
Step 1. Perform compare(x',y*,t[ll,tC21) to obtain camp, where x' is the scalar-value of
x, y' is the scalar-value of y, t[ll is the scalar <data-type> of x, and t[21 is
the scalar <data-type> of y.
Step 2. If camp is <not-equal*: <less-than+; or <equal> then the scalar-result is a
&it-string-value% containing <one-bit+; otherwise the scalar-result is a fbit-
string-value+ containing <zero-bit,.
Chapter 9: Expressions and Conversion 309
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 328

9.3.2.9 Infix-lt
Constraints: All scalar <data-type>s of x and y must have <computational-type>. Further,
each such <data-type> must not have <arithmetic> or <pictured-numeric> with
<mode> : <complex>.
The <aggregate-type+s of x and y must be compatible.
Attributes: Each scalar-result-type has <bit>.
The result <aggregate-type+ is the common <aggregate-type+ of x and y-
Operation: infix-lt(rdd,x,y)
Perform generate-aggregate-result.
Step 1. Perform compare(x',y',t[ll,t121) to obtain camp, where x' is the scalar-value of
x, y' is the scalar-value of y, tCl1 is the scalar <data-type> of x, and t[21 is
the scalar <data-type> of y.
Step 2. If camp is <not-equal+: <less-than+; then the scalar-result is a fbit-string-
value> containing <one-bit*; otherwise the scalar-result is a <bit-string-value+
containing <zero-bit%
9.3.2.10 Infix-multiply
Constraints: All scalar <data-type>s of x and y must have <computational-type>.
The <aggregate-type% of x and y must be compatible.
Attributes: Each scalar-result-type has <arithmetic> with the derived common <base>,
<scale>, and <mode> of the corresponding scalar <data-type>s of x and y. If
the result <scale> has <float>, then m = max(p,r). If the result <scale> has
<fixed>, then
m = min(N,p+r+l)
n = q+s.
The result <aggregate-type+ is the common <aggregate-type> of x and y.
Operation: infix-multiply(rdd,x,y)
Perform generate-aggregate-result.
Step 1. Convert the scalar-values of x and y to target scalar <data-type% which have
<arithmetic> with the derived common <base>, <scale>, and <mode> of the scalar
<data-type% of x and y. The target <precision> for the conversion of the
scalar-value of x (respectively, y) is the converted <precision> of the scalar
<data-type> of x (respectively. y). Let x' and y' be the converted values.
Step 2. Perform arithmetic-result(x**y',scalar-result-type), to obtain the scalar-
result.
310
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 329

9.3.2.11 Infix-ne
Constraints: Corresponding scalar <data-type>s of x and y must:
(1) both have <computational-type>, or
(2) both have <locator>, or
(3) have <non-computational-type>, with the immediate subnodes of the
<non-computational-type>s belonging to the same category other
than <locator> or <area>.
Further, if one scalar <data-type> has <offset> and the other has <pointer>,
then the <offset> must contain a <variable-reference>.
The <aggregate-type% of x and y must be compatible.
Attributes: Each scalar-result-type has <bit>.
The result eaqqreqate-type, is the common <aggregate-type+ of x and y.
Operation: infix-ne(rdd,x,y)
Perform generate-aggregate-result.
Step 1. Perform compare(x*,y',tCll,t123) to obtain camp, where x' is the scalar-value of
x, y' is the scalar-value of y, t[ll is the scalar <data-type> of x, and tl21 is
the scalar <data-type> of y.
Step 2. If camp is <not-equal,, the scalar-result is a <bit-string-value+ containing
<one-bit,; otherwise the scalar-result is a <bit-string-value+ containing fzero-
bit,.
9.3.2.12 Infix-or
Constraints: All scalar <data-type% of x and y must have <computational-type>.
The <aggregate-type% of x and y must be compatible.
Attributes: Each scalar-result-type has <bit>.
The result <aggregate-type> is the common <aggregate-type% of x and y.
Operation: infix-or(rdd,x,y)
Perform generate-aggregate-result.
Step 1. Convert the scalar-values of x and y to <bit>. Let n be the maximum of the
lengths of the converted values.
Step 2. If the length of one converted value is less than n. convert it to <bit> of
specified length n. Let x' and y' be the final converted values.
Step 3. If n = 0, the scalar-result is a fbit-string-value,: fnull-bit-strinq,.
Otherwise the scalar-result is a <bit-string-value+ of length n. The i'th fbit-
value+ of the result is <zero-bit+ if the i'th fbit-value% of both X* and y'
are <zero-bit+; otherwise the i'th <bit-value, of the result is <one-bit,.
Chapter 9: Expressions and Conversion 311
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 330

9.3.2.13 Infix-power
Constraints: All scalar <data-type>s of x and y must have <computational-type>.
The <aggregate-type+s of x and y must be compatible.
Attributes: Each scalar-result-type has <arithmetic>. The result <base>, <scale>,
Case 1
Case 2
Case 3
<mode>, and <precision> are determined from the corresponding scalar <data-
type& of x and y as follows.
The result <aggregate-type, is the common <aggregate-type+ of x and y.
The derived <scale> of x has <fixed>; y is a <constant> with <mode>:
0; and whose valu-, is positive; (p+l)*y'-1 does not
<real>;,
<scale-factor>: exceed
the maximum <number-of-digits> for the derived <base> and <scale> of x.
The result <scale> has <fixed>, the result <base> and <mode> has the derived
<base> and <mode> of x; and
m =(p+l)+y*-1,
n = q*y'.
The derived <scale> of y has <fixed>, the derived <mode> of y has <real>, s=O,
but Case 1 does not hold.
The result <scale> has <float>, the result <base> and <mode> have the derived
<base> and <mode> of x, and m=p.
(Otherwise).
The result <scale> has <float>; the result <base> and <mode> have the derived
common <base> and <mode> of x and y', and m = max(p,r).
Operation: infix-power(rdd,x,y)
Perform generate-aggregate-result.
Step 1. Determine values u and v as follows:
Case 1.1. Conditions the same as for Attributes, Case 1.
Let u and v be the scalar-values of x and y, respectively.
Case 1.2. Conditions the same as for Attributes, Case 2.
Convert the scalar-value of x to the scalar-result-type, and let u be the
converted value. Let v be the scalar-value of y.
Case 1.3. Conditions the same as for Attributes, Case 3.
Convert the scalar-values of x and y to target scalar <data-type>s which
have <arithmetic> with <scale>: <float>;, which have the derived common
<base> and <mode> of the scalar <data-type>s of x and y, and whose
<precision>s are the converted <precision% of the scalar <data-type>s of x
and y, respectively. Let u and v be the converted scalar-values of x and y,
respectively.
Step 2. Determine a value z as follows:
Case 2.1. The result <mode> is <real> and u < 0.
If the conditions of Attributes, Case 1 or Case 2, hold, then z=utv;
otherwise, perform raise-condition(<error-condition>).
Case 2.2. The result <mode> is <real>, and u = 0.
If v I 0 then perform raise-condition(<error-condition>); otherwise let z =
0.
312
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 331

Case 2.3. The result <mode> is <real>, and u > 0.
Then:
2 = l/(ut-v), if v < 0,
z = 1, if v = 0,
z = utv, if v > 0.
Case 2.4. The result <mode> has <complex>.
Interpret both u and v as <complex-number%.
Case 2.4.1. u = 0, the real part of v is greater than zero, and the imaginary part
of v is zero.
z = 0.
Case 2.4.2. u = 0, but the conditions of Case 2.4.1 do not hold.
Perform raise-condition(<error-condition>).
Case 2.4.3. u + 0.
Z = g t(v*log(u)), where log(u) is that value of the complex logarithm
function whose imaginary part w is in the interval -E < w 5 a.
Step 3. Perform arithmetic-result(z,scalar-result-type) to obtain the scalar-result.
9.3.2.14 Infix-subtract
Constraints: All scalar <data-type>s of x and y must have <computational-type>.
The <aggregate-type% of x and y must be compatible.
Attributes: Each scalar-result-type has <arithmetic> with the derived common <base>,
<scale>, and <mode> of the corresponding scalar <data-type>s of x and y. If
the result <scale> has <float>, then m = max(p,r). If the result <scale> has
<fixed>, then
m = min(N,max(p-q,r-s)+max(q,s)+l)
n = max(q,s).
The result <aggregate-type+ is the common *aggregate-type> of x and y.
Operation: infix-subtract(rdd,x,y)
Perform generate-aggregate-result.
Step 1. Convert the scalar-values of x and y to target scalar <data-type>s which have
<arithmetic> with the derived common <base>, <scale>, and <mode> of the scalar
<data-type>s of x and y. The target <precision> for the conversion of the
scalar-value of x (respectively, y) is the converted <precision> of the scalar
<data-type> of x (respectively, y). Let x' and y' be the converted values.
Step 2. Perform arithmetic-result(x'-y',scalar-result-type), to obtain the scalar-
result.
Chapter 9: Expressions and Conversion 313
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 332

9.4 Builtin-functions
This section describes the <builtin-function>s available in PL/I. Section 9.4.1 gives
the general rules for evaluating a <builtin-function-reference>, and Section 9.4.4
presents the details for each <builtin-function> in alphabetical order.
In this section an additional heading "Arguments" appears in the description of each
<builtin-function>. The number of <argument>s required by the <builtin-function> and the
<argument> names used in the description are given under this heading. The <argument>
names are separated by commas and the names of optional <argument% are enclosed in
brackets. An <argument-list> of indeterminate length, e.g. the <argument-list> of the
<max-bif>, is indicated by ellipsis.
Arguments: x.yCrpC.qll
The letters x, y, p, and q will be used to refer to the <arguments% in the
description of the <builtin-function>. The <argument>s p and q are optional, i.e.
neither need be specified; but if q is specified, then p must also be specified.
**************************t**+********+*******************************************~*****
Example 9.3. An Example of Optional Arguments.
9.4.1 BUILTIN-FUNCTION REFERENCE
Operation: evaluate-builtin-function-referencecbfr)
where bfr is a <builtin-function-reference>.
result: an *aggregate-value+.
Step 1. Let bif be the <builtin-function> immediately contained in bfr. Let x[ll,
x121 ,...,x[nl be the <argument>s, if any, simply contained in bfr.
Step 2.
Case 2.1. bif is <collate-bif>, <date-bif>. <empty-bit>, <null-bif>, <onchar-bif>,
<oncode-bif>, <onfield-bif>, <onfile-bif>, <onkev-bif>, <onloc-bif>,
<onsource-bif>, or <time-bif>.
Perform f, where f is the operation whose name is the same as that of bif,
to obtain an <aggregate-value+,v. Return v.
Case 2.2. -(Otherwise).
Let rdd be the <data-description> immediately contained in bfr. Perform
f(rad,x~13,xC23,...,x[n3). where f is the operation whose name is the same
as that of bif, to obtain an <aggregate-value>,v. Return v.
9.4.2 SPECIAL TERMS DEFINED FOR BUILTIN-FUNCTIONS
9.4.2-l Definition of N
Under the heading "Attributes", N is used in describing the <precision> of the result
<data-type>. It denotes the maximum <number-of-digits> allowed by the implementation for
the result <base> and <scale>.
314
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 333

9.4.2.2 The Arguments P and q
Constraints on o and q
When p and q are used under the heading "Arguments", certain special rules apply. Both p
and q must be of the form <argument>: <expression>: <constant>: <data-type>,dt;;;, where
dt has <fixed> and <decimal> and has <scale-factor> equal to zero. The value of p must
be greater than zero and less than or equal to N, i.e. the maximum <number-of-digits> for
the result <base> and <scale>. q must not occur unless each scalar-result-type has
<fixed>.
Attributes of Result Determined by p and q.
Case 1. p and q are both specified.
The <number-of-digits> of the result-type is the value of p. The <scale-factor>
of the result-type is the value of q.
Case 2. p is specified, q is not specified-
The <number-of-digits> of the result-type is the value of p. If the <scale> of
the result-type has <fixed>, then the <scale-factor> of the result-type has 0.
Case 3. Neither p nor q is specified.
The <precision> of the result-type is the converted common <precision> of the
<argument>s if there is more than one, and the converted <precision> of the
<argument> otherwise.
9.4.3 OPERATIONS USED IN BUILTIN-FUNCTION DEFINITIONS
9.4.3.1 Get-established-onvalue
Operation: get-established-onvalue(tp1
where tp is one of fonchar-value,, foncode-value,, fonfield-value,. fonfile-
value,, fonkey-value,, fonloc-values, or fonsource-value,.
result: a <character-string-value+, an <integer-value+, or <fail>.
Step 1. Let bs be the current &lock-state+.
Step 2.
Case 2.1. bs contains a 4condition-bif-value,rcbv, with an immediate component of the
type indicated by tp. Let r be the <integer-value+ or fcharacter-string-
value, component of cbv.
Return r.
Case 2.2. (Otherwise).
Case 2.2.1. There is a <block-state, immediately preceding bs in the fblock-state-
list,.
Let bs be this <block-state,. Go to Step 2.
Case 2.2.2. (Otherwise).
Return l =3.
Chapter 9: Expressions and Conversion 315
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 334

9.4..4 DEFINITION OF THE BUILTIN-FUNCTIONS
The descriptions of the <builtin-function7s are given in the following sections in
alphabetical order.
9.4.4.1 Abs-bif
Arguments: x
Constraints: All <data-type>s of x must have <computational-type>.
Attributes: The result <aggregate-type* is the <aggregate-types of x. Each scalar-
result-type has - <real>. Its <base7 is the derived <base> of the
corresponding <data-type> of x. The scalar-result-type <scale> and
<precision> are determined as follows:
Case 1,. The derived <mode7 of the corresponding <data-type> of x is <real>.
The <scale7 and <precision7 of the scalar-result-type are the derived <scale7
and converted <precision7 of the <data-type>.
Case 2. The derived <mode> of the corresponding <data-type7 of x is <complex>.
Let r be the converted <number-of-digits> of the corresponding <data-type>, and
let s be its converted <scale-factor>. Then the scalar-result-type <number-of-
digits7 is min(N,r+l), and the scalar-result-type <scale-factor7 is s.
Operation: abs-bif(rdd,x)
Perform generate-aggregate-result.
Step 1.
Case 1.1. The derived <mode7 of the <data-type> of x has <real>.
Convert the scalar-value of x to the scalar-result-type. Let y=lzl where x
is the converted value.
Case 1.2. The derived <mode> of the <data-type> of x has <complex>.
Convert the scalar-value of x to a target type which is the same as the
scalar-result-type except that the target Unode7 is <complex>. Perform
conditions-in-arithmetic-expression( where rt is the result-type.
Determine y. which is the positive square root of (ut2+vt2), where u and v
are, respectively, the real and imaginary parts of the converted value.
Step 2. Perform arithmetic-result(y,rt), where rt is the scalar-result-type, to obtain
the scalar-result.
316
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 335

9.4.4.2 Aces-bif
Arguments: x
Constraints: All <data-type>s of x must have <computational-type>. The derived <mode> of
the <data-type>s of x must have <real>.
Attributes: The result <aggregate-type+ is the <aggregate-type+ of x. The <scale> of
the scalar-result-type has <float>. The <base>, <mode>, and <precision> of
the scalar-result-type are the derived <base>, <mode>, and converted
<precision> of the corresponding <data-type> of x.
Operation: aces-bif(rdd,x)
Perform generate-aggregate-result.
Step 1. Convert the scalar-value of x to the scalar-result-type to obtain y.
Step 2. The <data-type> of y has <&>. The value of y must be between -1 and 1,
inclusive. Perform conditions-in-arithmetic-expression( where rt is the
scalar-result-type. Let w be the arc cosine of y. in radians, such that
0 5 w I a.
Step 3. Perform arithmetic-result(w,rt), where rt is the scalar-result-type, to obtain
the scalar-result.
9.4.4.3 Ada-bif
Arguments: x,y,p[,ql
Constraints: The <aggregate-type>s of x and y must be compatible. All <data-type% of x
and y must have <computational-type>. Constraints on p and q are described
in Section 9.4.2.2.
Attributes: The result <aggregate-type+ is the common <aggregate-type* of x and y- The
<base>, <scale>, and <mode> of the scalar-result-type are the derived common
<base>, <scale>, and <mode> of the corresponding <data-type% of x and y.
The <precision> of the scalar-result-type is determined as defined in
Section 9.4.2.2.
Operation: add-biffraa,x,y,pc,ql)
Perform generate-aggregate-result.
Step 1. Perform Steps 1.1 and 1.2 in either order.
Step 1.1. Convert the scalar-value of x to the derived common <base>, <scale>, and
<mode> of the corresponding <data-type% of x and yI and the converted
<precision> of the <data-type> of x.
Step 1.2. Convert the scalar-value of y to the derived common <base>, <scale>, and
<mode> of the corresponding <data-type>s of x and y, and the converted
<precision> of the <data-type> of y-
Step 2. Let z be the sum of the converted scalar-value of x and the converted scalar-
value of y.
Step 3. Perform arithmetic-result(z,rt), where rt is the scalar-result-type, to obtain
the scalar-result.
Chapter 9: Expressions and Conversion 317
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 336

9.4.4.4 Addr-bif
Arguments: x
Constraints: x must be of the form <argument>: <expression>: <value-reference>:
<variable-reference>.y.
Attributes: The result <aggregate-type+ immediately contains *scalar+. The result
<data-type> has <pointer>.
Operation: addr-bif(rdd,x)
Step 1. Let dp be the <declaration-designator> immediately contained in y.
Step 2. If the <declaration>,d, designated by dp contains <controlled>, perform find-
directory-entry to obtain the corresponding <controlled-directory-entry9,
cde.
Case 2.1. d contains <controlled> and cde does not contain a <generation-list,.
Return an <aggregate-value9 containing <pointer-values: fnull9.
Case 2.2. (Otherwise).
Perform evaluate-variable-reference(y) to obtain a fgeneratio&,g, which
must be connected. Return an <aggregate-value> containing *pointer-value+:
ST-
3.4.4.5 After-bif
Arguments: sa,ca
Constraints: The eaggregate-type% of sa and ca must be compatible. All <data-type>s of
sa and ca must have <computational-type>.
Attributes: The result iaggregate-type, is the common <aggregate-type> of sa and ca.
Each scalar-result-type has <string>. The <string-type> is the derived
common <string-type> of corresponding <data-type>s of sa and ca.
Operation: after-bif(rdd,sa,ca)
Perform generate-aggregate-result.
Step 1. In either order, convert the scalar-value of sa to the scalar-result-type to
obtain sb and convert the scalar-value of ca to the scalar-result-type to obtain
cb.
Step 2.
Case 2.1. The string sb is a null-string.
The scalar-result is a null-string.
Case 2.2. The string cb is a null-string.
The scalar-result is the string sb.
Case 2.3. The string cb is not a null-string, and cb is not a substring of sb.
The scalar-result is a null-string.
Case 2.4. The string cb is a substring of sb.
Let i denote the position of the last *it-value, or *character-values of
the leftmost occurrence of cb in sb. Let j denote the position of the last
<bit-value, or *character-value+ in sb. If i=j, then the scalar-result is a
null-string. If i+j then the scalar-result is a string of length (j-i) whose
k'th <bit-value+ or <character-value+ Gllksj-i> is the *bit-values or
<character-value> in position (i+kl in the string sb.
318
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 337

9.4.4.6 Allocation-bif
Arguments: x
Constraints: x must be of the form <argument>: <expression>: <value-reference>:
<variable-reference>,y;;;, and y must not contain an <identifier-list> or
<sIbscript-list>. The <declaration-designator> of y must designate a
<declaration> which contains <controlled>.
Attributes: The result <aggregate-type+ immediately contains *scalar+. The result
<data-type> is integer-type.
Operation: allocation-bif(rdd,x)
step 1. Let y be the <variable-reference> simply contained in x. Let dp be the
<declaration-designator> immediately contained in y.
Step 2. Perform find-directory-entry to obtain the corresponding fcontrolled-
directory-entry>,cde.
Case 2.1. cde immediately contains a <generation-list+,ge.
Let n be the number of <generation% immediately contained in ge.
Case 2.2. (Otherwise).
Let n be 0.
step 3. Return an <aggregate-value, containing freal-value,: n.
9.4.4.7 Asin-bif
Arguments: x
Constraints: All <data-type>s of x must have <computational-type>. The derived <mode> of
the <data-type% of x must have <w>.
Attributes: The result *aggregate-type* is the <aggregate-type+ of x. The <scale> of
the scalar-result-type has <float>. The <base>, <mode>, and <precision> of
the scalar-result-type are the derived <base>, <mode>, and converted
<precision> of the corresponding <data-type> of x.
Operation: asin-bif(rdd,x)
Perform generate-aggregate-result.
step 1.. Convert the scalar-value of x to the scalar-result-type to obtain y.
Step 2. The <data-type> of y has - <real>. The value of y must be between -1 and 1,
inclusive. Perform conditions-in-arithmetic-expression( where rt is the
scalar-result-type. Let w be the arc sine of yr in radians, such that
-pi/2 s w I a/2.
Step 3. Perform arithmetic-result(w,rt), where rt is the scalar-result-type, to obtain
the scalar-result.
Chapter 9: Expressions and Conversion 319
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 338

9.4.4-8 Atan-bif
Arguments: y[,xl
Constraints: The <aggregate-type% of y and x must be compatible. All <data-type>s of y
and x must have <computational-type>. If the derived <mode> of the <data-
type> of y has <real> and x occurs, the <data-type> of x must also have
<real>. If the derived <mode> of the <data-type> of y has <complex>, x must
nze specified.
Attributes: The result <aggregate-type> is the common *aggregate-type> of y and x. The
scalar-result-type has <scale>: <float>. The <base> and <mode> of the
scalar-result-type are the derived common <base> and <mode> of corresponding
<data-type>s in y and x. The <precision> of the scalar-result-type is the
greater of the converted <precision>s of the corresponding <data-type>s of y
and x.
Operation: atan-bif(rdd,yI,xl)
Perform generate-aggregate-result.
step 1.
Case 1.1. x is not <absent,.
In either order, convert the scalar-value of y to the scalar-result-type to
obtain s and convert the scalar-value of x to the scalar-result-type to
obtain r. The values of r and s must not both be 0.
Case 1.2. x is <absent>.
Convert the scalar-value of y to the scalar-result-type to obtain s. If the
scalar-result-type has <complex>, s must not be +L-
Step 2. Perform conditions-in-arithmetic-expression(rt>, where rt is the scalar-result-
type -
Case 2.1. x is not <absent+.
Let v be the value, in radians, of arctangent such that
if s 2 0 then 0 I v I a, and
if s < 0 then -pi < v < 0.
Case 2.2. x is <absent, and rt has <a>.
Let v be the arc tangent of s, such that
-e/2 < v < g/2.
Case 2.3. rt has <complex>.
Let v be the arc tangent of s, where the real part of the result, w,
satisfies
-pi < w I gg.
step 3.. Perform arithmetic-result(v,rtl to obtain the scalar-result.
320
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 339

9.4.4.9 Atand-bif
Arguments: y[,xl
Constraints: The *aggregate-type% of y and x must be compatible,. The derived <mode> of
the <data-type% of y and x, if specified, must have <real>.
Attributes: The result <aggregate-type, is the common *aggregate-type, of y and x. The
<mode> of the scalar-result-type has <real>. The <scale> of the scalar-
result-type has <float>. The <base> of the scalar-result-type is the
derived common <base> of the <data-type>s of y and x. The <precision> of
the scalar-result-type is the greater of the converted <precisiorOs of
corresponding <data-type>s of y and x.
Operation: atand-bif(rdd,yC,xll
Perform generate-aggregate-result.
Step 1. Perform atan-bif(y,x) to obtain a value, v..
Step 2. Let w be the value of v multiplied by 18O/pi-.
Step 3. Perform arithmetic-result(w,rt), where rt is the scalar-result-type, to obtain
the scalar-result.
g-4,.4.10 Atanh-bif
Arguments: x
Constraints: All <data-type>s of x must have computational type,.
Attributes: The result <aggregate-type, is the <aggregate-type, of x- The <scale> of
the scalar-result-type has <float>. The <base>, <mode>, and <precision> of
the scalar-result-type are the derived <base>, <mode>, and converted
<precision> of the corresponding <data-type> of x.
Operation: atanh-bif(rdd,x)
Perform generate-aggregate-result.
step 1. Convert the scalar-value of x to the scalar-result-type to obtain y. If the
scalar-result-type has <a>, v must be less than 1 in absolute value. If the
scalar-result-type has <complex>, y must not be 1 or -1.
Step 2. Perform conditions-in-arithmetic-expression(rt1, where rt is the scalar-result-
type - Let w be the arc-hyperbolic tangent of y-
Step 3. Perform arithmetic-result(w,rtl, where rt is the scalar-result-type, to obtain
the scalar-result.
Chapter 9: Expressions and Conversion 321
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 340

9.4.4.11 Before-bif
Arguments: sa,ca
Constraints: The <aggregate-type% of sa and ca must be compatible. All <data-type>s of
sa and ca must have <computational-type>.
Attributes: The result <aggregate-type+ is the common <aggregate-type+ of sa and ca.
The scalar-result-type has <string>, having the derived common <string-type>
of the corresponding <data-type>s of sa and ca.
Operation: before-bif(rdd,sa,cal
Perform generate-aggregate-result.
Step 1. In either order, convert the scalar-value of sa to the scalar-result-type to
obtain sb and convert the scalar-value of ca to the scalar-result-type to obtain
cb.
Step 2.
Case 2.1. The string sb is a null-string.
The scalar-result is a null-string.
Case 2.2. The string cb is a null-string.
The scalar-result is a null-string.
Case 2.3. The string cb is not a null-string, and it is not a substring of sb.
The scalar-result is the string sb.
Case 2.4. The string cb is a substring of sb.
Let i denote the position of the first <bit-valueb or <character-value+ Of
the leftmost occurrence of cb in sb. If i=l, then the scalar-result is a
null-string. If i > 1, then the scalar-result is a string of length (i-1)
whose j'th <bit-value+ or <character-value+ (1 I j 5 i-l) is the j'th fbit-
value, or <character-value, in the string sb.
9.4.4.12 Binary-bif
Arguments: xc.p~,qll
Constraints: All <data-type>s of x must have <computational-type>. Constraints on p and
q are described in Section 9.4.2.2.
Attributes: The result <aggregate-type+ is the <aggregate-type, of x. The <base> of the
scalar-result-type has <binary>. The unode> and <scale> of the scalar-
result-type are the derived Unode> and <scale> of the corresponding <data-
type> of x. The <precision> of the scalar-result-type is determined as
defined in Section 9.4.2.2.
Operation: binary-bif(rdd,xI,p[,qll)
Perform generate-aggregate-result.
Step 1. Convert the scalar-value of x to the scalar-result-type to obtain v.
Step 2. Perform arithmetic-result(v,rt)r where rt is the scalar-result-type, to obtain
the scalar-result.
322

322
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 341

9.4.4.13 Bit-bif
Arguments: xC,lel
Constraints: All <data-type>s of x and le must have <computational-type>. le must have
<aggregate-type+: <scalar>.
Attributes: The result <aggregate-type, is the <aggregate-type> of x. The scalar-result-
type has <bit>.
Operation: bit-bif(rdd,xC,lel)
Perform generate-aggregate-result.
Step 1.
Case 1.1. le is tabsent,.
Convert the scalar-value of x to <bit> to obtain the scalar-result.
Case 1.2. le is not fabsents.
Convert le to integer-type. The result, n, must not be negative. Convert
the scalar-value of x to <bit> of length n to obtain the scalar-result.
9.4.4.14 Pool-bif
Arguments: x#y,ca
Constraints: The <aggregate-type% of x and y must be compatible. All <data-type>s of x,
y, and ca must have <computational-type>. The <aggregate-type> of ca must
have tscalar,.
Attributes: The result <aggregate-type+ is the common <aggregate-type> of x and y. The
scalar-result-type has 0b>.
Operation: bool-bif(rdd,x,y,ca)
Perform generate-aggregate-result-
Step 1. In either order, convert the scalar-value of x to <bit> to obtain ra and convert
the scalar-value of y to <bit> to obtain rb.
lengths of ra and rb. In either order,
Let r be the greater of the
convert ra to <bit> of length le to
obtain sa, and convert rb to <m> of length le to obtain sb7
step 2. Convert ca to <@> of length 4 to obtain d. Let the <bit-value>s within d be
named dill, dC21, dC33, dC41, from left-to-right, respectively.
Step 3.
Case 3.1. le = 0.
The scalar-result is a null-string.
Case 3.2. le > 0.
The i'th <bit-value3 of the scalar-result is set to one of the values dC13,
dC21, d133, dC43 depending on the i'th fbit-value% of sa and sb as shown in
Table 9.2.
Chapter 9: Expressions and Conversion 323
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 342

Table 9.2. Table of Scalar-results as a Function of <bit-valueks for Bool-bif.
r----------------------------------------e---,
I i'th <bit-value, of I
scalar-result
I *****+*****+***********+***************************************************************** In this example BOOL is used to perform an exclusive-or operation on the first two
arguments. The value '1001"B is the result of the evaluation of:
BOOL('1100'B,'0101'B,'0110"B)
i
Example 9.4. An Example of the BOOL bif.
9.4.4.15 Ceil-bif
Arguments: x
Constraints: The derived <mode> of the <data-type>s of x must have <real>.
Attributes: The result <aggregate-type> is the <aggregate-type+ of x. The scalar-result-
type has <real>. The scalar-result-type has the derived <base> and <scale>
of the cor=onding <data-type> of x-
Case 1. The scalar-result-type has <scale>: <float>.
The <precision> of the scalar-result-type is the converted <precision> of the
<data-type> of x.
Case 2. The scalar-result-type has <scale>: <fixed>.
Let r denote the <number-of-digits> and s denote the <scale-factor> of the
converted value of x. The <precision> of the scalar-result-type has:
<number-of-digits>: min(N,max(r-s+l,l));
<scale-factor>: 0.
Operation: ceil-bif(rdd,x)
Perform generate-aggregate-result.
Step 1. Convert the scalar-value of x to its derived mode>, <base>, <scale>, and
converted <precision>.
Step 2. Perform conditions-in-arithmetic-expression( where rt is the scalar-result-
type- Let v be the smallest integer that is greater than or equal to the
converted scalar-value of x.
step 3. Perform arithmetic-result(v,rt), where rt is the scalar-result-type, to obtain
the scalar-result.
324
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 343

9.4.4.16 Character-bif
Arguments: saC,lel
Constraints: All <data-type>s of sa must have <computational-type>. le must have
<aggregate-type>: &calar+.
Attributes: The result eaggregate-type, is the <aggregate-type> of sa. The scalar-
result-type has <character>.
Operation: character-bif(rdd,saC,lelI
Perform generate-aggregate-result.
Case 1. le is *absent+.
Convert the scalar-value of sa to <character> to obtain the scalar-result.
Case 2. le is not <absent>.
Convert le to integer-type. The result, n, must not be negative. Convert the
scalar-value of sa to <character> of length n to obtain the scalar-result.
9.4.4.17 Collate-bif
Arguments: (none)
Attributes: The result <aggregate-type+ immediately contains <scalar*. The result
<data-type> has <character>.
Operation: collate-bif
Step 1. Letvbe a <character-string-value+ containing all the terminal nodes of the
category {symbol) just once in an implementation-defined order known as the
collating sequence. (See Section 9.3,.2.5.1 for use of this sequence in the
comparison of <character-string-value,s.)
Step 2. Return an <aggregate-value% containing v.
Chapter 9: Expressions and Conversion 325
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 344

9.4.4.18 Complex-bif
Arguments: x,y
Constraints: The faggregate-type+s of x and y must be compatible. The derived common
<mode> of the <data-type>s of x and y must have <&>.
Attributes: The result <aggregate-type+ is the common <aggregate-type> of x and y- The
scalar-result-type has <complex>. The <base> and <scale> of the scalar-
result-type are the derived common <base> and <scale> of the corresponding
<data-type>s of x and y. Let r and s denote the converted <number-of-digits>
and <scale-factor> of the <data-type> of x, let t and u denote the converted
<number-of-digits> and <scale-factor> of the corresponding <data-type> of y,
and let p and q denote the <number-of-digits> and <scale-factor> of the
scalar-result-type.
Case 1. The scalar-result-type <scale> has <fixed>.
p = min(N,max(r-s,t-u) +max(s,u))
q = max(s,u).
Case 2. The scalar-result-type <scale> has <float>.
p = max(r,t).
Operation: complex-bif(rdd,x,y)
Perform generate-aggregate-result.
Step 1. Let v be a <complex-value* whose real part is the scalar-value of x converted to
the <base>, <scale>, and <precision> of the scalar-result-type and <mode>:
<real>; and whose imaginary part is the scalar-value of y similarly converted.
Step 2. Perform arithmetic-result(v,rt), where rt is the scalar-result-type, to obtain
the scalar-result.
9.4.4.19 Conis-bif
Arguments: x
Constraints: All <data-type>s of x must have <computational-type>.
Attributes: The result <aggregate-type, is the <aggregate-type+ of x. The scalar-result-
type has <complex>. The scalar-result-type has the derived <base>, <scale>,
and converted <precision> of the corresponding <data-type> of x.
Operation: conjq-bif(rdd,x)
Perform generate-aggregate-result.
Step 1. Convert the scalar-value of x to the scalar-result-type. Let v be the complex
conjugate of the converted scalar-value.
Step 2. Perform arithmetic-result(v,rt), where rt is the scalar-result-type, to obtain
the scalar-result.
326
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 345

9.4.4.20 Copv-bif
Arguments: sa,le
Constraints: All <data-type>s of sa and le must have <computational-type>. le must have
<aggregate-type+: <scalar+.
Attributes: The result <aggregate-type+ is the <aggregate-type+ of sa. The scalar-
result-type has the derived <string-type> of the corresponding <data-type>
of sa.
Operation: copy-bif(rdd,sa,leI
Perform generate-aggregate-result.
step 1. Perform Steps 1.1. and 1.2. in either order.
Step 1.1. Convert le to integer-type. The result, n, must not be negative.
Step 1.2. Convert the scalar-value of sa to the scalar-result-type, to obtain sv.
step 2.
Case 2.1. n > 0.
Let v be a null-string. Perform Step 2(.1-l n times. The scalar-result is
V.
Step 2.1.1. Perform concatenate(v,sv) to obtain v.
Case 2.2. n = 0.
The scalar-result is a null-string.
s**+**********+*+*+**+*+*++*+****************************************~************************~
An invocation of
COPY('12',3)
will return the *character-string-value, '121212'.
**************+c**++**++*************+*********************~************************~******~
Example 9.5. An Example of the COPY bif.
9.4..4.21 Cos-bif
Arguments: x
Constraints: All <data-type>s of x must have <computational-type>.
Attributes: The result <aggregate-type, is the iaggregate-type, of x. The <scale> of the
scalar-result-type has <float>,. The <base>, <mode>, and <precision> of the
scalar-result-type have the derived <base>, <mode>, and converted
<precision> of the corresponding <data-type> of x.
Operation: cos-bif(rdd,x)
Perform generate-aggregate-result.
step 1. Convert the scalar-value of x to the scalar-result-type. The value of x is
assumed to be given in radians.
step 2. Perform conditions-in-arithmetic-expression( where rt is the scalar-result-
type- Let v be the cosine of the converted scalar-value of x.
step 3. Perform arithmetic-resulttv,rt), to obtain the scalar-result.
Chapter 9: Expressions and Conversion 327
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 346

9.4.4.22 Cosd-bif
Arguments: x
Constraints: All <data-type>s of x must have <computational-type>. The derived <mode> of
the <data-type% of x must have <real>.
Attributes: The result <aggregate-type+ is the <aggregate-type, of x. The <scale> of the
scalar-result-type has <float>. The <mode> of the scalar-result-type has
<real>. The <base> of the scalar-result-type is the derived <base> of the
corresponding <data-type> of x. The <precision> of the scalar-result-type
is the converted <precision> of the corresponding <data-type> of x.
Operation: cosd-bif(rdd,xl
Perform generate-aggregate-result.
Step 1. Convert the scalar-value of x to the scalar-result-type. The value of x is
assumed to be given in degrees.
Step 2. Perform conditions-in-arithmetic-expression( where rt is the scalar-result-
type- Let v be the cosine of the converted scalar-value of x-
Step 3. Perform arithmetic-result(v,rt), to obtain the scalar-result.
9.4.4.23 Cosh-bif
Arguments: x
Constraints: All <data-type>s of x must have <computational-type>.
Attributes: The result <aggregate-type+ is the <aggregate-type, of x. The <scale> of
the scalar-result-type has <float>. The <base>, <mode>, and <precision> of
the scalar-result-type are the derived <base>, <mode>, and converted
<precision> of the corresponding <data-type> of x.
Operation: cash-bif(rdd,x)
Perform generate-aggregate-result.
Step 1. Convert the scalar-value of x to the scalar-result-type to obtain w-
Step 2. Perform conditions-in-arithmetic-expression( where rt is the scalar-result-
type - Let v be the hyperbolic cosine of w.
Step 3. Perform arithmetic-result(v,rt) to obtain the scalar-result.
9.4.4.24 Date-bif
Arguments: (none)
Attributes: The result <aggregate-type* immediately contains <scalar+. The result
<data-type> has <character>.
Operation: date-bif
The result has length 6. It represents the date in the form "yymmdd" where:
each letter represents one <character-value in the result,
each &character-value+ is a digit,
yy. mm and dd are respectively in the ranges 00:99, 01:12, and 01:31,
representing year, month, and day.
328
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 347

9,.4.4.25 Decat-bif
Arguments: sa.ca,pa
Constraints: All <data-type>s of sa, ca, and pa must have <computational-type>. pa must
have <aggregate-type+: <scalar+.
Attributes: The result <aggregate-type, is the common *aggregate-type+ of sa and ca.
The scalar-result-type has the derived common <string-type> of sa and ca.
Operation: decat-bif(rdd,sa,ca,pa)
Perform generate-aggregate-result.
Step 1. Perform Steps 1.1 through 1.3 in any order.
Step 1.1. Convert the value of pa to <&> of length 3 to obtain pb-
step 1.2. Convert the scalar-value of sa to the scalar-result-type to obtain sb.
Step 1.3. Convert the scalar-value of ca to the scalar-result-type to obtain cb.
Step 2. Divide sb into 3 strings, sbfll, sb[21, sbL31:
Case 2.1. sb is a null-string.
sbtll, sbC21, and sb[31 are null-strings.
Case 2.2. sb is not a null-string and cb is a null-string.
sbC11 and sbC21 are null-strings and sbC31 is the string sb.
Case 2.3. sb and cb are not null-strings and cb is a substring of sb.
sbCl1 is the substring of sb before the leftmost occurrence of cb in sb.
sbC21 is the string cb and sbC31 is the substring of sb after the leftmost
occurrence of cb in sb.
Case 2.4. (Otherwise)-
sbC11 is the string sb and sbC21 and sb[31 are null-strings-
Step 3. For i=l,2.3, let t[il be sCi1 if the i'th <bit-value+ of pb has <one-bit+,
otherwise let t[il be a null-string- Perform concatenate(tCll,tC23) to obtain
V. Perform concatenate(v,tC31) to obtain the scalar-result.
1
*****+*****************~~*******+*+*+*+***********************************************
The value of DECAT('XYZ','Y','OO~*B) is the <character-string-value+ '5'. Note
that this result is the same as the value of AFTER('XYZ'. 'Y'). The value of
DECAT('XYZ','Y'. *Oll*B) is the <character-string-value+ *YZ'
*******+****+*+**********************+$************************************************
Example 9.6. An Example of the DECAT bif.
Chapter 9: Expressions and Conversion 329
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 348

9.4.4.26 Decimal-bif
Arguments: xt,Pl,qll
Constraints: All <data-type>s of x must have <computational-type>. Constraints on p and q
are described in Section 9.4.2.2.
Attributes: The result <aggregate-type> is the <aggregate-type, of x. The <base> of the
scalar-result-type has <decimal>. The <mode> and <scale> of the scalar-
result-type are the derived <mode> and <scale> of the corresponding <data-
type> of x. The <precision> of the scalar-result-type is determined as
defined in Section 9.4.2.2.
Operation: decimal-bif(rdd,x[,p[,qll)
Perform generate-aggregate-result.
Step 1. Convert the the scalar-value of x to the scalar-result-type to obtain v.
step 2. Perform arithmetic-result(v,rt), where rt is the scalar-result-type, to obtain
the scalar-result.
9.4.4.27 Dimension-bif
Arguments: x,n
Constraints: The <data-type> of n must have <computational-type>. n must have
<aggregate-type+: fscalar,. x must have the form <argument>: <expression>:
<value-reference>: <variable-reference>: <data-description>: <dimensioned-
data-description>.
Attributes: The result *aggregate-type+ immediately contains <scalar%. The result
<data-type> is integer-type.
Operation: dimension-bif(rdd,x,nl
Step 1.
Step 2.
Step 3.
Let y be the <variable-reference> in x. Let dp be the <declaration-designator>
immediately contained in y.
Perform evaluate-expression-to-integer(n) to obtain j. The value of j must be
positive and not greater than the number of <bound-pair>s simply contained in
the <data-description> immediate component of y.
Perform evaluate-variable-reference(y) to obtain a tgeneratioti,g. Let k and i
be the <lower-bound> and <upper-bound> respectively of the j'th <bound-pair> in
the <bound-pair-list> of g's <evaluated-data-description+. Return an
*aggregate-value, containing <real-value+: (i-k+l).
t********************************+*****+************************************************
DECLARE A(10,2:6) CONTROLLED;
ALLOCATE A;
N = DIM(A,Z);
The value of N is 5 after the execution of the statements shown.
Example 9.7. An Example of the DIM bif.
330
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 349

9.4.4.28 Divide-bif
Arguments: xry.pCrql
Constraints: The taggregate-type+s of the <argument>s x and y must be compatible. All
<data-type>s of x and y must have <computational-type>. Constraints for p
and q are described in Section 9.4.2.2.
Attributes: The result *aggregate-type+ is the common <aggregate-type+ of x and y. The
<base>, <scale>, and <mode> of the scalar-result-type are the derived common
<base>, <scale>, and <mode> of the corresponding <data-type>s in x and y.
The <precision> of the scalar-result-type is determined as defined in
Section 9.4.2.2.
operation: divide-bif(rdd,x,y,pC,ql)
Perform generate-aggregate-result.
Step 1,. Perform Steps 1.1 and 1.2 in either order.
Step 1.1. Convert the scalar-value of x to the derived common <base>, <scale>, and
<mode> of the corresponding <data-type% of x and y. and the converted
<precision> of the <data-type> of x.
Step 1.2. Convert the scalar-value of y to the derived common <base>, <scale>, and
<mode> of the corresponding <data-type% of x and yI and the converted
<precision> of the <data-type> of y.
step 2. If the converted scalar-value of y is zero, perform raise-
condition(<zerodivide-condition>),.
step 3.
Case 3.1. The derived <mode> of the <data-type% of x and y has <a>.
Let v be the converted scalar-value of x divided by the converted scalar-
value of y.
Case 3.2. At least one of the <mode>s of the <data-type>s of x and y has <complex>.
Perform conditions-in-arithmetic-expression( where rt is the scalar-
result-type. Let v be the converted scalar-value of x divided by the
converted scalar-value of y.
Step 4. Perform arithmetic-result(v,rt), where rt is the scalar-result-type, to obtain
the scalar-result.
Chapter 9: Expressions and Conversion 331
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 350

9.4.4.29 Dot-bif
Arguments: x.yI,pt,qll
Constraints: x and y must both be of the form <argument>: <expression>: <data-
Attributes:
Operation:
Step 1. In
description>: <dimensioned-data-description>: <element-data-description>:
<item-data-description>: <bound-pair-list>,bpl;;;;;; where bpl contains one
component. All <data-type>s of x and y must have <computational-type>. The
constraints on p and q are described in Section 9.4.2.2. If p is not
specified, then the common derived <scale> of x and y must have <float>.
The result *aggregate-types immediately contains <scalar% The result
<base>, <scale>, and <mode> are the derived common <base>, <scale>, and
<mode> of the <data-type>s of x and y. If p is not specified then the
result-type has <scale>: <float>; and the <precision> of the converted
<precision>s of x and y. Otherwise the <scale> and <precision> of the
result-type are determined as defined in Section 9.4.2.2.
dot-bif(rdd,x,yCrpl,qll)
either order. perform evaluate-expression(x) and evaluate-expression(y) to _ - obtain <aggregate-value%, u and v, each of which has a single <bound-pair> in
its <aggregate-type+. The aggregates u and v must have the same number-of-
scalar-elements, n.
Step 2. Let udt and vdt be the <data-type>s with the derived common <base>, <scale>, and
<mode> of x and y, and the converted <precision>s of x and y, respectively. In
any order, convert the scalar-elements of u to udt and the scalar-elements of v
to vdt to obtain ut[ll,utC21 ,.*.,ut[nl and vt[ll,vt[2l,...,vtCnl. Let
n
w= c ut[il * vtCi1.
i=l
Step 3. Perform conditions-in-arithmetic-expression( where rt is the <data-type>
component of rdd.
step 4. Perform arithmetic-result(w,rt), where rt is the <data-type> component of rdd,
to obtain z. Return an <aggregate-value+ containing 2.
9.4.4.30 Empty-bif
Arguments: (none)
Attributes: The result <aggregate-type, immediately contains *scalar+. The result
<data-type> has <area>.
Operation: empty-bif
Step 1. Return the <area-value+: iempty+.
332
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 351

9.4.4.31 Erf-bif
Arguments: x
Constraints: All <data-type% of x must have <computational-type>. The derived <mode> of
the <data-type>s of x must have <a>.
Attributes: The result <aggregate-type+ is the <aggregate-type> of x. The <scale> of
the scalar-result-type has <float>. The Unode> of the scalar-result-type
has __ <real>. The <base> of the scalar-result-type is the derived <base> of
the corresponding <data-type> of x. The <precision> of the scalar-resdlt-
type is the converted <precision> of the corresponding <data-type> of x.
Operation: erf-bif(rdd,x)
Perform generate-aggregate-result.
Step 1. Convert the scalar-value of x to the scalar-result-type to obtain y.
Step 2. Perform conditions-in-arithmetic-expression( where rt is the scalar-result-
type- Let w be
”
1
J -tL'
(2/3&I * e dt.
0
Step 3. Perform arithmetic-result(w,rt), where rt is the scalar-result-type, to obtain
the scalar-result.
9.4.4.32 Erfc-bif
Arguments: x
Constraints: All <data-type>s of x must have <computational-type>. The <derived <mode>
of the <data-type>s of x must have <real>.
Attributes: The result <aggregate-type* is the <aggregate-type, of x. The <scale> of
the scalar-result-type has <float>. The Unode> of the scalar-result-type
has __ <real>. The <base> of the scalar-result-type is the derived <base> of
the corresponding <data-type> of x. The <precision> of the scalar-result-
type is the converted <precision> of the corresponding <data-type> of x.
Operation: erfc-bif(rdd,x)
Perform generate-aggregate-result.
step 1. Convert the scalar-value of x to the scalar-result-type to obtain y.
Step 2. Perform conditions-in-arithmetic-expression( where rt is the scalar-result-
type- Let w be
Y
J -t'
l-(2/&) + e dt.
0
Step 3. Perform arithmetic-result(w,rt), where rt is the scalar-result-type, to obtain
the scalar-result.
Chapter 9: Expressions and Conversion 333
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 352

9.4.4.33 Every-k&f
Arguments: x
Constraints: All <data-type>s of x must have <computational-type>.
Attributes: The result <aggregate-type+ immediately contains *scalar>. The result
<data-type> has <bit>.
Operation: every-bif(rdd,x)
Step 1. Perform evaluate-expression(x) to obtain an <aggregate-value%,u.
Step 2. In any order, convert each scalar-element of u to <s>, to obtain v.
Step 3.
Case 3.1. Every scalar-element of v that does not contain <null-bit-strinq> has a
(bit-string-values with every <bit-values containing fone-bie.
Let r be <one-bit%
Case 3.2. (Otherwise).
Let r be <zero-bit,.
Step 4. Return an <aggregate-value, containing a <bit-string-value+ containing r.
9.4.4.34 Exp-bif
Arguments: x
Constraints: All <data-type>s of x must have <computational-type>.
Attributes: The result <aggregate-type+ is the <aggregate-type+ of x. The <scale> of the
scalar-result-type has <float>. The <base>, <mode>, and <precision> of the
scalar-result-type are the derived <base>, <mode>, and converted <precision>
of the corresponding <data-type> of x.
Operation: exn-bif(rdd,x)
Perform generate-aggregate-result.
Step 1. Convert the scalar-value of x to the scalar-result-type to obtain w.
Step 2. Perform conditions-in-arithmetic-expression( where rt is the scalar-result-
type- Let v be etw, where e is the base of the natural logarithm system.
Step 3. Perform arithmetic-result(v,rt) to obtain the scalar-result.
334
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 353

9.4.4.35 Fixed-bif
Arguments: x,p~.ql
Constraints: All <data-type>s of x must have <computational-type>. Constraints on p and q
are given in Section 9.4.2.2.
Attributes: The result taggregate-type+ is the <aggregate-type* of x. The <scale> of the
scalar-result-type has <fixed>. The <mode> and <base> of the scalar-result-
type are the derived <mode> and <base> of the corresponding <data-type> of
x. The <precision> of the scalar-result-type is determined as defined in
Section 9.4.2.2.
Operation: fixed-bif(rdd,x,pC,qlI
Perform generate-aggregate-result.
Step 1. Convert the scalar-value of x to the scalar-result-type to obtain v.
Step 2. Perform arithmetic-result(v,rt), where rt is the scalar-result-type, to obtain
the scalar-result.
9.4.4.36 Float-bif
Arguments: x,p
Constraints: All <data-type>s of x must have <computational-type>. Constraints on p are
described in Section 9.4.2.2.
Attributes: The result <aggregate-type+ is the <aggregate-type> of x. The <scale> of the
scalar-result-type has <float>. The <mode> and <base> of the scalar-result-
type are the derived <mode> and <base> of the corresponding <data-type> of
x. The <precision> of the scalar-result-type is determined as defined in
Section 9.4.2.2.
Operation: float-bif(rdd,x,p)
Perform generate-aggregate-result.
Step 1. Convert the scalar-value of x to the scalar-result-type to obtain v.
Step 2. Perform arithmetic-result(v,rt), where rt is the scalar-result-type, to obtain
the scalar-result.
Chapter 9: Expressions and Conversion 335
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 354

9.4.4.37 Floor-bif
Arguments: x
Constraints: The derived <mode> of the <data-type>s of x must have <real>.
Attributes: The result <aggregate-type+ is the <aggregate-type+ of x. The scalar-result-
type has <real>. The scalar-result-type has the derived <base> and <scale>
of the corresponding <data-type> of x.
Case 1. The scalar-result-type has <scale>: <float>.
The <precision> of the scalar-result-type is the converted <precision> of the
<data-type> of x.
Case 2. The scalar-result-type has <scale>: <fixed>.
Let r denote the <number-of-digits> and s denote the <scale-factor> of the
converted scalar-value of x. The <precision> of the scalar-result-type is given
by:
<number-of-digits>: min(N,max(r-s+l,l)>;
<scale-factor>: 0.
operation: floor-bif(rdd,x)
Perform generate-aggregate-result.
step 1. Convert the scalar-value of x to its derived -ode>, <base>, <scale>, and
converted <precision>.
Step 2. Perform conditions-in-arithmetic-expression(rt1. where rt is the scalar-result-
type - Let v be the greatest integer less than or equal to the converted scalar-
value of x.
step 3. Perform arithmetic-result(v,rt)r where rt is the scalar-result-type, to obtain
the scalar-result.
9.4.4.38 Hbound-bif
Arguments: x,n
Constraints: The <data-type> of n must have <computational-type>. n must have
taggregate+zype+: *scalar*. x must have the form
<variable-reference>:
<argument>: <expression>:
<value-reference>: <data-description>: <dimensioned-
data-description>.
Attributes: The result *aggregate-type, immediately contains *scalar*. The result
<data-type> is integer-type.
Operation: hbound-bif(rdd.x,n?
Step 1. Let y be the <variable-reference> simply contained in x. Let dp be the
<declaration-designator> immediately contained in y.
step 2. Perform evaluate-expression-to-integer(n) to obtain j. The value of j must be
positive and not greater than the number of (bound-pair>s simply contained in
the <data-description> immediate component of y.
Step 3. Perform evaluate-variable-reference(y) to obtain a *generation>,g. Let k be the
<upper-bound> of the j*th <bound-pair> in the <bound-pair-list> of g's
4evaluated-data-description+. Return an <aggregate-value+ containing treal-
value*: k.
336
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 355

9.4,.4.39 Hiqh-bif
Arguments: le
Constraints: The <data-type> of le must have <computational-type>. le must have
<aggregate-type+: fscalarS.
Attributes: The result 4aggregate-type+ immediately contains *scalar>. The result
<data-type> has <character>.
Operation: hiqh-bif(rdd,le)
Step 1. Perform evaluate-expression-to-integercle) to obtain n. The value of n must not
be negative.
step 2.
Case 2.1. n + 0.
Let v be a <character-string-value, of length n containing n occurrences of
the last <character-value+ in the r.esult of performing collate-bif.
Case 2.2. n = 0.
Let v be a <character-string-value+: <null-character-strinq>.
step 3. Return an *aggregate-value, containing v.
9.4.4.40 Imaq-bif
Arguments: x
Constraints: All <data-type% of x must have <computational-type>.
Attributes: The result <aggregate-type+ is the <aggregate-type, of x. The scalar-result-
type has <real>. The scalar-result-type has the derived <base>, <scale>, and
converted <precision> of the corresponding <data-type> of x.
Operation: imaq-biffrdd,x)
Perform generate-aggregate-result.
step 1. Convert the scalar-value of x to <complex> with the <base>, <scale>. and
<precision> of the scalar-result-type to obtain w. Let v be the imaginary part
of w.
step 2. Perform arithmetic-resultfv,rt), where rt is the scalar-result-type, to obtain
the scalar-result.
Chapter 9: Expressions and Conversion 337
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 356

9.4.4.41 Index-bif
Arguments: sa,ca
Constraints: The <aggregate-type% of sa and ca must be compatible. All <data-type>s of
sa and ca must have <computational-type>.
Attributes: The result <aggregate-type+ is the common <aggregate-type+ of sa and ca. The
scalar-result-type is integer-type.
Operation: index-bif(rdd,sa,ca)
Perform generate-aggregate-result.
Step 1. In either order, convert the scalar-values of sa and ca to their derived common
<string-type>. Let sb be the converted scalar-value of sa. and cb be the
converted scalar-value of ca.
step 2.
Case 2.1. cb is not a substring of sb or the length of either sb or cb is 0.
The scalar-result is 0.
Case 2.2. cb is a substring of sb.
Let j denote the position of the first <bit-value, or fcharacter-value, of
the leftmost occurrence of cb in sb. The scalar-result is j.
I *****************************++**+******~****~~**********************~********~************** The integer 3 is the result of evaluating: INDEX('ASCDADCD','CD*~
*************************+***********t************************************************ I
Example 9.8 An Example of the INDEX bif.
9.4.4.42 Lbound-bif
Arguments: x,n
Constraints: The <data-type> of n must have <computational-type>. n must have
<aggregate-type+: *scalar,. x must have the form <argument>:
<variable-reference>:
<expression>:
<value-reference>: <data-description>: <dimensioned-
data-description>.
Attributes: The result <aggregate-type+ immediately contains <scalar+. The result
<data-type> is integer-type.
Operation: lbound-bif(rdd,x,n)
Step 1. Let y be the <variable-reference> simply contained in x. Let dp be the
<declaration-designator> immediately contained in y-
Step 2. Perform evaluate-expression-to-integer(n) to obtain j. The value of j must be
positive and not greater than the number of <bound-pair>s simply contained in
the <data-description> immediate component of y.
Step 3. Perform evaluate-variable-reference(y) to obtain a fgeneration%,g. Let k be the
<lower-bound> of the j'th <bound-pair> in the <bound-pair-list> of g's
<evaluated-data-description+. Return an <aggregate-value> containing freal-
value,: k.
338
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 357

9.4.4.43 Lenqth-bif
Arguments: sa
Constraints: All <data-type% of sa must have <computational-type>.
Attributes: The result <aggregate-types is the <aggregate-type, of sa. The scalar-
result-type is integer-type.
Operation: lenqth-bif(rdd,sa)
Perform generate-aggregate-result.
step 1. Convert the scalar-value of sa to the derived <string-type> of sa to obtain sb.
Step 2. The scalar-result is the length of sb.
9.4.4..44 Lineno-bif
Arguments: fn
Constraints: fn must have *aggregate-type+: <scalar+. The <data-type> of fn must have
<file>.
Attributes: The result <aggregate-type, immediately contains *scalar+. The result
<data-type> is integer-type.
Operation: lineno-bif(rdd,fn)
Step 1. Perform evaluate-expression to obtain a <file-values,fv. The ffile-
information+,fi, designated by fv must contain <open,. The fevaluated-file-
description+ of fi must contain <print>.
Step 2. Perform evaluate-current-line(fv) to obtain an <integer-value,,iv. Return an
<aggregate-value+ containing iv.
9.4,.4.45 Loq-bif
Arguments: x
Constraints: All <data-type>s of x must have <computational-type>.
Attributes: The result <aggregate-type, is the <aggregate-type> of x. The <scale> of the
scalar-result-type has <float>. The <base>, <mode>, and <precision> of the
scalar-result-type are the derived <base>, <mode>, and converted <precision>
of the corresponding <data-type> of x.
Operation: loq-bif(rdd,x)
Perform generate-aggregate-result.
Step 1. Convert the scalar-value of x to the scalar-result-type to obtain u. If the
scalar-result-type has <mode>: <real>; then the value of u must be greater than
0. If the scalar-result-type haszde>: <complex>; then the value of u must not
equal 0.
step 2. Perform conditions-in-arithmetic-expreSsion , where rt is the scafar-result-
type- Let v be the natural logarithm of u, such that if the scalar-result-type
has <mode>: <complex>, then:
-& < imaginary part of v I pi.
Step 3. Perform arithmetic-result(v,rt)r where rt is the scalar-result-type, to oDtain
the scalar-result-
Chapter 9: Expressions and Conversion 339
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 358

9.4.4.46 LoqlO-bif
Arguments: x
Constraints: The derived <mode> of the <data-type>s of x must have <w>.
Attributes: The result *aggregate-type+ is the <aggregate-type+ of x. The scalar-result-
type has <a> and <float>- The <base> and <precision> of the scalar-
result-type are the derived <base> and converted <precision> of the
corresponding <data-type> of x,.
Operation: loqlO-bif(rdd,x)
Perform generate-aggregate-result.
Step 1. Convert the scalar-value of x to the scalar-result-type to obtain u. The value
of u must be greater than 0.
Step 2. Perform conditions-in-arithmetic-expression( where rt is the scalar-result-
type. Let v be the common logarithm, i.e. base 10, of u.
Step 3. Perform arithmetic-result(v,rt), where rt is the scalar-result-type, to obtain
the scalar-result.
9.4.4.47 LoqZ-bif
Arguments:
Constraints:
Attributes:
Operation:
X
The derived <mode> of the <data-type>s of x must have <real>.
The result *aggregate-type+ is the <aggregate-type+ of x. The scalar-result-
type has <m> and <float>. The <base> and <precision> of the scalar-
result-type are the derived <base> and converted <precision> of the
corresponding <data-type> of x.
loq2-bif(rdd,x)
Perform generate-aggregate-result.
Step 1. Convert the scalar-value of x to the scalar-result-type to obtain u. The value
of u must be greater than 0.
Step 2. Perform conditions-in-arithmetic-expression( where rt is the scalar-result-
type - Let v be the base 2 logarithm of u.
Step 3. Perform arithmetic-result(v,rt), where rt is the scalar-result-type, to obtain
the scalar-result.
340
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 359

9.4.4.48 Low-bif
Arguments: le
Constraints: The <data-type> of le must have <computational-type>. le must have
<aggregate-type+: <scalar,.
Attributes: The result <aggregate-type+ immediately contains <scalar+. The result
<data-type> has <character>.
Operation: low-bif(rdd,le)
Step 1. Perform evaluate-expression-to-integertle) to obtain n. The value of n must not
be negative.
Step 2.
Case 2.1. n + 0.
Let v be a <character-string-value> of length n containing n occurrences of
the first l character-value, in the result ot performing collate-bif.
Case 2.2. n = 0.
Let v be a <character-string-value,: <null-character-strinq;+.
Step 3. Return an *aggregate-value, containing v.
9.4.4.49 Max-bif
Arguments: xCll,xC21r....x[nl
Constraints: n. the number of arguments, must be at least 1. The <aggregate-type>s of
the <argument>s must be compatible. The derived common (mode> of the <data-
type>s of the <argument>s must have <real>.
Attributes: The result <aggregate-type+ is the common <aggregate-type, of the
<argument>s. The <base> and <scale> of the scalar-result-type are the
derived common <base> and <scale> of the corresponding <data-type% of the
<argument>s, and its <mode> is <real>.
Case 1. The derived common <scale> of the corresponding <data-type>s of the <argument%
has <float>.
The <precision> of the scalar-result-type is max(ptll,p~2l,...,p~nl1, where
p[ll,p[2l,...,pCnl are the converted <number-of-digits> of the <data-type>s of
xtll,xl2l,...,xlnl.
Case 2. The derived common <scale> of the corresponding <data-type% of the <argument>s
has <fixed>.
Let (p111,q[111, (pC21,q1211,..., (p[nl,q[nll be the converted <number-of-digits>
and <scale-factor> of the <data-type>s of xIll,x123,...,xLnl respectively. Then
the <precision> of the scalar-result-type is given by:
<number-of-digits>:min(N,max(pCll-qC13rp~2l-qC21r . . ..pCnl-qCnl)+max(q[Ilrq[2lr..-rq[nl)).
<scale-factor>: max(qC1l,ql2l,...,qlnl1.
Operation: max-bif(rdd,xfll,xl2l,...rx[nl)
Perform generate-aggregate-result.
Step 1. In any order, convert the scalar-values of xCll,...,xCnl to the Scalar-result-
type. Let the converted scalar-values be uC13,u12l,...,uCnl.
Step 2. Let v be max(uClJ,uC21,...ruCnl).
Step 3. Perform arithmetic-result(v,rt)r where rt is the scalar-result-type, to obtain
the scalar-result.
Chapter 9: Expressions and Conversion 341
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 360

9.4.4.50 Min-bif
Arguments: xc1l,x[21 ,---, xinl
Constraints: n, the number of arguments, must be at least 1. The <aggregate-type% of
the <argument>s must be compatible. The derived common <mode> of the <data-
type>s of the <argument> must have <a>.
Attributes: The result <aggregate-type+ is the common <aggregate-type, of the
<argument>s. The <base> and <scale> of the scalar-result-type are the
derived common <base> and <scale> of the corresponding <data-type% of the
<argument>s, and its <mode> is <real>.
Case 1. The derived common <scale> of corresponding <data-type>s of the <argument% has
<float>.
The <precision> of the scalar-result-type is max(plll,p~2l,...,p~nl), where
p[ll,pC21 ,...,p[nl are the converted <number-of-digits> of the <data-type>s of
x~1l,xI2l,...,x[n3.
Case 2. The derived common <scale> of corresponding <data-type% of the <argument% has
<fixed>.
Let (p[11,qc11),(pc21,q~21~,..., (pCnJ,qlnl) be the converted <number-of-diqits>
and <scale-factor> of the <data-type>s of x113,x~2l,...,x[nl respectively. Then
the <precision> of the scalar-result-type is given by:
(number-of-digits>:min(N,max(plll-q[U,p[2l-q[2] ,...,ptnJ-q[nl)+max(q[l3rq[2l....rq[nl));
<scale-factor>: max(q~ll,q[2l.--~~q~nl~-
Operation: min-bif(rdd,xCll,x[23r...,xCnl)
Perform generate-aggregate-result.
Step 1. In any order, convert the scalar-values of xl11 ,....xlnl to the scalar-result-
type- Let the converted scalar-values be uCll,u~2l,...,u[nl.
Step 2. Let v be min(u~11,u121r...,u~nl~.
Step 3. Perform arithmetic-result(v,rt). where rt is the scalar-result-type, to Obtain
the scalar-result.
342
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 361

9,.4.4.51 Mod-bif
Arguments: x,y
Constraints: The <aggregate-typess of x and y must be compatible. All derived <data-
type% of x and y must have <real>.
Attributes: The result <aggregate-type+ is the common iaggregate-type* of x and y. The
<base> and <scale> of the scalar-result-type are the derived common <base>
and <scale> of the corresponding <data-type>s of x and y, and its <mode> is
<real>.
Case 1. The <scale> of the scalar-result-type has <float>.
The <precision> of the scalar-result-type is the greater of the converted
<precision% of the corresponding <data-type>s of x and y-
Case 2. The <scale> of the scalar-result-type has <fixed>.
Let (pCll,q[ll) and (pC2l,qt23) be the respective converted <number-of-digits>
and <scale-factor> of the corresponding <data-type% of x and y. Then the
<precision> of the scalar-result-type is given by:
<number-of-digits>: min(N,p[2l-qC23+max(q[13r9[23));
<scale-factor>: max(qCll,q~21).
Operation: mod-bif(rdd,x,y)
Perform generate-aggregate-result.
Step 1. Perform Steps 1.1 and 1.2 in either order.
Step 1.1. Convert the scalar-value of x to the derived common <base>, <scale>, and
<mode> of the corresponding <data-type>s of x and yl and the converted
eprecisior-0 of the <data-type> of x.
Step 1.2. Convert the scalar-value of y to the derived common <base>, <scale>, and
<mode> of the corresponding <data-type>s of x and y. and the converted
<precision> of the <data-type> of y.
Step 2. Let the converted scalar-value of x be u and the converted scalar-value of y be
W.
Case 2.1. w + 0.
Perform conditions-in-arithmetic-expression( where rt is the scalar-
result-type. Let v be given by:
v=u-w*floor(u/w).
Case 2.2. w = 0.
Let v=u-
Step 3. Perform arithmetic-result(v,rt), where rt is the scalar-result-type, to obtain
the scalar-result.
Chapter 9: Expressions and Conversion 343
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 362

9.4.4.52 Multiply-bif
Arguments: xry,pC,ql
Constraints: The *aggregate-type* of x and y must be compatible. All <data-types of x
and y must have <computational-type>. Constraints on p and q are given in
Section 9.4.2.2,.
Attributes: The result <aggregate-type+ is the common *aggregate-type, of x and y. The
<base>, <scale>, and <mode> of the scalar-result-type are the derived common
<base>, <scale>, and <mode> of the corresponding <data-type>s of x and y.
The <precision> of the scalar-result-type is determined as defined in
Section 9.4.2.2.
Operation: multiply-bif(rdd,x,y,pC,ql)
Perform generate-aggregate-result.
Step 1. Perform Steps 1.1 and 1.2 in either order.
step 1.1. Convert the scalar-value of x to the derived common <base>, <scale>, and
<mode> of the corresponding <data-type>s of x and ye and the converted
<precision> of the <data-type> of x.
Step 1.2. Convert the scalar-value of y to the derived common <base>, <scale>, and
<mode> of the corresponding <data-type>s of x and yl and the converted
<precision> of the <data-type> of y.
Step 2.
Case 2.1. The scalar-result-type has <real>.
Let v be the product of the converted scalar-value of x and the converted
scalar-value of y.
Case 2.2. The scalar-result-type has <complex>.
Perform conditions-in-arithmetic-expression( where rt is the scalar-
result-type. Let v be the product of the converted scalar-value of x and
the converted scalar-value of y.
Step 3. Perform arithmetic-result(v,rt), where rt is the scalar-result-type, to obtain
the scalar-result.
344
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 363

9.4.4.53 Null-bif
Arguments: (none)
Attributes: The result <aggregate-type+ immediately contains fscalar,. The result
<data-type> has <pointer>.
Operation: null-bif
Step 1. Return an <aggregate-value, containing <pointer-value+: fnull+.
9.4.4.54 Offset-bif
Arguments: W,ar
Constraints: All <data-type>s of pt must have <pointer>. ar must have <aggregate-type+:
<scalar*. The <data-type> of ar must have <area>. ar must have the form
<argument>: <expression>: <value-reference>: <variable-reference>.
Attributes: The result <aggregate-type* is the <aggregate-type+ of pt. The scalar-
result-type has <offset> with no subnode.
Operation: offset-bif(rdd,pt,ar)
Perform generate-aggregate-result.
Step 1. Let odt be a <data-type> containing <offset>: vr; where vr is the <variable-
reference> in ar.
Step 2. Convert the scalar-value of pt to <data-type>,odt to obtain the scalar-result.
9.4.4.55 Onchar-bif
Arguments: (none)
Attributes: The result *aggregate-type+ immediately contains <scalar>. The result
<data-type> has <character>.
Operation: onchar-bif
Step 1. Perform get-established-onvalue(fonchar-value,) to obtain j.
Case 1.1. j is *fail+ or j=O.
Let v be a <character-value+ containing b.
Case 1.2. j > 0.
Perform get-established-onvalue(fonsource-value,) to obtain a fcharacter-
string-value%,sa. Let v be the j'th <character-value+ in sa.
Step 2. Return an <aggregate-value, containing <character-string-value>: Echaracter-
value-lists: v.
Chapter 9: Expressions and Conversion 345
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 364

9.4.4.56 Oncode-bif
Arguments: (none)
Attributes: The result <aggregate-type* immediately contains <scalar+. The result
<data-type> is integer-type.
Operation: oncode-bif
Step 1. Perform get-established-onvalue(foncode-value,) to obtain j.
Case 1.1. j is ts+.
Let v be 0.
Case 1.2. (Otherwise).
Let v be j, an implementation-defined value.
Step 2. Return an <aggregate-values containing *real-value,: v.
9.4.4.57 Onfield-bif
Arguments: (none)
Attributes: The result <aggregate-type+ immediately contains <scalars. The result
<data-type> has <character>.
Operation: onfield-bif
Step 1. Perform get-established-onvalue(fonfield-value,) to obtain sa.
Case 1.1. sa is *fail,.
Let v be a <character-string-value+: fnull-character-strinq>-
Case 1.2. (Otherwise).
Let v be sa.
Step 2. Return an <aggregate-value+ containing v.
9.4.4.58 Onfile-bif
Arguments: (none)
Attributes: The result <aggregate-type* immediately contains 4scalar>. The result
<data-type> has <character>.
Operation: onfile-bif
Step 1. Perform get-established-onvalue(fonfile-value,) to obtain sa.
Case 1.1. sa is *fails.
Let v be a <character-string-value,: fnull-character-strinq>.
Case 1.2. (Otherwise).
Let v be sa.
Step 2. Return an <aggregate-value+ Containing V.
346
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 365

9.4.4.59 Onkey-bif
Arguments: (none)
Attributes: The result <aggregate-type+ immediately contains <scalar+. The result
<data-type> has <character>.
Operation: onkey-bif
Step 1. Perform get-established-onvalue(fonkey-value+) to obtain sa.
case 1.1. sa is ffai.3).
Let v be a *character-string-value+: <null-character-strinq*.
case 1.2. (Otherwise).
Let v be sa.
Step 2. Return an <aggregate-value, containing v.
9.4.4.60 Onloc-bif
Arguments: (none)
Attributes: The result <aggregate-type+ immediately contains tscalarS. The result
<data-type> has <character>.
Operation: onloc-bif
step 1. Perform get-established-onvalue(fonloc-value,) to obtain sa.
Case 1.1. sa is <fail+,.
Let v be a <character-string-value%: <null-character-strinqs.
Case 1.2. (Otherwise).
Let v be sa.
step 2. Return an <aggregate-value+ containing v.
9.4.4.61 Onsource-bif
Arguments: (none)
Attributes: The result <aggregate-type+ immediately contains tscalarS. The result
<data-type> has <character>.
Operation: onsource-bif
Step 1. Perform get-established-onvalue(fonsource-values) to obtain sa.
Case 1.1. sa is *fail*.
Let v be a fcharacter-string-value+: <null-character-strinqs.
Case 1.2. (Otherwise).
Let v be sa.
step 2,. Return an *aggregate-value+ containing v.
Chapter 9: Expressions and Conversion 347
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 366

9.4.4.62 Pageno-bif
Arguments: fn
Constraints: fn must have <aggregate-type+: &scalar,. The <data-type> of fn must have
<file>.
Attributes: The result <aggregate-type+ immediately contains fscalar,. The result
<data-type> is integer-type.
Operation: paseno-bif(rdd,fn)
step 1. Perform evaluate-expression to obtain a <file-value,,fv. The ffile-
information+,fi, designated by fv must contain t-3. The fevaluated-file-
description3 of fi must contain <print>.
step 2. Let v be the <integer-value+ in <page-number+ in fi.
step 3. Return an <aggregate-value% containing <real-value>: v.
9.4.4.63 Pointer-bif
Arguments: ofe.ar
Constraints: All <data-type>s of ofe must have <offset>. ar must have <aggregate-type+:
<scalar%. The <data-type> of ar must have <area>. ar must have the form
<argument>: <expression>: <value-reference>: <variable-reference>.
Attributes: The result <aggregate-type, is the <aggregate-type> of ofe. The scalar-
result-type has <pointer>.
Operation: pointer-bif(rdd,ofe,ar)
Perform generate-aggregate-result.
Step 1. Perform Steps 1.1 and 1.2 in either order.
step 1.1. Perform evaluate-expression(ofe) to obtain ofs.
Step 1.2. Let odt be a <data-type> containing <offset>: vr; where vr is the <variable-
reference> in ar.
Step 2. Perform convert(pdt,odt,ofs) to obtain the scalar-result, where pdt is a <data-
type> containing <pointer>.
9.4.4.64 Precision-bif
Arguments: xrp[.ql
Constraints: Each <data-type> of x must have <computational-type>. Constraints on p and q
are given in Section 9.4.2.2.
Attributes: The result <aggregate-type+ is the <aggregate-type+ of x. The <base>,
<scale>, and <mode> of the scalar-result-type are the derived <base>,
<scale>, and <mode> of the corresponding <data-type> of x. The <precision>
of the scalar-result-type is determined as defined in Section 9.4.2.2.
Operation: precision-bif(rdd,x,p[rql)
Perform generate-aggregate-result.
step 1. Convert the scalar-value of x to the scalar-result-type to obtain v.
step 2,. Perform arithmetic-result(v,rt)r where rt is the scalar-result-type, to obtain
the scalar-result.
340
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 367

9.4..4.65 Prod-bif
Arguments: x
Constraints: x must be of the form <argument>: <expression>: <data-description>:
<dimensioned-data-description>: <element-data-description>: Citem-data-
description>. The <data-type> of x must have <computational-type>.
Attributes: The result <aggregate-type+ immediately contains iscalars. The (base> and
<mode> of the result <data-type> are the derived <base> and <mode> of the
<data-type> of x.
Case 1. The derived <scale> of the <data-type> of x has <fixed> and the converted
<scale-factor> has 0.
The result <data-type> has <fixed> and <number-of-digits>: N. The result <data-
type> has <scale-factor>: 0.
Case 2. (Otherwise).
The result <data-type> has <float> and its <number-of-digits> is the converted
<number-of-digits> of the <data-type> of x.
Operation: prod-bif(rdd,x)
Step 1. Perform evaluate-expression(x) to obtain an <aggregate-value*,v.
Step 2. Convert the scalar-elements of v to the result <data-type> in any order. Let w
be the product of the converted values.
Step 3. Perform conditions-in-arithmetic-expressionIrt). where rt is the <data-type>
component of rdd.
Step 4. Perform arithmetic-result(w,rt)r where rt is the <data-type> of rdd, to obtain
Z. Return an <aggregate-value+ containing z.
Chapter 9: Expressions and Conversion 349
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 368

9.4.4.66 Real-bif
Arguments: x
Constraints: Each <data-type> of x must have <computational-type>.
Attributes: The result <aggregate-type, is the <aggregate-type* of x. The scalar-result-
type has <real>. The <base> and <scale> of the scalar-result-type are the
derived <base> and <scale> of the corresponding <data-type> of x. The
<precision> of the scalar-result-type is the converted <precision> of the
corresponding <data-type> of x.
Operation: real-bif(rad,x)
Perform generate-aggregate-result.
Step 1. Convert the scalar-value of x to <complex> with the <base>, <scale>, and
<precision> of the scalar-result-type.
Step 2. Let v be the real part of the converted scalar-value of x.
Step 3. Perform arithmetic-result(v,rt), where rt is the scalar-result-type, to obtain
the scalar-result.
9.4.4.67 Reverse-bif
Arguments: sa
Constraints: Each <data-type> of sa must have <computational-type>.
Attributes: The result <aggregate-type+ is the <aggregate-type+ of sa. The scalar-
result-type has the derived <string-type> of the corresponding <data-type>
of sa.
Operation: reverse-bif (raa,sa)
Perform generate-aggregate-result.
step 1. Convert the scalar-value of sa to the scalar-result-type to obtain sb.
Step 2.
Case 2.1. sb is a null-string.
The scalar-result is a null-string.
Case 2.2. sb is not a null-string.
Let m be the length of sb. The scalar-result is a string of length m whose
i'th <bit-value+ or <character-value+ is the (m-i+l)'th <bit-value+ or
<character-value, in sb.
350
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 369

9.4.4.68 Round-bif
Arguments: x,n
Constraints: All <data-type>s of x must have <computational-type>. n must have <constant>
and integer-type. If the derived <scale> of any <data-type> of x has
<float>, the value of n must be greater than zero.
Attributes: The result <aggregate-type+ is the <aggregate-type> of x. The <base>,
<scale>, and <mode> of the scalar-result-type are the derived <base>,
<scale>, and <mode> of the corresponding <data-type> of x.
Case 1. The derived <scale> and <mode> of the <data-type> of x have <a> and <fixed>.
Let r and s be the converted <number-of-digits> and <scale-factor> of the
corresponding <data-type> of x.
The <precision> of the scalar-result-type is given by:
<number-of-digits>: max(l,min(r-s+l+n,N));
<scale-factor>: n.
Case 2. The derived <scale> and <mode> of the <data-type> of x are <real> and <float>.
The <number-of-digits> in the scalar result-type is given by min(n.N).
Case 3. The derived <mode> of the <data-type> of x has <complex>.
The <precision> of the scalar-result-type is determined from the converted
<precision> of the real part of the scalar-value of x by either Case 1 or Case
2.
Operation: round-bif(rdd.x,n)
Perform generate-aggregate-result.
Step 1. Convert the scalar-value of x to its derived <mode>, <base>, <scale>, and
converted <precision>. Let v be the converted value.
Step 2. Let b be the numerical base of the scalar-result.
Case 2.1. The <scale> and <mode> of the <data-type> of v have <real> and <fixed>.
Perform conditions-in-arithmetic-expression( where rt is the scalar-
result-type- Let w be given by:
w=sign(v)*(bt-n)*floor(abs(v)*(b,n)+l/Z).
Case 2.2. The <scale> and <mode> of the <data-type> of v have <real> and <float>.
Let c be the unique integer such that
bt(c-1) I abs(v) < btc.
Perform conditions-in-arithmetic-expression( where rt is the scalar-
result-type. Let w be an implementation-defined value that satisfies the
inequality:
abs(w-v) 5 (bt(c-n))/Z.
Case 2.3. The <mode> of the <data-type> of v has <complex>.
Let u be the real part of v and z be the imaginary part of v. Obtain w by
applying Case 2.1 to u and then to z, if the <scale> of v has <fixed>;
otherwise obtain the value w by applying Case 2.2 to u and then to z.
Step 3. Perform arithmetic-result(w,rt), where rt is the scalar-result-type, to obtain
the scalar-result.
Chapter 9: Expressions and Conversion 351
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 370

9.4.4.69 Sian-bif
Arguments: x
Constraints: Each <data-type> of x must have <computational-type>; the derived Unode7
must not have <complex>.
Attributes: The result <aggregate-type, is the <aggregate-type+ of x. The scalar-
result-type is integer-type.
Operation: siqn-bif(rdd,x)
Perform generate-aggregate-result.
Step 1. Convert the scalar-value of x to its derived Unode>, <scale>, <base>, and
converted <precision> to obtain v.
step 2.
Case 2.1. v>O.
The scalar-result is <real-value>: 1.
Case 2.2. v=O.
The scalar-result is *real-value7: 0.
Case 2.3. HO.
The scalar-result is treal-value,: -1.
9.4.4.70 Sin-bif
Arguments: x
Constraints: All <data-type>s of x must have <computational-type>.
Attributes: The result *aggregate-type> is the iaggregate-type+ of x. The <scale> of the
scalar-result-type has <float>. The <base>, <mode>, and <precision7 of the
scalar-result-type are the derived <base>, <mode>, and converted <precision7
of the corresponding <data-type7 of x-
Operation: sin-bif(rdd,x)
Perform generate-aggregate-result.
step 1. Convert the scalar-value of x to the scalar-result-type. The scalar-value of x
is assumed to be given in radians.
step 2. Perform conditions-in-arithmetic-expression( where rt is the scalar-result-
type- Let v be the sine of the converted scalar-value of x.
step 3. Perform arithmetic-resulttv,rt), where rt is the scalar-result-type, to obtain
the scalar-result.
352
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 371

9.4.4.71 Sin+bif
Arguments: x
Constraints: Each <data-type> of x must have <computational-type>; the derived <mode>
must not have <complex>.
Attributes: The result <aggregate-type% is the <aggregate-type> of x. The <scale> of
the scalar-result-type has <float>. The <mode> of the scalar-result-type
has __ <real>. The <base> and <precision> of the scalar-result-type are the
derived <base> and converted <precision> of the corresponding <data-type> of
X.
Operation: sind-bif(rdd,x)
Perform generate-aggregate-result.
Step 1. Convert the scalar-value of x to the scalar-result-type. The value of x is
assumed to be given in degrees.
Step 2. Perform conditions-in-arithmetic-expression( where rt is the scalar-result-
type- Let v be the sine of the converted scalar-value of x.
Step 3. Perform arithmetic-result(v.rt), where rt is the scalar-result-type, to obtain
the scalar-result.
9.4.4.72 Sinh-bif
Arguments: x
Constraints: All <data-type% of x must have <computational-type>-
Attributes: The result *aggregate-type+ is the iaggregate-type of x. The <scale> of
the scalar-result-type has <float>. The <base>, <mode>, and <precision> of
the scalar-result-type are the derived <base>, <mode>, and converted
<precision> of the corresponding <data-type> of x.
Operation: sinh-bif(rdd,x)
Perform generate-aggregate-result.
Step 1. Convert the scalar-value of x to the scalar-result-type to obtain w..
Step 2. Perform conditions-in-arithmetic-expression( where rt is the scalar-result-
type - Let v be the hyperbolic sine of w.
Step 3. Perform arithmetic-resultIv,rt) to obtain the scalar-result-
Chapter 9: Expressions and Conversion 353
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 372

9.4.4.73 Some-bif
Arguments: x
Constraints: All <data-type>s of x must have <computational-type>.
Attributes: The result <aggregate-type% immediately contains <scalar+. The result
<data-type> has <bit>.
Operation: some-bif(rdd,x)
Step 1. Perform evaluate-expression(x) to obtain an <aggregate-values,u.
Step 2. In any order, convert each scalar-element of u to <bit>.
Step 3.
Case 3.1. At least one <bit-value, in some converted scalar-element has <one-bit%.
Let r be <one-bit+.
Case 3.2. (Otherwise).
Let r be <zero-bit%.
Step 4. Return an <aggregate-values containing a <bit-string-value> containing r.
9.4.4.74 Sqrt-bif
Arguments: x
Constraints: All <data-type% of x must have <computational-type>.
Attributes: The result <aggregate-type+ is the <aggregate-type> of x. The <scale> of the
scalar-result-type has <float>. The <base>, (mode>, and <precision> of the
scalar-result-type are the derived <base>, <mode>, and converted <precision>
of the corresponding <data-type> of x.
Operation: sqrt-bif(rdd,x)
Perform generate-aggregate-result.
Step 1. Convert the scalar-value of x to the scalar-result-type to obtain y.
Step 2.
Case 2.1. The <data-type> of y has <real>.
The value of y must not be negative. Perform conditions-in-arithmetic-
expression(rt1, where rt is the scalar-result-type. Let w be the positive
square root of y.
Case 2.2. The <data-type> of y has <complex>.
Perform conditions-in-arithmetic-expression( where rt is the scalar-
result-type. Let w be that square root of y satisfying the following: If
the real part of w is u and its imaginary part is v, then either u is
greater than zero, or u is zero and v is non-negative.
Step 3. Perform arithmetic-result(w,rt), where rt is the scalar-result-type, to obtain
the scalar-result.
354
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 373

9.4.4.75 String-bif
Arguments: sa
Constraints: All <data-type% of sa must have <computational-type>.
Attributes: The result *aggregate-type, immediately contains <scalar+. The result
<data-type> has the derived common <string-type> of the <data-type>s of sa.
Operation: strinq-bif(rdd,sa)
Step 1. Perform evaluate-expression(sa1 to obtain an <aggregate-value+,v.
Step 2. In any order, convert all the scalar-elements in v to the derived common
<string-type> of all the <data-type>s of sa. Let the converted values be cvCi1
in the order of the scalar-elements of v.
Step 3. Let r be a null-string. For i=l to number-of-scalar-elements of v, perform Step
3.1.
step 3.1. Perform concatenate(r,cvCil) to obtain r.
Step 4. Return an <aggregate-value, containing r.
9.4..4.76 Substr-bif
Arguments: sa,sti,lel
Constraints: All <data-type>s of sa, st and le must have <computational-type>. The
<aggregate-type% of sa, st and le must be compatible.
Attributes: The result <aggregate-type+ is the common <aggregate-type> of sa, st and le.
Each scalar-result-type has <string> with the derived <string-type> of the
corresponding <data-type> of sa.
Operation: substr-bif<rdd,sa,stC,lel)
Perform generate-aggregate-result.
step 1. Perform Steps 1.1 to 1.3 in any order.
Step 1.1. Convert the scalar-value of sa to the scalar-result-type to obtain sb. Let
the length of sb be k.
Step 1.2. Convert the scalar-value of st to integer-type to obtain i.
Step 1.3. If le is not <absent,, convert its scalar-value to integer-type to obtain j.
step 2. If le is fabsent,, let j=k-i+l..
step 3. Test the inequalities:
llisk+l
Oljlk-i+l.
If these inequalities are
condition(<strinqranqe-condition>).
not satisfied, perform raise-
step 4.
Case 4.1. j = 0.
The scalar-result is a null-string.
Case 4.2. j > 0.
The scalar-result is a string of length j whose n'th <bit-value+ or
*character-value, (llnlj) is the (i+n-1) 'th *bit-value, or *character-value,
in the string sbr n=l,...,j.
Chapter 9: Expressions and Conversion 355
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 374

9.4.4.77 Subtract-bif
Arguments: x.yrpC,ql
Constraints: The <aggregate-type% of x and y must be compatible. Each <data-type> of x
and y must have <computational-type>. Constraints on p and q are described
in Section 9.4.2.2.
Attributes: The result <aggregate-type+ is the common <aggregate-type> of x and y. The
<base>, <scale>, and <mode> of the scalar-result-type are the derived common
<base>, <scale>, and <mode> of the corresponding <data-type% of x and y.
The <precision> of the scalar-result-type is determined as defined in
Section 9.4.2.2.
Operation: subtract-bif(rdd,x,y,pC,qll
Perform generate-aggregate-result.
Step 1. Perform Steps 1.1 and 1.2 in either order.
Step 1.1. Convert the scalar-value of x to the derived common <base>, <scale>, and
<mode> of the corresponding <data-type>s of x and y, and the converted
<precision> of the <data-type> of x.
Step 1.2. Convert the scalar-value of y to the derived common <base>, <scale>, and
<mode> of the corresponding <data-type% of x and yI and the converted
<precision> of the <data-type> of y.
Step 2. Let v be the difference between the converted scalar-value of x and the
converted scalar-value of y.
Step 3. Perform arithmetic-result(v,rt), where rt is the scalar-result-type, to obtain
the scalar-result.
9.4.4.78 Sum-bif
Arguments: x
Constraints: x must be of the form <argument>: <expression>: <data-description>:
<dimensioned-data-description>: <element-data-description>: <item-data-
description>. Its <data-type> must have <computational-type>.
Attributes: The result <aggregate-type> immediately contains <scalar+. The <base>,
<scale>, and <mode> of the result <data-type> are the derived <base>,
<scale>, and <mode> of the <data-type> of x.
Case 1. The derived <scale> of the <data-type> of x has <fixed>.
Let r be the converted <scale-factor> of the <data-type> of x. Then the
<precision> of the result <data-type> has <number-of-digits>: N: and <scale-
factor>: r.
Case 2. The derived <scale> of the <data-type> of x has <float>.
The <precision> of the result <data-type> has the converted <precision> of the
<data-type> of x.
Operation: sum-bif(rdd,xl
Step 1. Perform evaluate-expression(x) to obtain an <aggregate-value,,v.
Step 2. In any order, convert the scalar-elements of v to the result <data-type>. Let w
be the sum of the converted values.
Step 3. Perform conditions-in-arithmetic-expression(rt1, where rt is the <data-type>
component of rdd.
step 4. Perf arm arithmetic-result(w,rt.), where rt is the <data-type> component of rdd.
to obtain z. Return an <aggregate-value> containing z.
356
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 375

9.4.4.79 Tan-bif
Arguments: x
Constraints: All <data-type% of x must have <computational-type>.
Attributes: The result <aggregate-type+ is the <aggregate-type, of x. The <scale> of the
scalar-result-type has <float>. The <base>, -ode>, and <precision> of the
scalar-result-type are the derived <base>, <mode>, and converted <precision>
of the corresponding <data-type> of x.
operation: tan-bif(rdd,x)
Step 1.
Step 2.
Step 3.
Perform generate-aggregate-result.
Convert the scalar-value of x to the scalar-result-type. Its value is assumed to
be given in radians, and must not be an odd multiple of e/2.
Perform conditions-in-arithmetic-expression( where rt is the scalar-result-
type - Let v be the tangent of the converted scalar-value of x.
Perform arithmetic-result(v,rt), where rt is the scalar-result-type, to obtain
the scalar-result.
g-4-4.80 Tand-bif
Arguments: x
Constraints: Each <data-type> of x must have <computational-type>; the derived <mode>
must not have <complex>.
Attributes: The result <aggregate-type, is the <aggregate-type+ of x. The <scale> of the
scalar-result-type has <float>. The Unode> of the scalar-result-type has
<real>. The <base> and <precision> of the scalar-result-type are the
dzed <base> and converted <precision> of the corresponding <data-type> of
X.
Operation: tand-bif(rdd,x)
Perform generate-aggregate-result.
Step 1. Convert the scalar-value of x to the scalar-result-type. The converted scalar-
value is assumed to be given in degrees, and must not be an odd multiple of 90
degrees.
step 2. Perform conditions-in-arithmetic-expression( where rt is the scalar-result-
type- Let v be the tangent of the converted scalar-value of x.
Step 3. Perform arithmetic-result(v,rt), where rt is the scalar-result-type, to obtain
the scalar-result.
Chapter 9: Expressions and Conversion 357
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 376

9.4.4.81 Tanh-bif
Arguments: x
Constraints: All <data-type>s of x must have <computational-type>.
Attributes: The result <aggregate-type+ is the <aggregate-type> of x. The <scale> of
the scalar-result-type has <float>. The <base>, <mode>, and <precision> of
the scalar-result-type are the derived <base>, <mode>, and converted
<precision> of the corresponding <data-type> of x.
Operation: tanh-bif(rdd,x>
Perform generate-aggregate-result.
Step 1. Convert the scalar-value of x to the scalar-result-type to obtain w.
Step 2. Perform conditions-in-arithmetic-expression( where rt is the scalar-result-
type- Let v be the hyperbolic tangent of w.
Step 3. Perform arithmetic-result(v,rt) to obtain the scalar-result.
9.4.4.82 Time-bif
Arguments: (none)
Attributes: The result <aggregate-type+ immediately contains <scalar,. The result
<data-type> has <character>.
Operation: time-bif
Step 1. Let v be a icharacter-string-value+ of length 6+t, where t is an implementation-
defined non-negative integer. v represents the time-of-day on a 24-hour scale,
in the form hhmmssld... 1, where each letter represents one *character-value+ and
each contained {symbol) is a $digitI+. hh,mm,ss are respectively in the ranges
OC:23, 00:59, and 00: 59, representing hours, minutes, and seconds. d...
represents decimal fractions of a second.
Step 2. Return an <aggregate-values containing v.
358
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 377

9.4.4.83 Translate-bif
Arguments: sa,raC,pal
Constraints: The *aggregate-type% of sa, ra and pa must be compatible. All <data-type>s
of sa, ra and pa must have <computational-type>.
Attributes: The result <aggregate-type, is the common *aggregate-type+ of sa, ra and pa.
The scalar-result-type has <character>.
Operation: translate-bif(rdd,sa,raC,pall
Perform generate-aggregate-result.
Step 1. Perform steps 1.1 and 1.2 in either order.
Step 1.1. Convert the scalar-value of sa to <character> to obtain sb. Let m be the
length of sb.
Step 1.2.
Case l-2,.1. pa is not <absent>.
Convert the scalar-value of pa to <character> to obtain pb.
Case 1.2.2. pa is fabsent,.
Perform collate-bif to obtain pb.
Step 2. Convert the scalar-value of ra to <character>. If the length of this converted
scalar-value is less than the length of pb, then convert it to a <character>
string of the length of pb. Let rb denote the converted scalar-value of ra.
Step 3.
Case 3.1. The string sb is a null-string.
The scalar-result is a null-string.
Case 3.2. The string sb is not a null-string.
For each i, llilm, let stil be the i'th <character-value> in sb and let t[il
be the i'th fcharacter-value, in the scalar-result. The string pb is
searched for the leftmost occurrence of siil. If sCi1 is not found in pb,
then let tCi1 be sCi1. If sCi1 occurs in pb, let j be the ordinal of the
occurrence in pb and let tCi1 be the j'th <character-value+ in the string
rb. The scalar-result is t.
+*******************+**++++*****+*****+**************************************************
The result of evaluating:
TRANSLATE( '1 2'r 'O‘, ' * )
is the <character-string-value+ '102'.
***+**+******+******++**+*+**+*+***+**+*************************************************~
Example 9.9. An Ehcample of the TRANSLATE bif.
Chapter 9: Expressions and Conversion 359
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 378

9.4.4.84 Trunc-bif
Arguments: x
Constraints: The derived <mode> of the <data-type> of x must have <&>.
Attributes: The result <aggregate-type+ is the <aggregate-type, of x. The <base> and
<scale> of the scalar-result-type are the derived <base> and <scale> of the
corresponding <data-type> of x.
Case 1. The derived <scale> of the corresponding <data-type> of x has <fixed>.
Let r be the converted <number-of-digits> and s be the converted <scale-factor>
of the <data-type> of x. Then the <precision> of the scalar-result-type is given
by:
-umber-of-digits>: min(N,max(r-s+l,l));
<scale-factor>: 0.
Case 2. The derived <scale> of the corresponding <data-type> of x has <float>.
The <precision> of the scalar-result-type is the converted <precision> of the
corresponding <data-type> of x.
Operation: trunc-bif(rdd,x)
Perform generate-aggregate-result.
Step 1. Convert the scalar-value of x to the scalar-result-type to obtain v- Perform
conditions-in-arithmetic-expression( where rt is the scalar-result-type.
step 2.
Case 2.1. v < 0.
Let w=ceil(v).
Case 2.2. v = 0.
Let r-0.
Case 2.3. v > 0.
Let w=floor(v).
Step 3. Perform arithmetic-result<w,rt), where rt is the scalar-result-type, to obtain
the scalar-result.
9.4.4.85 Unspec-bif
Arguments: x
Constraints: x must be of the form <argument>: <expression>: <value-reference>:
<variable-reference>: <data-description>: <item-data-description>.
Attributes: The result <aggregate-type> immediately contains *scalar,. The result
<data-type> has <bit>.
operation: unspec-bif(rdd,x)
Step 1. Let y be the <variable-reference> simply contained in x.
step 2,. Perform evaluate-variable-reference(y) to obtain a fgeneration+,g.
Step 3. Return an <aggregate-value, containing a &it-string-value+,v, which depends on
the properties of g in an implementation-defined manner. This value may be
<undefined%
360
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 379

9.4.4.86 Valid-bif
Arguments: sa
Constraints: All <data-type>s of sa must have <pictured>.
Attributes: The result *aggregate-type, is the *aggregate-type of sa. The scalar-
result-type has <bit>.
Operation: valid-bif(rdd,sa)
Perform generate-aggregate-result.
Step 1. Let sv be the scalar-value of sa; let dt be the scalar <data-type> corresponding
to sa. Perform validate-numeric-pictured-value(dt,sv) or validate-character-
pictured-value(dt,sv) according as dt has <pictured-numeric> or <pictured-
character>, to obtain a fpicture-validity+,pv.
step 2.
Case 2.1. pv has *picture-valid,.
The scalar-result is a &&t-string-value+: fbit-value-list+: <bit-value+:
<one-bit+.
Case 2.2. pv has <picture-invalid,.
The scalar-result is a fbit-string-value+: <bit-value-list+: *bit-value+:
<zero-bit>.
9.4.4.87 Verify-bif
Arguments: sa,ca
Constraints: The <aggregate-type% of sa and ca must be compatible. All <data-type>s of
sa and ca must have <computational-type>.
Attributes: The result *aggregate-type* is the common <aggregate-type+ of sa and ca.
The scalar-result-type is integer-type.
Operation: verify-bif(rdd,sa,ca)
Perform generate-aggregate-result.
Step 1. In either order, convert the scalar-value of sa to <character> to obtain sb and
convert the scalar-value of ca to <character> to obtain cb. Let m be the length
of sb.
Step 2. Each <character-value+ of sb, sbiil, (1 I i I ml, is compared in turn with the
<character-value% of cb.
Case 2.1. The string sb has a <null-character-strinq,.
The scalar-result is 0.
Case 2.2. The value of sbCi1 occurs in cb for all values of i.
The scalar-result is 0.
Case 2.3. (Otherwise).
The scalar-result is j, where the j'th <character-values of sb is the
leftmost <character-value+ of sb that does not occur in cb.
Chapter 9: Expressions and Conversion 361
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 380

I
**+*****+********+****+++******+****++**********************~************************* VBRIFY('297B', '0123456789')
will return the value 4, i.e., the position of the first non-numeric character in
the string '297B'
**********+***+*****++***+******+********************************************************** I
Example 9.10. An Example of the VERIFY bif.
362
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 381

9.5 Conversion
9.5.1 CONVRRSION OF SCALAR VALUES
The operation convert is used to convert a basic-value, sv, into a new basic-value
consistent with the target <data-type>,tt. The operand, st is a <data-type> associated
with sv; for example, it
yielded sv.
may be the <data-type> of an <expression> whose evaluation
One Of the following three relationships always holds for the operands:
(1) The <data-type>s, tt and st, have <computational-type> and sv is a <real-value,,
<complex-value+, <character-string-value,, or &it-string-value> that does not
contain fundefined,.
(2) The <data-type>, tt has <offset>, st has <pointer>, and sv is a <pointer-value+.
(3) The <data-type>, tt has <pointer>, st has <offset>, and sv is an <offset-value+.
In (2) and (31, the <offset> contains a <variable-reference>.
Note that convert may be invoked informally (see Section 9.5.1.1.).
Operation: convert(tt,st,sval)
where tt is a <data-type>,
st is a <data-type>,
sval is a *real-value+,sv,
or a <complex-value+,sv.
or a <character-string-value+,sv,
or a <bit-string-value+,sv,
or a *pointer-value+,sv,
or an <offset-value,,sv,
or a <basic-value+ containing an immediate component, sv,
of one of the above categories,
or an *aggregate-value, containing a *basic-value, with
such an irmuediate component, sv.
result: a <real-values, or a fcomplex-value,, or a <character-string-values,
or a <bit-string-value+, or a <pointer-value>, or an <offset-value+.
Case 1. tt has er> and <fixed> but not <pictured-numeric>, and sv is a &zeal-value+.
Perform convert-to-fixed(tt,st,sv) to obtain cv. Return cv.
Case 2. tt has <real> and <float> but not <pictured-numeric>, and sv is a freal-value,.
Perform convert-to-float(tt,st,sv) to obtain cv. Return cv.
Case 3. tt has <complex> but not <pictured-numeric>, and sv is a *real-value,.
Let rtt be a <data-type> with <mode>: <w>; but which is otherwise the same as
tt. Perform convert(rtt,st,sv) to obtain x. Return a <complex-value, whose
real part is x and whose imaginary part is 0.
Case 4. tt has <complex> but not <pictured-numeric> and sv is a *complex-value,.
Let rtt and rst be <data-type>s whose <mode>s have <w> but which are
otherwise the same as tt and st. Let x and y be the real and imaginary parts of
sv. In any order perform convert(rtt,rst,x) to obtain x' and convert(rtt,rst,y)
to obtain y..
Return a <complex-value+ whose real part is x' and whose imaginary part is y*.
Case 5. tt has er> but not <pictured-numeric>, and sv is a <complex-value,.
Let x be the real part of sv; let rst be a <data-type> with <mode>: er>; but
otherwise the same as st. Perform convert(tt,rst,x) to obtain cv. Return cv.
Chapter 9: Expressions and Conversion 363
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 382

Case 6. tt is <arithmetic> but not <pictured-numeric> , and sv is a fcharacter-string-
value, or a <bit-string-value,.
Step 6.1. Perform convert-to-arithmetic(st,sv) to obtain a *value-and-type+: rv rdt;
or a <value-and-type,: rv rdt iv idt.
Step 6.2.
Case 6.2.1. tt has <m>.
Perform convert(tt,rdt,rv) to obtain v. Return v.
Case 6.2.2. tt has <complex>.
If iv and idt do not exist, let iv be a <real-value,: 0; and idt be rdt.
Let rtt be a <data-type> which has - <real> but is otherwise as tt.
Perform convert(rtt,rdt,rv) to obtain vl; perform convert(rtt,idt,iv) to
obtain v2. Return a <complex-value+ whose real part is vl and imaginary
part is v2.
Case 7. tt is <pictured-numeric> and sv is a <real-value+, <complex-value+, fbit-string-
values, or <character-string-value+.
Step 7.1. If tt and st are identical, optionally return sv.
Step 7.2. Let att be the associated arithmetic data-type (see Section 9.5.2) of tt.
Step 7.3.
Case 7.3.1. att has <fixed>.
Perform convert(att,st,sv) to obtain v.
Case 7.3.2. att has <float> and <real>.
Step 7.3.2.1.
Case 7.3.2.1.1. sv is a <character-string-value+ or a tbit-str 'ng-value9.
Perform convert-to-arithmetic(st,sv) to obtain a fvalue-and-
type?. whose first two components are cv and ct.
Case 7.3.2.1.2. sv is a <complex-value+.
Let cv be the real part of sv; let ct be a <data-type> which
has <a> but otherwise is as St.
Case 7.3.2.1.3. sv is a freal-value,.
Let cv be sv; let ct be St.
step 7.3.2.2. Terform convert-to-float-decimal(att,ct,cvl to obtain v.
Case 7.3.3. att has <float> and <complex>.
Step 7.3.3.1.
Case 7.3.3.1.1. sv is a <character-string-value+ or a <bit-string-value+.
Perform convert-to-arithmetic(st,sv) to obtain a fvalue-and-
types: rv rdt; or a <value-and-type,: rv rdt iv idt. If iv
and idt do not exist, let iv be a freal-value,: 0; and let
idt be rdt.
Case 7.3.3.1.2. sv is a fcomplex-value+.
Let rv and iv be the real and imaginary parts of sv; let rdt
and idt be <data-type>s which have <real> but are otherwise
as st.
369
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 383

Case 7.3.3.1.3. sv is a <real-value+.
Let rv be sv; let iv be a <real-value+: 0; let rdt and idt
be st.
Step 7.3.3.2. Let ratt be a <data-type> which has <real> but is otherwise as
att. Perform convert-to-float-decimal(ratt,rdt,rv) to obtain
rvl; perform convert-to-float-decimal(ratt,idt,iv) to obtain ivl.
Let v be a &complex-value+ with real part: rvl; and imaginary
part: ivl.
Step 7.4. Perform edit-numeric-picture(v,tt) to obtain a <character-string-value+,
csv.
Return csv.
Case 8. tt has <string>, and sv is a freal-value,, <complex-value>, <bit-string-value+,
or <character-string-value+.
Step 8.1. If tt has <bit>, perform convert-to-bit(st,sv) to obtain s; otherwise,
perform convert-to-character(st,sv) to obtain s.
Step 8.2. Let n be the <maximum-length> component of tt. If n has <asterisk>, return --
S.
Step 8.3. Let m be the length of s. If m>n then perform raise-condition(<strinqsize-
condition>).
Step 8.4.
Case 8.4.1.
Case 8.4.2.
Case 8.4.3. m < n.
Case 9. tt has
n = 0.
Return a null-string.
m2 n>O.
Return a string whose length is n and whose i'th <bit-value+ or
<character-value> is the same as the i'th fbit-value> or fcharacter-
value+ in s, i=l,...,n.
If tt has <varyin+, then return s; otherwise return a string whose
lenqth is n, whose i'th <bit-value+ or <character-value+ is the same as
the- i'th <bit-value+ or <character-values in s for lli<m, and whose
remaining <bit-valuess or <character-value>s have, respectively,
fzero-bit+s or L)s.
<pictured-character>, and sv is a <real-value>. <complex-value>, fbit- _ - _ string-value>, or fcnaracter-string-value*.
Step 9.1.
Step 9.2.
Step 9.3.
Case 10. tt has <pointer>, st has <offset>: <variable-reference>,vr; and sv is an
Let ct be a <data-type> that has <character> and <nonvarvinq> and whose
<maximum-length> contains the associated character-string length of tt.
Perform convert(ct,st,sv) to obtain s.
Perform validate-character-pictured-value(tt,s) to obtain a fpicture-
validity+,pv. If pv has <picture-valid3, return s.
Let cbifs be a <condition-bif-value-list+ containing ionsource-value,: s;
and fonchar-values: n;, where n is the <integer-value+ in pv. Perform
raise-condition(<conversion-condition>,cbifsl. Let s be the immediate
component of the current <returned-onsource-value+, and go to Step 9.2.
<offset-value,.
Step 10.1. Perform Steps 10.1.1 and 10.1.2 in either order.
Step 10.1.1. If sv contains f-3, then return <pointer-value: tin+.
Step 10.1.2. Perform evaluate-variable-referencefvr) to obtain a fgeneration,,g.
Chapter 9: Expressions and Conversion 365
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 384

Step 10.2. Let edd , sal, and sil be, respectively, the tevaluated-data-
description+, l significant-allocation-list>, and <storage-index-list%
components of sv. The value of the tgeneration+,g (see Section 7.1.3),
has an farea-value>,av. av must not immediately contain <empty* and
there must be an immediate component, aa, of the farea-allocation-list%
of av such that the <significant-allocation-list+ of aa is identical
with sal. Let aud be an <allocation-unit-designator+ that designates
aa*s <allocation-unit+. Return a <pointer-value3: <generation+: edd
aud sil.
Case 11. tt has <offset>: <variable-reference>,vr:, st has <pointer> and sv is a
<pointer-value%.
Step 11.1. Perform Steps 11.1.1 and 11.1.2 in either order.
Step 11.1.1. If sv contains f-3, then return <offset-value+: f-3.
step 11.1.2. Perform evaluate-variable-referencetvr) to obtain a fgeneration+,g.
Step 11.2. Let edd, aud, and sil be, respectively, the fevaluated-data-
description,, +allocation-unit-designator3, and *storage-index-list,
components of sv. The value of the fgeneration+,g (see Section 7.1.3)
has an <area-value>,av. av must not immediately contain femutx+ and aud
must designate an fallocation-unit, that is an immediat=omponent of
one of the elements, aa, of the <area-allocation-list> of av. Let sal
be the <significant-allocation-list+ of aa. Return an <offset-value>:
edd sal sil.
9.5.1.1 Informal Invocation of Convert
The operation convert may be invoked informally by text of the form "convert source-value
to target-type", where "source-value" is text describing the third operand, sv, and
"target-type" is text describing the first operand, tt. The result reizurned by convert
may be referred to as "the converted value".
The second operand, st, may be specified explicitly in following text that describes the
source-type for the conversion. Alternatively, if the source-type is not explicitly
specified, and sv was obtained by performing evaluate-expression(x), the source-type is
by implication the scalar <data-type> of x that corresponds to the ibasic-value>,sv.
The description of the operand tt need not be complete. The following conventions are
applied to complete the description of the operand tt:
(1) When tt has <string>, its <maximum-length> has <asterisk> unless the contrary is
explicitly specified by a phrase such as "of specified length n", and it has
<varyinq> unless <nonvarvinq> is explicitly specified.
(2) When <base>, <scale> , and <mode> components are specified, then tt has <arithmetic>
and not <pictured-numeric> unless <pictured-numeric> is explicitly specified.
9.5.1.2 Convert-to-fixed
Operation: convert-to-fixed(tt,st.sv)
where tt is a <data-type> that has <real> and <fixed>,
st is a <data-type>,
sv is a <real-value>.
result: a <real-value3.
step 1. Let b=2 or b=lO, according as tt has <binary> or <decimal>. Let p and q be the
-umber-of-digits> and <scale-factor> of tt.
Step 2. Let cv = tb?-q~+signLsv)*floorC~btq)*abstsv)l.
366
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 385

Step 3.
Case 3.1. st has <fixed> and <scale-factor>: O;, or st has <fixed> and the same <base>
as tt.
Let v = cv.
Case 3.2. (Otherwise).
Let v be an implementation-dependent approximation to cv such that v=w*bt-q
for some integer w.
Step 4,.
Case 4.1. abs(v) 2 bt(p-q).
Perform raise-condition(<size-condition>).
Case 4.2. (Otherwise).
Return a <real-value>: v.
9.5.1.3 Convert-to-float
Operation: convert-to-float(tt,st,svI
where tt is a <data-type> that has <real> and <float>,
st is a <data-type>,
sv is a <real-value+.
result: a *real-value+.
Step 1. Let b=2 or b=lO, according as tt has <binary> or <decimal>. Let p be the
<number-of-digits> of tt.
Step 2,.
Case 2.1. sv is an integer such that abs(sv) < btp.
Return sv.
Case 2.2. (Otherwise).
Perform Step 2.2.1 or Step 2.2.2 or Step 2.2.3.
Step 2.2.1. Let e be the unique integer such that bt (e-l)<abs(sv)<bte. Let v be a
value such that abs(v-svllbt (e-p). Return a <real-value> containing an
implementation-dependent approximation to v.
Step 2*.2.2. Perform raise-condition(<underflow-condition>). Return a +real-value>:
0.
Step 2.2.3. Perform raise-condition(<overflow-condition>).
Chapter 9: Expressions and Conversion 367
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 386

9.5.X.4 Convert-to-bit
Operation: convert-to-bit(st,sv)
where st is a <data-type>,
sv is a (real-value>, *complex-value*, <bit-string-value+, or
*character-string-value+.
result: a tbit-string-value+.
Case 1. st has <arithmetic> (including <arithmetic> in <pictured-numeric>).
Step 1.1.
Case 1.1.1. sv is a *real-value>.
Let v=abs (sv) . Let ct be st.
Case 1.1.2. sv is a <complex-value,.
Let v=abs(x), where x is the real part of sv. Let ct be a <data-type>
with <mode>: <real>; but otherwise as st.
Case 1.1.3. st has <pictured-numeric>, and sv is a fcharacter-string-values.
Step 1.2.
Step 1.3.
Step 1.4.
Perform validate-numeric-pictured-value(st,sv) to obtain tpicture-
validity+,w. w must have *picture-vali*. The *picture-valid+
component of w then immediately contains a *real-value*,x, or a
<complex-value, whose real part is x. Let v = abs(x). Let ct be the
associated arithmetic data-type of st.
Let r be the <number-of-digits> of ct , and let s be the <scale-factor> of
ct. According to the <base> and <scale> of ct, determine p as follows:
<binary><fixed> : p = min(N,maxtr-s,O))
<decimal><fixed> : p = min(N,maxtceil(3.32+(r-s)),OJj
<binary><float> :P = min(N,r)
<decimal><float> :P = min(N,cei1(3.32+r)).
N is the maximum <precision> for <binary> and <fixed>.
If p=o, return a (bit-string-value+: <null-bit-strinq,. Otherwise let tt be
a <data-type> that has <w>, /Fixed>, i and <binarv> with <number-of-
digits>: .p; and <scale-factor>: 0. Perform convert(tt,ct,v) to obtain a
non-negative integer, n.
If n22tp, then perform raise-condition(<size-condition>); otherwise n can be
exactly represented as the sum of c[il+2t(p-i) for i=l to p where each ciil
is 0 or 1.
Return a fbit-string-value+ of length p, whose i’th *bit-value> has a fzero-
bit+ or <one-bit> according as cCi3 is 0 or 1.
Case 2. st has <&>, and sv
Return sv.
Case 3. st has <character>
values.
is a ibit-string-value,.
or <pictured-character>, and sv is a <character-string-
Step 3.1.
Step 3.2.
If sv contains a *character-value* that has neither $0) nor $12, go to Step
3.3.
If sv has a *null-character-strinq3, return the *bit-string-values:
*null-bit-strinq>. Otherwise return a -it-string-value>, whose length
equals the length of sv, and whose i'th <bit-value* has a (zero-bit.9 or
<one-bit+ according as the i'th <character-value, in sv has $0) or $1).
368
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 387

Step 3.3. Let cbifs be a fcondition-bif-value-list+ containing fonsource-value,: sv;
and fonchar-value,: n; where n is the position of the first fcharacter-
value, in sv that has neither FOO nor Cl?- Perform raise-
condition(<conversion-condition>,cbifsl. Let sv be the immediate component
of the current <returned-onsource-value*, and go to Step 3.1.
9.5.1.5 Convert-to-character
In this operation the notation *(n)$picture-element?' (or some other lowercase letter in
place of n) is used to indicate n (n>O) consecutive occurrences of that Cpicture-
element?).
Operation: convert-to-character(st,svl
where st is a <data-type>,
sv is a <real-value+, <complex-value>, fbit-string-value,, or
<character-string-value+.
result: a <character-string-value+.
Case 1. st has <real> and <fixed> but not <pictured-numeric>.
Step 1.1. If st has <binary>, let dst be a <data-type> which has <real> <fixed>
<decimal> with the converted <precision> of st for that target; othe=,
let dst be st. Perform convert(dst,st,sv) to obtain cv.
Let p and q be the <number-of-digits> and <scale-factor> of dst.
Step 1.2.
Case 1.2.1. p 2 q 2 0.
Let pit be the <data-type> corresponding to the Cpicture? with concrete-
representation as follows:
if q=O: '(21B(pl-9'
if p=q>O: '-9V.(q)9‘
if p>q>O: 'B(n)-9V.(q)9' where n=p-q.
Perform edit-numeric-picture(cv,picJ to obtain a fcharacter-string-
value+,v.
Return v.
Case 1.2.2. q>porq<O.
Let picl be the <data-type> corresponding to the Cpicture) with
concrete-representation '(p)-9'.
Perform edit-numeric-picture(x,picl) to obtain a fcharacter-string-
value,,vl, where x is a freal-value,: cv*lOtq.
Let k be the unique integer such that lOt(k-l)<abs(q)<lOtk. Let pic2 be
the <data-type> corresponding to the {picture? with concrete-
representation 'S(kl9'. Perform edit-numeric-pictured-qrpic2) to obtain
a <character-string-value,,v2.
Let f be a <character-string-value+ containing the single $symbol>: CF).
Perform concatenate(vl,fl to obtain ~1; perform concatenate(sl,v2) to
obtain s2.
Return s2.
Case 2. st has <real> and <float> but not <pictured-numeric>.
Step 2.1. If st has <binary>, let dst be a <data-type> which has <real> <float>
<decimal> with the converted <precision> of st for that target; othex,
let dst be st. Perform convert-to-float-decimal(dst,st,sv) to obtain cv.
Let p be the <number-of-digits> of dst.
Chapter 9: Expressions and Conversion 369
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 388

Step 2.2. Let k be the implementation-defined size of the exponent field for
<floating-point-format>s. Let pit be the <data-type> corresponding to the
{picture) with concrete-representation '-9V.(n)9ES(kI9', where n=p-1.
Perform edit-numeric-picture(cv,pic) to obtain a <character-string-value+,
V.
Return v.
Case 3. st has <complex> but not <pictured-numeric>, and sv is a <complex-value+.
Step 3.1. Let rst be a <data-type> that has <real> but is otherwise the same as st.
Let x and y be the real and imaginary parts of sv. In either order, perform
convert-to-character(rst,x) to obtain x' and convert-to-character(rstry) to
obtain y'.
Step 3.2. Let n be the length of the <character-string-value*,x' (and necessarily,
also of ~'1. Let i be the number of leading fcharacter-value>s in y' which
have Ms. Let y'Cj3. j=l,...,n, be the <character-value% in y'.
Step 3.3.
Case 3.3.1
Case 3.3.2.
y'[i+ll has C-t.
Let yi be a <character-string-value+ of length n+l containing, in order,
y'[i+ll through y'[nl, {symbol): $1); and y'[ll through y'lil.
(Otherwise).
Let yi be a <character-string-value+ of length n+l containing, in order,
$symbol~: f+);, y'li+ll through y'[nl, {symbol): {I); and y'C13 through
y'li-11.
Step 3.4. Perform concatenate(x',yi) to obtain a <character-string-value>,v, of length
(2*n+l).
Return v.
Case 4. st has <bit> and sv is a <bit-string-value+.
If sv has the <null-bit-strinq9, return *character-string-value,:
tnull-character-strinq9. Otherwise return a <character-string-value> whose
length equals the length of sv, and whose i'th <character-value> has CO+ or $1)
according to whether the i'th <bit-value, of sv has *zero-bit> or *one-bit+.
Case 5. st has <character>, <pictured-character>, or <pictured-numeric>, and sv is a
<character-string-value>.
Return sv.
9.5.1.6 Conversion to Float Decimal
This operation returns a <real-value> which is exactly representable in floating notation
in a given number (p) of significant digits. It is used in the conversion of numeric
values to character representations.
Operation: convert-to-float-decimal(tt,st,sv)
where tt is a <data-type> which has <real>, <float>, and <decimal>,
st is a <data-type> which has <real>,
sv is a *real-value+.
result: a <real-value>.
Step 1. If sv is 0, return a *real-value*: 0.
Step 2. Let p be the <number-of-digits> in tt. Let e be the unique integer such that
lOt(e-l)labs(sv)<lOte.
Let cv = lOt(e-p)+sign(sv)+floor(lOt~p-e)*abs(sv)+O.S)-
370
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 389

Step 3.
Case 3.1. st has <fixed> and <decimal>.
Return a freal-value,: cv.
Case 3.2. (Otherwise).
Return a *real-value, containing an implementation-dependent approximation
to cv, having the form m*lOtn, where m and n are integers and abs (m)<lOtp.
9.5.1.7 Conversion from Strinq or Picture to Arithmetic
Operation: convert-to-arithmetic(st,svI
where st is a <data-type>,
sv is a *character-string-value+ or a <bit-string-value+.
result: a *value-and-type> (see Section 9.5.1.8).
Case 1. st has <pictured-numeric>,.
Step 1.1. Perform validate-numeric-pictured-value(st,sv) to obtain a fpicture-
validity+,pv, which must have <picture-valid,.
Step 1.2.
Case 1.2.1. st has <real>.
Let rv be the <real-value> in pv. Let ast be the associated arithmetic
data-type of st. Return a *value-and-type,: rv ast.
Case 1.2.2. st has <complex>.
Let rv and iv be the real and imaginary parts of the fcomplex-values in
Pv. Let rast be a <data-type> which has <real> but is otherwise as the
associated arithmetic data-type of st. Return a <value-and-type+: rv
rast iv rast.
Case 2. st has <bit>.
Step 2.1.
Case 2.1.1. sv has <null-bit-strinq+.
Let cv=O and n=l.
Case 2.1.2. (Otherwise).
Let n be the length of sv. Let xlil be 0 or 1 according as the i'th
<bit-value+ of sv is <zero-bit+ or <one-bit+; let cv be the sum of
xCil+2t(n-i), for l<i<n.
Step 2.2. Let ct be a <data-type> containing <real>, <fixed>, <binary>, <number-of-
digits>: n; and <scale-factor>: 0.
Return a <value-and-type,: <real-value3: cv; ct.
Case 3. st has <character> or <pictured-character>.
Step 3.1.
Case 3.1.1. sv conforms to the syntax for <numeric-string+ (see Section 9.5.1.8) but
does not contain {P).
Perform basic-numeric-value(sv) to obtain a <value-and-type+,vt. Return
vt.
Chapter 9: Expressions and Conversion 371
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 390

Case 3.1.2. (Otherwise).
Let cbifs be a <condition-bif-value-list, containing fonsource-value>:
sv; and fonchar-value,: n;, where n is the smallest integer such that
the <character-string-value> of length n containing the first n
<character-value%s of sv does not have a continuation conforming to the
syntax of <numeric-string+ without $ZPp. (If the whole of sv has such a
continuation, let n be the length of sv.)
Perform raise-condition(<conversion-condition>,cbifsl. Let sv be the
immediate component of the current <returned-onsource-value>, and go to
Step 3.1.
9.5.1.8 Basic Numeric Value of a Strinq
<value-and-type+::= <real-value+ <data-type> [<real-value> <data-type>]
The optional components occur when a <value-and-type+ represents the two
parts of a <complex-value,.
<numeric-string+::= <blanks, 1
<blanks, EC+(-I {arithmetic-constant+ 1
<complex-expression,3 <blanks,
<blanks+::= I$%-list)1
*complex-expression+::= <sign-r+ {real-constant?
tsign-is {imaginary-constant+
*sign-r,::= [+)-I
<sign-i*::= +I-
Operation: basic-numeric-value(str)
where str is a <character-string-value, whose terminal components are
permitted terminal components of <numeric-string>.
result: a <value-and-type*.
Step 1. Let ns be the <numeric-string, whose components are the elements of str, taken
in order.
Step 2.
Case 2.1. ns contains only <blanks>.
Return a <value-and-type+ with components <real-value+: 0; and <data-type>
containing <arithmetic> with <real>, <fixed>, <decimal>, <number-of-digits>:
l;, and <scale-factor>: 0.
Case 2.2. ns immediately contains Carithmetic-constant?: Creal-constant+,rc.
Perform evaluate-real-constant(rc) to obtain a <value-and-type+,vt. If ns
immediately contains C-?, return a <value-and-type, equal to vt except that
the sign of the <real-value+ is negative; otherwise, return vt.
Case 2.3. ns immediately contains erithmetic-constant): @maginary-constant): Treal-
constant),rc $1).
Perform evaluate-real-constant(rc) to obtain a <value-and-type>: rv rdt. If
ns immediately contains C-2, let v=-rv; otherwise, let v=rv. Return a
<value-and-type+: <real-value+: 0; rdt <real-value+: v; rdt.
Case 2.4. ns contains <complex-expressions,cex.
Step 2.4-l. Perform evaluate-real-constant(rrc), where rrc is the {real-constant+
immediately contained in cex, to obtain a *value-and-type+: rv rdt.
Perform evaluate-real-constant(irc), where ire is the {real-constant) in
the {imaginary-constant) in cex, to obtain a *value-and-type>: iv idt.
372
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 391

step 2.4.2. If *sign-r+ has C-2, let rvl=-rv; otherwise, let rvl=rv. If eign-is
has C-t, let ivl=-iv; otherwise let ivl=iv.
Step 2.4.3. Return a *value-and-type+: rvl rdt iv1 idt.
9.5.1.9 Evaluate-real-constant
Operation: evaluate-real-constant(rc1
where rc is a {real-constant).
step 1.
result: a *value-and-type> (see Section 9.5.1.8).
Case 1.1. rc contains a fdecimal-constant).
Let v be the <real-value> obtained by interpreting the {decimal-number) in
rc as a decimal constant.
Let b = 10.
Case 1.2. rc contains a {binary-constant).
Let v be the <real-values obtained by interpreting the <binary-number) in rc
as a binary constant.
Let b = 2.
step 2. If *exponent* exists in rc, let x be the *integer-value+ obtained by
interpreting the components of {exponent) as a decimal constant; otherwise, let
x=0.
Step 3. Let ar be a <data-type> containing <arithmetic> containing <mode>: <real>; and
<scale>, (base>, and <precision> as defined below:
Step 3.1. <base> has <decimal> or <binary> as b=lO or b=2.
Step 3.2. <number-of-digits> is the total number of $digit@ or <binary-digit)s in
{decimal-number* or Cbinary-number).
Step 3.3.
Case 3.3.1. rc contains <scale-type): $E*.
<scale> has <float>.
Case 3.3.2. (Otherwise).
<scale> has <fixed>. <scale-factor> is (q-x), where q is the number of
{digit+ or {be-digit)s following C.) (if any) in sdecimal-number)
or {binary-number).
step 4. The <number-of-digits> in ar must not be greater than the maximum <numher-of-
digits> allowed for the <base> and <scale> of ar. Return a fvalueand-type,:
<real-value+: v+btx; ar.
Chapter 9: Expressions and Conversion 373
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 392

9.5.2 NUMERIC PICTURES
Informally, <pictured-numeric> is a way of holding a numeric value in a fcharacter-
string-value>. For a <pictured-numeric> in a <data-type>, the *character-string-value+
will be a value in the machine-state and represents a real or complex numeric value; for
a <pictured-numeric> in a <format-item> the value will be a string of characters
transmitted to or from a *stream-dataset,, an <expression> in a <get-string>, or a
<target-reference> in a <put-string>, and represents a real numeric value.
The <numeric-picture-specification> in <pictured-numeric> specifies:
(1) In conjunction with the optional <picture-scale-factor>, the <precision> and <scale>
of the decimal numeric values which may be held.
Together with <base>: <decimal>; and a <mode>, which is declared or defaulted (or
always (real> for a <format-item>), these make up the <arithmetic> subnode of
<pictured-numeric> established in Section 4.4.6.1. The <data-type> which contains
<arithmetic> with the same subnodes is known as the associated arithmetic data-tvne
of the <pictured-numeric> <data-type> or <format-item>.
(2) The constant length of the *character-string-value+ representations of numeric
values.
The associated character-strinq lenqth of a Ulumeric-picture-specification> or
<numeric-picture-element-list> is equal to the number of its terminal nodes, with
the exceptions of {K) and of {v).
The associated character-string lenqth of a <data-type> (or <format-item>)
containing <pictured-numeric> and <real> is equal to that of its <numeric-picture-
specification>.
The associated character-strinq lenqth of a <data-type> containing <pictured-
numeric> and <complex> is twice that of its (numeric-picture-specification>.
(3) Exactly how the <real-value+ or 4complex-value, is to be edited into or extracted
from the <character-string-value+.
Section 9.5.2.1 defines the editing of the freal-value+ or *complex-value? into the
<character-string-value+ under the control of the <pictured-numeric> specification;
Section 9.5.2.2 is a sub-operation of Section 9.5.2.1 which edits a single field (see
below) of a picture.
Section 9.5.2.3 defines the reverse process: checking the validity of a fcharacter-
string-value, against a <pictured-numeric> specification, and extracting the associated
freal-value, or fcomplex-value, from a valid <character-string-values; Section 9.5.2.4
is a sub-operation of Section 9.5.2.3 which checks the validity of (and extracts the
value from) a single field (see below) of a picture.
A field of a <numeric-picture-specification> (or of a <character-string-value+) is the
subtree of <numeric-picture-specification> (or the substring of the value) which
corresponds to one of the following:
<fixed-point-picture>
<picture-mantissa>
<picture-exponent>.
374
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 393

9.5.2.1 Editinq Numeric Pictures
Operation: edit-numeric-oicture(vc,pic)
where vc is a freal-value3 or a <complex-value+,
pit is a <data-type> containing <pictured-numeric> with associated
arithmetic data-type, adt.
result: a <character-string-value+.
Case 1. adt has <scale>: <fixed>; and <mode>: <real>.
Step 1.1. Let p and q be the <number-of-digits> and <scale-factor> of adt. There is a
unique representation of vc in terms of integers d[jl :
P
sgn + c (dCj1 * lOt(p-q-j))
j=l
where 0 I drjl 5 9, j=l to p
sgn = +l if vc 2 0 = -1 if vc < 0.
Perform edit-numeric-picture-field(spec,d,sgnI to obtain a tcharacter-
string-value+,s, where:
spec is the <numeric-picture-element-list> of pit,
d is the echaracter-string-value+ of length p whose j'th gcharacter-value,
has the $symbol) which is the {digit? that represents drjl.
Step 1.2. Return s.
Case 2. adt has <scale>: <float>: and <mode?: <real>.
step 2.1. Let p be the <number-of-digits> in adt; let q be the number of digit-
positions (as in Section 4.4.6.1) following the *VT <numeric-picture-
element> in <picture-mantissa> of pit (q = 0 if there is no +?I)); let px be
the number of digit-positions in <picture-exponent> of pit.
Then there is a unique representation of vc as vm + 10 t vx, where vx is a
signed integer such that:
10 *(p-q-l) 5 abs(vm) < lO*(p-q) if vc + 0
or vm=vx=o if vc = 0.
step 2.2. There is a unique representation of vm in terms of integers dCj3:
P
sgn + c tdCj1 l lOt(p-q-j))
j=l
where 0 I dtjfl 99, j=l to p
sgn = +I if vm 20 = -1 if vm <O.
Perform edit-numeric-picture-field(spec,d,sgn) to obtain a fcharacter-
string-value+,cm, where:
spec is the <numeric-picture-element-list> of <picture-mantissa> of pit,
d is the <character-string-value+ of length p whose j'th <character-value+
has the Csymbol) which is the {digit* that represents dfjl.
chapter 9: Expressions and Conversion 375
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 394

Step 2.3. If abs(vx) 2 10 t px, perform raise-condition(<size-condition>); otherwise,
there is a unique representation of vx in terms of integers dtjl :
Px
sgn + c tdCj1 * 10 t(px-j))
j=l
where 0 I d[jl I 9, j=l to px
sgn = +l if vx 20 = -1 if vx <O.
Perform edit-numeric-picture-field(spec,d,sgn) to obtain a fcharacter-
string-value+,cx, where:
spec is the <numeric-picture-element-list> of the <picture-exponent> of pit,
d is the <character-string-value* of length px whose j'th <character-value,
has the {symbol+ which is the {digit? that represents dCj1.
Step 2.4. Perform concatenate(cm,cx) to obtain c. Return c.
Case 3. adt has <mode>: <complex>.
Step 3.1. Let vcr, vci be the real and imaginary parts of vc; let pier be the <data-
type> which is the same as pit except that it has -de>: <real>.
Perform edit-numeric-picture(vcr,picr) to obtain cr; perform edit-numeric-
pictu.re(vci,picr) to obtain ci.
Step 3.2. Perform concatenate(cr,ci) to obtain c. Return c.
9.5.2.2 Editinq a Numeric Picture Field
fpic-status,::= <suppression+ [*suppression-type+1
<suppression*::= <on+ 1 l a> -
*suppression-type+::= [C-s+ I CS? I c+* I f-2 I VP I w-1
Operation: edit-numeric-picture-field(pic,d,sgn)
where pit is a <numeric-picture-element-list> containing ns <numeric-
picture-element>s,
d is a <character-string-value+ containing p $symbol)s, where p is the
number of digit-positions in pit and each {symbol) corresponds to
a digit,
sgn is the value +l or -1.
result: a <character-string-value+ of length nc. where nc is the
associated character-string length of pit.
step 1.
step 2.
If sgn = -1, perform raise-condition(<size-condition>) unless pit contains at
least one <numeric-picture-element> which immediately contains C-5). C+?r c-?*
CT*, {I), CR), <credit>, or <debit>.
If pit contains no <numeric-picture-element> which immediately contains $90,
CT), $9, CR), or $Y), and if all Csymbolts in d have CO), then return a
*character-string-value> of length nc all of whose *character-value% contain
$*a or I according to whether pit contains an $+) or not.
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 395

Step 3. Let stil, i=l,...,ns, be the <numeric-picture-elementis in pit.
Let dCj1, j=l,...,p, be the <character-value% in d.
Let cCk1, k=l....,nc, be *character-value,s; the remainder of this operation
completes the trees cCk3 as a function of the sCi.1 and dCj1.
Let pstat be a fpic-status,:
tsuppression,,sup:
<off+.
Let each of i, j, k be initially 1.
Step 4. Select the appropriate Case depending on ski].
Case 4.1. sCi1 immediately contains $9). CY), $T+, $12, or fR).
step 4.1.1.
Case 4.1.1.1. sup is <suppression+: dE+.
Replace sup by *suppression+: ts+. If tsuppression-type+ has
CS?. Cs*, C+?, or C-?. replace the +qmbol~ contained in cCk-11
by the {symbol) obtained from Table 9.3 as a function of
tsuppression-type, and (possibly) sgn.
Case 4.1.1.2. sup is fsuppression+: <m+.
No action.
Step 4.1.2. Attach to c[kl the {symholp obtained from Table 9.4 as a function of
sCi1, dijl, and (possibly) sgn.
Step 4.1.3. i=i+l; j=j+l; k=k+l.
Case 4.2. sCi1 has $2) or $+).
Step 4.2.1. If pstat does not have <suppression-type+, replace sup by <suppression,:
te ; and append fsuppression-typed: pc; to pstat,
component of stil.
where pc is the,,
Step 4.2.2.
Case 4.2.2.1. sup is fsuppression>: <on+; and dCj1 has CO).
If s[il has $2). attach 16 to cCk1;
if slil has $+), attach $+p to c1k3.
Case 4.2.2.2. sup is <suppression+: <off,; or dCj3 does not have $0).
Attach the immediate component of dijl to c[kl. Replace sup by
<suppression>: <off+.
Step 4.2.3. i=i+l; j=j+l; k=k+l.
Case 4.3. sCi1 has $$), CS), $+q, or $-a, and there is no other n such that s[nl =
sCi1.
Step 4.3.1. Attach to cCk1 the $symbol) obtained from Table 9.3 as a function of
s[il and (possibly) sgn.
Step 4.3.2. i=i+l; k=k+l.
Case 4.4. stil has $$), $32, $+X), or $-), and sin1 = sCi1 for some n greater than i,
but for no n less than i.
Step 4.4.1. Replace sup by <suppression+: tz*; append fsuppression-types: pc; to
pstat, where pc is the component of sCi1.
Step 4.4.2. Attach I to cCk3.
Step 4.4.3. i=i+l; k=k+l.
Chapter 9: Expressions and Conversion 377
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 396

Case 4.5. S[il has $$),,$S), C+), or f-a, and s[nl = s[il for some n less than i.
Step 4.5.1.
Case 4.5.1.1. sup is fsuppression,: t-3; and d[jl has $02.
Attach b to cIk1.
Case 4.5.1.2. sup is fsuppression,: tz+; and dljl does not have $0).
Replace the immediate component of cCk-11 by the +qmbol)
obtained from Table 9.3, as a function of sCi1 and (possibly)
sgn. Attach the immediate component of dCj1 to cCk1. Replace
sup by <suppression+: <off+.
Case 4.5.1.3. sup is *suppression+: tm+.
Attach the immediate component of dljl to clkl.
Step 4.5.2. i=i+l; j=j+l; k=k+l.
Case 4.6. sCi1 has $v).
Step 4.6.1. Perform Step 4.1.1.
Step 4.6.2. If pstat does not have <suppression-type>, append +suppression-type,
(with no subnode) to pstat.
Note: This handles the case of pictures with all digit-positions suppressible
and after fv;), for non-zero values.
Step 4.6.3. i=i+l.
Case 4.7. sCi1 has <insertion-character>.
Step 4.7.1.
Case 4.7.1.1. sup is *suppression*: t=,.
If sCi1 has CB+, attach ld to clkl; otherwise attach to c[kl the
terminal component of sCi1.
Case 4.7.1.2. sup is *suppression+: *on*. -
If <suppression-type, has $I), attach $+T to cCk1, otherwise,
attach 13 to cCk1.
Step 4.7.2,. i=i+l; k=k+l.
Case 4.8. sCi1 has <credit> or <debit>.
Step 4.8.1.
Case 4.8.1.1. sgn=+L
Attach 13 to each of cCkJ and cIk+ll.
Case 4.8.1.2. sgn=-1.
Attach to c[kl and clk+ll $Cg and {R) (respectively) if s[iJ has
<credit>, or CD) and $E) (respectively) if sCi1 has <debit>.
Step 4.8.2. i=i+l; k=k+2.
Case 4.9. sCi3 has {E).
Attach fE) to cCk1.
i=i+l; k=k+l.
378
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 397

Case Y.lO.stil has CR*.
i=i+l.
Step 5. If i<ns then go to Step 4.
step 6. Return a <character-string-value+ containing cLk1, k=l,....nc, in order.
Table 9.3. Table of fsymbolps as a Function of <suppression-type+ for Edit-numeric-
picture-field.
i I i I sgn I sCi3 or <suppression-type+ 1
i j---;s;----~-;----~I~----~~~--- I + I I ------- +------------------------------I
I i +1 I F+O c+4 u *
I I
-1 I c-q u c-* *
I f __--_-_-----------_-------------------
I The positions indicated by + I
represent a {symbolp which is I
implementation-defined. This I
:;zY*
represents a currency I
. I
I .--_----------------------------------- 1
Table 9.4. Table of fsymbol+s as a Function of <numeric-picture-element% and
<character-value% for Edit-numeric-picture-field.
i I dfjl i lsIi1 I sgn I -----------------------------------------------------------
I 1 (if applicable) I $0) I $1) I $2) I $32 I $4) I $52 I $6) 1 $72 I W I VP i I-----+--------------------+-----+-----+-----+ ----- +-----+-----+-----+-----+-----+-----,
i I CY) I I i I B j $1) j $2) / $3) / $4) 1 $52 j $6) 1 $7) / Cf3P / C9P I
~-----!--------------------!-----+-----+-----+-----+-----+-----+-----+-----+-----+----- I
I I I f i I I I f 1 $2) 1 C3t ; C4? ; $5) 1 $6) 1 +7# 1 CW ; W (
I I I I I I I I .+-----+-----+-----+-----+-----+-----+-----+----- !
I The 20 positions indicated by l represent 20 {symbol+ which are implementation- 1
I defined. These +?.ymbol)s represent a digit and a sign in one $symbol). I
I I L-------------------------------------------------------------------------------------- J
Chapter 9: Expressions and Conversion 379
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 398

Step 2.2. Perform validate-field-of-pictured-value(pm,vm) to obtain pvm; perform
validate-field-of-pictured-value(px,vxI to obtain pvx.
Step 2.3.
Case 2.3.1. pvm has <picture-invalid>.
Case 2.3.2.
Case 2.3.3.
9.5.2.3 Validity of a Numeric Pictured Value
<picture-validity*::= <picture-valid> 1 <picture-invalid>
<picture-valid+::= [<real-value+ 1 *complex-value31
<picture-invalid+::= <integer-value+
The subnode of <picture-valid+ is the associated numeric value of v with respect to pit.
The subnode of <picture-invalid+ is the ordinal of the first <character-value+ in v which
is invalid with respect to pit.
This operation is invoked by the operation valid-bif, by the operation convert for
<pictured-numeric> source-type, and by the operation validate-input-format for a
<picture-format>.
Operation: validate-numeric-pictured-value(pic,v)
where pit is a <data-type> or <picture-format> containing <pictured-
numeric>,
V is a <character-string-value+ of length equal to the associated
character-string length of pit.
result: a <picture-validity+.
Case 1. pit has associated arithmetic data-type with <mode>: <real>; and <scale>:
<fixed>.
Step 1.1. Perform validate-field-of-pictured-value(plst,v) to obtain a fpicture-
validity,,pv, where plst is the <numeric-picture-element-list> in pit.
Step 1.2.
Case 1.2.1. pv has <picture-valid+.
Let rv be the <real-value+ in pv. Return a <picture-validity+:
<picture-valid% freal-value*: rv*lOt(-q);;;, where q is the <scale-
factor> in the associated arithmetic data-type of pit.
Case 1.2.2. pv has <picture-invalid,.
Return pv.
Case 2. pit has associated arithmetic data-type with <mode>: <*> ; and <scale>:
<float>.
Step 2.1. Let pm, px be the <numeric-picture-element-list>s of <picture-mantissa> and
<picture-exponent> in pit. Let im, ix be the associated character-string
lengths of pm and px. Let vm, vx be <character-string-value% containing
the first im and last ix <character-value% of v, respectively.
Return pvm.
pvm has <picture-valid>, and pvx has <picture-invalid,.
Return a <picture-validity+: <picture-invalid+: <integer-value+:
(im+n);;;, where n is the value in pvx.
(Otherwise).
Return
(m+lOtfx-7);
<picture-validity+: <picture-valid+: +real-value,:
;;;, where m is the value in pvm, x is the value in pvx, and
q is the number of digit-positions following $Vt in pm (q=O if there is
no {V) in pm).
380
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 399

Case 3,. pit has associated arithmetic data-type with <mode>: <complex>.
Step 3.1. Let pier be the <data-type> which is the same as pit except that it has
<mode>: <real>. Let ir be the associated character-string length of
that of pit is therefore Z*ir.
pier;
Let vr, vi be <character-string-value&
containing the first ir and last ir <character-valueka of v, respectively.
Step 3.2. Perform validate-numeric-pictured-value(picr,vr) to obtain pvr, and perform
validate-numeric-pictured-value(picr,vtI to obtain pvi.
step 3.3.
Case 3.3.1. pvr has <picture-invalid%.
Return pvr.
Case 3.3.2. pvr has *picture-valid+, and pvi has <picture-invalid+.
Return a <picture-validity+: <picture-invalid>: <integer-value,: n;;;,
where n is the sum of ir and the value in pvi.
Case 3.3.3. pvr and pvi both have <picture-valid,.
Return a <picture-validity,: <picture-valid,: cv;;, where cv is a
<complex-value+ with real and imaginary parts equal to the values in pvr
and pvi, respectively.
9.5.2.4 Validity of a Field of a Numeric Pictured Value
Operation: validate-field-of-pictured-value(p1st.v) I__-
where plst is a <numeric-picture-element-list> of a <fixed-point-picture>,
<picture-mantissa>, or <picture-exponent>,
V is a <character-string-value+ of length equal to the associated
character-string length of plst.
result: a <picture-validity+ (Section 9.5.2.3).
Case 1. v is one of the values obtainable by normal return from performing edit-numeric-
picture-field(plst,drsgn) where d is the <character-string-value+ containing as
many digits as there are digit-positions in plst, each digit independently takes
each value 0 through 9 in turn, and sgn takes values +1 and -1 in turn.
Let vd and vsign be the unique values of d and sgn which edited to v. Let vx be
the <real-value, which is the integer containing the same digits as vd.
Return a <picture-validity+: <picture-valid,: <real-value,: vx+vsign.
Case 2. (Otherwise).
Let n be the lowest integer such that the first n <character-value% of v are
different from the first n <character-value% of all the values obtainable from
the editing operations in the predicate of Case 1.
Return a <picture-validity+: <picture-invalid+: <integer-value+: n.
Chapter 9: Expressions and Conversion 381
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 400

~---_L_-“^X-LI-- ___.___ _ _-. . _L _“~
9.5.3 CHARACTER PICTURES
The operation validate-character-pictured-value defines the checking of a echaracter-
string-value+ for validity with respect to a <data-type> (or <format-item>) which
contains <pictured-character>. This operation is invoked by operation valid-bif, by
operation convert for a <pictured-character> target-type, and by operation validate-
input-format for a <picture-format>.
The associated character-strinq lenqth of a <data-type> (or <format-item>) containing
<pictured-character>, or of a <character-picture-element-list> is the number of
<character-picture-element>s in its <character-picture-element-list>.
Operation: yalidate-character-pictured-value(pic.v)
where pit is a <data-type> or a <picture-format> that has
<pictured-character>,
v is a <character-string-value, whose length, n, is the
associated character-string length of pit.
result: a <picture-validity> (Section 9.5.2.3).
Step 1. For i=l,...,n, in order, perform Step 1.1.
Step 1.1. Let pc be the i'th <character-picture-element> in pit. Let c be the i'th
<character-values in v. Perform test-char-pit-char(pc,cl to obtain x. If x
is <false+, return a <picture-validity% <picture-invalid+: <integer-value+:
1.
Step 2. Return a <picture-validity,: *picture-valid,.
9.5.3.1 Test-char-pit-char
Operation: test-char-pit-char(pc,c)
where pc is a <character-picture-element>,
c is a <character-value%.
result: *true+ or *false,
Case 1. pc has $9).
If c has a digit or a b, return <true+; otherwise, return *false>.
Case 2. pc has {A).
If c has a letter or a b, then return f-3; otherwise, return <false+.
Case 3. pc has {x).
Return <true,.
382
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 401

Index
Entries in this index consist of operation nan;es, category-names, and other specially
defined terms. Operation naxes and category-names are cross-referenced, with the number
of occurrences (other than 1) on a page being indicated in parentheses. For operations
and non-terminal category-names, the first number references the gage of definition. For
other specially defined terms, the numbers indicate where a definition may be found.
Category-names ending in '-list*, '-commalist', and '-designatcr' are indexed with the
unsuffixed category.
A
abs-bif 316
<abs-bif> 59
4absent3 25 27 72 78 79 80 81(2)
82(2) 83 84(2) 90(3) 91(3) 92
95(2) 96 97(2) 98(5) 99(2)
lOl(2) 102(2) 104(3) 106(4) 117
118(2) 119(2) 123(3) 124(3) 126 154
171 173 189 190(3) 22712) 232 234
236(2) 237 238 239(3) 241(2) 243
244(3) 250 253 256(2) 259 262(2) 263
267 268 270 271 276 320(4) 323(2)
325(2) 355(2) 359(2)
abstract-block 63
abstract-block-component 63
abstract-block-contain 63
<abstract-external-procedure> 51(2)
32(2) 63(5) 148 220
aces-bif 317
<aces-bif> 59
activate-begin-block 155 156 158
activate-procedure 154 148 153 156 163 173
300
active ioperationl 29
<add> 58 115 140 303
add-bif 317
<add-bif> 59
addr-bif 318
<ad&-bif> 59
adjust-bound-pairs 215 214
advance-execution 157(2) 156 166 246
after-bif 318
<after-bif> 59
<aggregate-type> 146(3) 101 102(2)
115(3) 116 140 162 177 181 183 186(2)
190 196(5) 198(2) 199 200(3) 201(2)
202 203(7) 204(3) 222(2) 223(2) 238
250 255 261 289(8) 290(18) 291(7)
292(10) 293(5) 301(2) 302(4) 304(6)
305(6) 306(3) 308(3) 309(6) 310(6)
311(6) 312(3) 313(3) 316(2) 317(5)
318(4) 319(3) 320(3) 321(5) 322(5)
323(7) 324(2) 325(4) 326(5) 327(5)
328(5) 329(3) 330(4) 331(3) 332(3)
333(4) 334(3) 335(4) 336(4) 33714)
338(5) 339(6) 340(4) 341(5) 342(3)
*aggregate-type> (Ccntinued)
343(3) 344(3) 345(5) 346(3) 347(3)
348(7) 349 350(4) 351(2) 352(4)
353(4) 354(3) 355(4) 356(4) 357(4)
358(3) 359( 3) 360(3) 361(5)
<aggregate-value> 146 140 145 147(3) 159
160(2) 161 162(3) 163 164 165(2) 166
167 177(2) 181 183 186 189 190(3)
194 (2) 195(J) 196(5) 199 200(2)
201(2) 203(6) 204 207 220 222 223 227
231 234 235 238 243 248 250 255 261 266
280(7) 287 289(2) 290(2) 291(2)
292( 3) 293(4) 295 299(2) 300(5) 301
303 314(3) 318(2) 319 325 330 332(2)
334(2) 336 337 338 339 341 345(2)
346(3) 347(3) 348 349(2) 354(2)
355(2) 356(2) 358 360 363
ALIGNED 37 38 39 78(6) 84 85 86(2)
111 128 129
<aligned> 52 111
(alianment? 85 84
<alignment; 52(2) ill(2) 122 128
ALLOC 50
allocate 182 151 156 165 175 180 181 235
238
ALLCCA'IE 41 so
allocate-based-storage 181 180
allccate-contrclled-stcrage 180(2)
Callocate-statement) 41 36
<allocate-stateuent> 55 54 175(2) 180
allocate-static-stcrage-and-
build-static-directory 151 149 175
Qallocated, 146 183 192-
*allocated-buffer, 145(2) 193 226(2) 232
233(2) 234 235(2) 236(3) 237(3) 238
239 240 241 246
*allocated-storage, 146 143 149 175(2) 176
177 181 182(3) 192 193 197(2) 207 225
$allocationp 41(2) 82 94 103
ALLOCA'IION 50
<allocation> 55(2) 103(2) 180(3) 181
182(2)
allocation-bif 319
<allocation-bif> 59
*allocation-unit, 146(3) 147 175(7)
177(4) 180 182(5) 183(2) 184 185(2)
186(2) 190 132(4) 193(5) 197(2) 199
201 202(2) 203 205(3) 206(2) 207t4)
Index 383
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 402

*allocation-unit+ (Continued)
211 212 213 214 216 218 281(2) 282
307(3) 308 366(4)
ALLOCN 50
+alpha+ 147(3) 225 226(2) 230 241 263
278(2) 279(2)
<and> 58 115
aznd [instruction1 27
append-system-defaults 86 82
apply-by-name-parts 120 119
apply-constraints 140 137
apply-defaults 87 82 83(2) 135
apply-subscripts 121 118(2)
AREA 37 38 39 40 80 85 86(3) 93
100 112 129
<area> 53(2) 112 138 181 186 197 306 311
332 345 348
<area-allocation+ 146(2) 183(6) 192(4)
366(2)
<area-condition> 55 145 180 181 197(2)
{area-size) 37(2) 112
<area-sizes 53(2) 122 125 137 138(2)
165(2) 184 214 218(2) 307 308(2)
<area-value+ 146(2) 175 183(2) 186
192(2) 205 207 332 366(2)
<argument> 59(S) 123(6) 124(S) 126(4)
127(2) 164(4) 165 195(2) 198 201 202
203(2) 204 293 299(2) 314(7) 315(3)
318 319 330 331 332 336 338 341(6)
342(6) 345 348 349 356 360
Targuments) 44(3) 78 81 102(2) 117(2)
118(8) 119(2) 120 121 123
{arithmetic) 85(3)
<arithmetics 53(2) 60 113 133 135 138
159 160 198(2) 199 201(2) 202 295(6)
296 (5) 298 301 302 304(2) 305(2)
306(2) 308 309(2) 310(3) 312(2)
313(2) 364 366 368(2) 372 373 374(2)
{arithmetic-constant? 47(2) 38 44
66(2) 135 372(3)
arithmetic-result 298(2) 28 162 299 304
305 310 313(2) 316 317(2) 319 320
321(2) 322 324 326(2) 327 328(2) 330
331 332 333(2) 334 335(2) 336 337 339
340(2) 341 342 343 344 348 349 350 351
352 353(2) 354 356(2) 357(2) 358 360
asin-bif 319
<asin-bif> 59
assign 196 160 162(2) 181 186(2) 189 190
i94(3) 205 234 235(4) 238 250 255 261
266
assign-imag-pv 199
assign-onchar-pv 200
assign-onsource-pv 200
assign-pageno-pv 201
assign-real-pv 201
assign-substr-pv 203
assign-unspec-pv 204
*assignment-statement) 41 36 94 lOl(2)
102
<assignment-statement> 55 54 lOl(5) 175
194(2)
associated 289
associated arithmetic data-
type 374
associated character-
string length 374 382
<asterisk> 52(2) 53(2) 59 94 110
112(2) 114(2) 121(2) 122(2) 134 135
137(3) 13f!(4) 139 149 163 165 175
189 (2) 130 192 136(2) 212(2) 216(3)
217(S) 218(3) 223 242(2) 248 250 255
261 267 269 272(2) 283 289 290 291 232
365 366
Tasterisk-bounds? 38(3> 70 127
atan-bif 320 321
<atan-bif> 59
atand-bif 321
<atand-bs> 59
atanh-tif 321
tatanh-bif> 59
attach LinstructionJ 27
attribute consistency [syntax] 82
<attribute) 37 36 39 70(2) 71(5)
72(3) - 73(S) 74(S) 75(3) 76
80(6) 81(9) 82 83 84(4) 85
87(2) 88 89 90(2) 93(4) 108
109( 4) 116
Cattribute-keyword) 39(2) 84(2) 87 88
93
AU'IO 50
AU'ICMAIIC 37 39 50 84 86(2) 93 lb9
<automatic> 52 109 137(2) 156(5) 185(2)
205
*automatic-directory> 143(2) 154 155 185
193
fautomatic-directory-entry> 143(2) 156c.3)
185
B
{balanced-unit? 34(4) 105 106(2)
<base> 53(2) 113(2) 133(2) 135(3) 159
160(3) 295 (41 236(7) 298 301 302
303(3) 304(2) 305(2) 306 310(2)
312(9) 313(2) 314 315 316(2) 317(6)
319(2) 320(2) 321(4) 322 324(2)
326(4) 327(2) 328(4) 330 331(4)
332(3) 3.33(4) 334(2) 335(4) 336(2)
337(2) 339( 2) 340(4) 341(2) 342(2)
343(4) 344(4) 348(2) 349 (2) 35Of.3)
351(3) 352(3) 353(4) 354(2) 356(6)
357(4) 358(2) 360(2) 366 367 368
373(3) 374
<base-item> 52(2) 96 109 125 139 213 218
BASED 37 39 81 84 86 93 109
<based> 52(2) 96 103 106 109 119(2' 137
180 181(2) 191(2) 192 193 205 207(2)
237 254 268
basic-bit-value 264 249 254 262
basic-character-value 264 249 254
basic-numeric-value 372 253 262 263 371
{basic-reference) 44(3) 78(3) 80
81(2) 82 '92 118 120(2)
<basic-value, 146(3) 28 60 134 135
136(2) 140 142(2) 144 147 160(5)
161(2) 162(4) 163(2) 165 175(3) 176
177(3) 178(4) 179 181(2) 183 185
186(2) 189(2) 190(4) 192 194(2)
195(2) 196(3) 197(6) 199(2) 2u1
202(2) 204 206 207 210 219(5) 220(2)
221 223(S) 231 234 235(3) 242(2) 243
384
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 403

<basic-value+ (Continued)
248 250(2) 253 255(2) 260(2) 261(3)
267 269 272(4) 273(3) 280(4) 281 289
293 295 300 306(2) 307 363(2) 366
tbasic-value-index> 147(2) 178 183(2)
184(2) 192 197 206(3) 209
before-bif 322
<before-bif>
BEGIN 39
59
{begin-block) 34(4) 63 71 72 73(3)
94 95 96(2) 106(2) 107(2) 135
<begin-block> 54(2) 63 95(3) 96 148
154 155(2) 157 158 166 167 168 170(2)
220 221 222 253(3)
$begin-statement) 39 34 36 66 95
BIN 50
BINARY 37 38 39 50 85 86(3) 113 129
135
<binary> 53 113 295(3) 296(2) 322 366
367 368(4) 369 (2) 371 373
binary-bif 322
<bina&bif> 59
<binarv-constant>
ibinari-digit?
47(2)
'47(4)
373
373(2)
$binary-number? 47(2) 373(3)
BIT 37 38 39 85(2) 86(3) 93 112 129
<bit> 53 112 135 163 178 186 192 202
204(2) 208 218 249 254 261 267 269
272(2) 295 296 297(5) 302(2) 304(3)
306 308 309(2) 310 311(4) 323(9) 329
334(2) 354(2) 360 361 365 368 370 371
bit-bif 323
<bit-bif> 59
<bit-format> 43(2)
<bit-formats 58(2) 262 272
&it-strinq-value> 146(2) 134(2) 165
<bound-pair> 52(2) 53 108 llO(4) 111
11774) 120(4) 121(4) 122(S) 125
127(2) 137 138(2) 146 149(4) 165(2)
175(2) 176(2) 184 188(2) 192 196(7)
203(2) 210(2) 211(2) 212(3) 214(2)
215(3) 216(3) 217 218(2) 219(S)
220(2) 222(3) 223 253(2) 282 284(2)
289 (2) 29014) 291(3) 292(4) 293(2)
330(3) 332(2) 336(3) 338(3)
build-controlled-directory 150 149
build-foi 150 149(2)
build-file-directory-and-
intormations 149(2)
EUILTIN 37 39 81 84 93 108
<builtin> 51 108 118
<tuiltin-function> 59(2) 123(3) 314(S)
316
tuiltin-functicn-name 123
<builtin-function-reference> 59(2)
lOl(2) 118 121 123(2) 140 164 298 299
314(2)
BY 40 41 101
<by-nane-parts> 59(2) 57 101 102(7)
103(4) 115 117 120 205 213(3) 214 218
(by-option> 40(3)
<by-opticnj 54(3) 158 160(3) 162(2)
*by-value> 144(2) 148(2) 160(2) 161(2)
162
C
CALL 40
Ccall-statement) 40 36 80
<call-statement> 55 54 154 163
178(3)- 186(2) 197 203 208 249 254 f_carriaqe-return, 147 225 230 276 277
261(2) 262 264(3) 272 273 297(3) 278(3) 279
302(2) 304(2) 306(2) 307 308(2) case 26
309(4) 310(2) 311(4) 334(2) 354 360 <cat> 58 115 140 __
361(2) 363(3) 364(4) 365(2) 368(7) category-name 18
369 370 371 ceil 28
*bit-value> 146(2) 134(2) 135t41 136(2) ceil-bif 324
163
311(3)
186
318(4)
192
322(3)
197
323(3)
206
324(2)
329 334 338 350(2)
264(2)
354 355(2)
272(3)
361(4)
365(5)
297(2)
368(2)
302(3)
370 371
<blanks>
304(3)
372(5)
305(2)
block
307(3)
148
block-component 63
block-contain 63
fblock-control, 144 143 154 155 169(3) 187
246 248 249(2) 254 257 258 260 266(2)
267 268 270(2) 279 280(2) 281(2) 286
287(4)
<block-directory, 143(2) 154 155
<block-environment+ 145 143 154(2) 155 187
*block-state, 143(2) 29 (2) 144 145
146(3) 148(g) 15316) 154(7) 155(4)
156(5) 166(8) 167(g) 168(2) 169
172(3) 173(5) 185 187(4) 192 194 205
213 220(2) 221(2) 222(2) 247 248 263
281 287(4) 300 315(4)
BOOL 324
bool-bif 323 324
<bool-bif> 59
wund-pair) 37(2) 74(4) llO(3)
CHARACTER 37 38 39 50 85(2) 86(3)
93 112(2) 129
<character>
<ceil-bif>
53 112 134 138 178 186 192 199
200(3)
59
202 208 218 231 234 242(2) 248
249( 2) 254(2) 261 266 267(2)
CHAR
269 (2)
272(2)
50
296 297(4) 325(4) 328 337 341
345 346(2) 347(3) 358 359(S) 361(2)
365 368 370 371
character-bif 325
<character-bif> 59
Ccharacter-fornat? 43(2)
<character-tornat> 58(2) 262 272
<character-picture-element> 60(2) 131
382(5)
<character-string-value+ 146(2) 134(2)
144 145(8) 147(2) 165 169 170(2) 171
173(2) 178(3) 186(2) 197 203 208 226
227 230(2) 231(S) 232 234 236 242(3)
248 249(3) 250(3) 251(2) 252(3) 253
254 255(3) 256(6) 257(3) 260(6)
261(S) 262(3) 263(2) 264(4) 266(2)
267 269 272(2) 273(6) 274 275(3) 276
297 (3) 307 315(2) 325(2) 337(2>
Index 385
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 404

<character-string-value, (Continued)
341(2) 345(2) 346(2) 347(3) 358
363(3) 364(4) 365(3) 368(3) 369(6)
370(8) 371 372(2) 374(8) 375(5)
376(5) 379 380(2) 381(3) 382(2)
<character-value, 146(2) 134(4) 186(2)
192 197 200(2) 206 231(10) 249 250(2)
255 256(2) 260(3) 263 264(7) 273 275
276(4) 297(2) 305(2) 307(3) 318(4)
322(3) 328(2) 337 338 341 345(3)
350(2) 355(2) 358 359(3) 361(4)
365(S) 368(2) 369 370(3) 372 375(2)
376(2) 377(2) 379 380(2) 381(3)
382(2)
check-attribute-completeness-and-
delete-attributes 93 82 83(2)
check-based-reference 207(2)
check-simply-defined-reference 218 214 216
close 232(2) 151
CLOSE
{close-stZ:ementp 41 36
<close-statement> 56 54 232
fclosed> 145 150 232 234 236 237 239 240
247(2) 248 265
COL 50
collate-bif 325 307 337 341 359
<collate-bif> 59 314
collating sequence 325
collect-subscripts 120(2) 118
COLUMN 43 50
+olumn-format* 43(2)
<column-format> 58(2) 259 271
<comma-subscript> 283(g) 281(2) 284(3)
285(2)
comment 174(3) 47
{comment* 47(2) 64 65
{comment-body> 47(2)
<comment-character) 47(2)
common 290
common aggregate-type 290
compare 306(2) 88 142(4) 164(2) 308
309(2) 310 311
+comparison-operator? 44(2)
fcomparison-result, 306(2)
compatible 290
complete [tree1 22
complete-attribute-implications 70 68
complete-concrete-procedure 68 63
complete-declarations 82 68
fcomplete-file-description+ 145(2) 201 230
complete-options 68(2)
complete-structure-declarations 75 68
COMPLEX 37 38 39 50 85(2) 129 131(2)
135(2)
<complex> 53 113 131 133 185 195 198 201
260 273 295(2) 298(2) 306 308 309(2)
310 313 316(3) 320(3) 321 326(2) 331
337 339(2) 344 350 351(2) 352 353 354
357 363(2) 364(2) 370 371 374 376 381
complex-bif 326
<complex-bif> 59
*complex-expression, 372(3)
<complex-format) 43(2)
<comnlex-format>
fcomplex-number,
58(2) 57 259 260 272
146(Z) 313
fcomplex-value, 146(2) 28 136 144 160
162(3) 185 199(2) 202(2) 260 273
298(4) 299 326 363(7) 364(4) 365(3)
*complex-value> (Continued)
368(3) 369 370 371 372 374(3) 375 380
381
component 17
Ccomputational-conditicn) 36(2) 40 99
lOO(3)
<computational-condition> 55(3) 99(2)
lOO(4) 145 169(2) 170(3)
Ccomputational-type? 85(2)
<computational-type> 53(2) 92 102
112(2) 113 125 135 138(2) 158(2) 159
160 163 197 203 204 254 269 295 296 297
301 302(2) 304(2) 305(2) 306 308
309(2) .310(2) 311(2) 312 313 316
317(2) 318 319 320 322(2) 323(2) 325
326 327(2) 328(2) 329 330(2) 331 332
333(2) 334(2) 335(2) 336 337(2)
338(2) 339(2) 341 344 348 349 350(2)
351 352(2) 353(2) 354(2) 355(2)
356(2) 357(2) 358 359 361 363
concatenate 305(2) 327 329(2) 355 369(2)
370 376(2)
concrete-block 63
concrete-block-component 63
concrete-Hock-contain 63
Tconcrete-external-procedure) 31(2)
63(5) 68(6) 69(6) 7014) 71(4)
75(4) 80(2) 82(6) 83(2) 86
87(3) 91 92(4)
concrete-representation 19
CCND 50
CONDITION 37 39 40 50 81 84 86 108
<condition> 51 108 117
fcondition-bif-value> 145 143 154(2) 169
170(5) 171(2) 172 173(4) 174 194
227(5) 315 365 369 372
Cconditicn-name) 40(3) 34 35 94
lOO(5) 107(2)
<condition-name> 55(3) 54 lOO(5)
107(2) 169 170 171(2) 172(3) 174
Ccondition-prefix) 36 34 35 99 107
<condition-prefix> 55 51 54 58 95 98
99(5) 104 106 170(2)
conditions-in-arithmetic-
expression 299(2) 316 317 319 320 321
324 327 328(2) 331 332 333(2) 334 336
339 340(2) 343 344 349 351(2) 352
353(21 354(2) 356 357(2) 358 360
conjg-bif 326 ,
<conjq-bif> 59
connectez 206
<consistent-declaration) 84(2) 93
{consistent-description+ 85 84 93
<consistent-literal-constant* 85 84
$constantp 44(2) 94 116 134
CONSTANT 37 39 74 80 85(2) 86(10)
92( 6) 93( 2) 108 135
<constant> 60 58 92 116 125 134I2) 135
136 138 139(3) 14012) 142(4) 164 253
300 312 315 351
constraint 140(4) 37 38 52 53 55 56
57(3) 58 137 141(81
constraint-expression 140(5) 141(3)
construct-contextual-declarations 80 68
construct-explicit-declarations 71 68
construct-implicit-declarations 82 68
construct-record 243 232 236(2) 238 239
386
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 405

construct-statement-name-
declarations 73(4) 71 92(2)
contain 17
contents of a picture [syntax] 131
{control-format? 43(2)
<control-format> 58 57 258(2) 270 271
286(2)
*control-state> 31(3) 29(3)
CONTROLLED 37 39 50 84 86 109
<controlled> 52 103 105 109 137 142 150
180(2) 185 191(3) 205(2) 318(2) 319
fcontrolled-directory, 143(2) 149 150 180
185 192
*controlled-directory-entry> 143(2)
150(3) 180 185(2) 191 318 319
<controlled-group> 54(2) 97 158 159
167(2) 168
fcontrolled-group-states 144(2) 148(6)
159(5) 160(4) 161(2) 162(2) 167(4)
CONV 50
CONVERSICN 36 50
<conversion-condition> 55 169 170 226
227(4) 249 255 260(2) 261(2) 365 369
372
convert 363(8) 28(2) 136(2) 142 160(2)
161 162 163 197 200(2) 201 204(2) 207
216 218 231 242(2) 248 260(2) 267 269
272(3) 273(3) 274 295 301 302(2)
304(3) 305(2) 306 307(3) 310 311(2)
312(2) 313 316(2) 317(3) 318(2) 319
320(3) 321 322(3) 323(8) 324 325(3)
326 327(3) 328(2) 329(3) 330 331(2)
332 333(2) 334(2) 335(2) 336 337 338
339 (2) 340(2) 341 342 343(2) 344(2)
345 348(2) 349 350(2) 351 352(2)
353(2) 354(2) 355(4) 356(3) 357(2)
358 359(4) 360 361(2) 364(4) 365
366(3) 368 369 380 382
convert-to-arithmetic 371 272 273 364(3)
convert-to-bit 368 365
convert-to-character 369 365 370(2)
convert-to-fixed 366 363
convert-to-float 367 363
convert-to-float-decimal 370 274 364
365(2) 369
convert-to-logical-levels 77 75
converted <number-of-
digits> 296
converted <precision> 296
converted <scale-factor> 296
*converted-by-type,
*converted-to-type,
copy [of a tree1
COPY 42 68 327
copy-bif 327
<copy-bif> 59
copy-descriptors
*copy-file+ 145 143 _
144(2) 160(2) 161 162
144(2) 160(2) 161(2)
21
88 74 75 83
247 248(3) 263
$copy-option* 4213) 68(3) 80
<copy-option> 56(2) 57 227 247(3)
248(3)
correspond 18 291
corresponding block 148
cos-bif 327
<cos-bif> 59
cosd-bif 328
<cosd-bif> 59
cash-bif 328
<cash-bif> 59
CPLX 50
CR 60 132
create-abstract-equivalent-tree 94(2) 95
96 97(3) 98 99 103(2) 104(_1) 105
106(3) 107 113 114(3) 115 117 128 135
create-allocation 103
create-argument-list 123 118(2) 119
create-assignment-statement 101
create-balanced-unit 106(3) 105
create-begin-block 95
create-block 96 95(Z)
create-bound-pair-list 110 108 111
create-builtin-function-reference 123 118
create-by-name-assignment lOl(2)
create-by-naue-parts-list 102 101
create-conditicn lOO(3) 94 99(2) lU7
create-conditicn-prefix-list 99 95 98
104 106 107
create-constant 134 84 116 136
create-data-description 110 99 109 111
113(2) 129(2)
create-data-type 112 111
create-declaration 107 63 96
create-entry 113 108 112
create-entry-or-executable-unit-
list 97(2) 96
create-entry-pcint 98 95 97
create-entry-reference 124 119(2)
create-executable-unit 98(2) 97 105(2)
create-executatie-unit-list 98 96 97(2)
create-expression 115(5) 92(2) lOl(2)
104 105 106 114 116(2) 121 123
create-format-iteration 104
create-f ormat-statement 104 96
create-freeing 105
create-grcup 97
create-identifier 114 99 103 107 111
create-if-statement 105
create-initial-element 114
create-locate-statement 106
create-named-constant 108(2)
create-named-constant-reference 122 118
create-numeric-picture 133 131
create-on-statement 107
create-picture 131 112 129
create-procedure 95 63 96
create-pseudo-variable-reference 124 118
create-refer-option 114 llO(2) 112(2)
create-reference 117 94 95 96(2) 100
lOl(2) 105 116 119 126
create-statement-name-list 99(2) 98 104
106
create-value-reference 121 119(3)
create-variable 109 108
*credit* 132(3) 133
<credit> 60(2) 376 378(2)
CTL 50
current block 148
current 148
*current-file-value+ 144(2) 169 246 249
250 254 255 260 261
*current-position, 145(2) 226(4) 230(3)
233 234(2) 235 236(2) 238 239(3) 240
241(4) 244(3) 2SO 255 256 259(2) 263
276 277 278 279
*current-scalar-item> 144(2) 246 249 258
267 269(2) 270 272 279(3) 280(11)
Index 387
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 406

*current-scalar-item, (Continued)
281(2)
+cv-target, 144(2) 159 160(3) 161 162(2)
l cv-type, 144(2) 159(3) 160(5) 161 162
D
DATA 42 43 147
<data-attribute) 37(3) 70(E) 71
73(3) 74 75 80(3) 81 83 84 85
87(2) 88(2) 90(7) 91(5) 93(4)
112
{data-basic-reference) 252(4)
*data-description3 a4t2> 85
<data-descriDtion> 52(3) 53(2) 55
58(4) -59(a) 96 99 lOl(3) 102(2)
109 llO(3) 111 113(2) 115(2) 116(6)
117(5) 118(3) 119(5) 120(5) 121(3)
122(2) 123(3) 124(4) 125(4) 127(5)
128(2) 129(2) 137(2) 138 13912) 140
141(4) 147(2) 151 156 160 162(2)
164(2) 165(2) 166 167 175(2) 176(g)
177(2) 180 181(2) 182 184(4) 185
186(2) 188(5) 189(2) 190(3) 192
194(4) 196(4) 197 198(2) 199 200(2)
201(2) 203(2) 204(2) 205 206 207
208(4) 209(4) 210(5) 211(3) 212(3)
213(2) 215(3) 216(3) 218 231 234 235
238(2) 248 250 252 253(4) 254(2) 255
261 266 268 280 282(5) 283 284 289(10)
291(2) 292(3) 293(2) 301 303 308 314
330(2) 332 336(2) 338(2) 349 356 360
*data-directed-input) 42(2)
<data-directed-input> 57(2) 248(2) 252
{data-directed-output) 43(2)
<data-directed-output> 57(2) 159 265 266
268
<data-format) 43(2)
<data-format> 57(2) 258 259 270 272
286(Z)
*data-item-control, 144(2) 246 249(2)
257(2) 258(3) 267(2) 268(2) 270(4)
271 279(2) 280 281(E)
<data-item-indicator, 144(2) 162 249 257
258 267 268 270 271 279 280 281(4)
*data-list-indicator> 144(2) 249 257
258(2) 267 268 270(2) 271 279 280
281(3)
*data-name-fields 144(2) 269(2) 281
*data-source) 43(3)
<data-source> 57(3) 268(4)
$data-structure-reference) 252(2)
*data-subscript) 252(2) 253(3>
{data-subscripts) 252(2) 253(2)
*data-symbol) 251(4) 257(5)
(data-target) 42(2)
<data-target> 57f2) 254(3)
{data-type) 85 84
<data-type> 53 52 60 102(2) 111 112(8)
113-il5(4) 116(5) 119(4) 122 124 134
135(3) 136 138 140 142(2) 144(4)
159(3) 160(7) 162(5) 163(2) 164 165
166 177 178 192 193 195(21 196 197
198(3) 199 200(7) 201(4) 202(5)
203(4) 204(4) 207 208(3) 209(2) 210
231(2) 242(4) 248(2) 250 253 254 255
<data-type> (Continued)
260(2) 269(2)
272(7)
"2;;:;; 263 267(2)
274(4) 275(2) 280 281
287 291 292(8) 293(6) 295(E) 296(8)
297(3) 298 299 301(2) 302(3) 303(4)
304(6) 305(8) 306(7) 307 308(6)
309(8) 310(9) 31X(5) 312(5) 313(5)
314 315 316(8) 317(10) 318(3) 319(5)
320(6) 321(5) 322(4) 323(2) 324(3)
325(2) 326(6) 327(4) 328(7) 329
330(4) 33118) 332(6) 333(E) 334(4)
335(4) 336(5) 337(4) 338(3) 339 (5)
340(4) 341(8) 342(6) 343(8) 344(6)
345(6) 346(3) 347(3) 348(a) 349(11)
350(5) 351(10) 352(3) 353(4) 354(6)
355(6) 356f18) 357(4) 358(3) 359
360(7) 361(3) 363(11) 364(3) 365(2)
366(3) 367(2) 368(3) 369(6) 370(4)
371(3) 372(3) 373 374(5) 375 376 380
381 382(3)
tdataset, 147 32 143 145 148 149 225(6)
226(4> 228(2) 230(4) 232(2) 236(2)
238 241 244(4) 250(2) 255(2) 256(2)
258(2) 263(2) 276(2) 278(2) 279(2)
fdataset-name, 147(2) 230
date-bif 328
<date-bif> 59 314
DB 60 132
DCL 33 50
{debit) 132(3) 133
<debit> 60(2) 376 37812)
DEC 50
DECA'I 329
decat-bif 329
<decat-bif> 59
DECIMAL 37 38 39 50 85 86(2) 129 135
<decimal> 53 113 133I2) 160 260(3)
262(2) 273 274(3) 295 296(2) 315 330
366 367 368(2) 369(2) 370 371 372 373
374
decimal-kif 330
<decimal-bif>
{decimal-constant:' 47(2) 373
idecimal-number> 47(2> 262 263(2)
373(3)
$declaration) 36(3) 31(2) 51 63(2)
68(2) 71(14) 72(2) 73 74 75(7)
7615) 77(5) 78(3) 79(10) 80
81(2) 82(7) 83(5) 84(3) 87(3)
88(4) 89(3) 91 92I4) 93(6) 96
lOO(2) 102(2) 103(4) 105(2) 106(2)
107 108(2) 109 110 111 112 113 117(4)
126 131(3) 135
<declaration> 51(6) 54 55(3) 56
59(21 63(3) 96(4) lOO(2) 103(2)
105(2) 106(2) 107 108 109 110 117(4)
118(2) 119(3) 122(2) 123 124 125(3)
137(7> 138(2) 139(4) 141(6) 142(U)
143(3) 149 150(4) 151(2) 155 156(2)
164(2) 173(2) 180(7) 181(2) 182(j)
185(3) 187(2) 188(3) 189 191(4)
192(2) 193(6) 202(2) 205(2) 207(4)
210 213(3) 214(2) 215 216(3) 217
218(6) 220(2) 223(4) 233 237(6)
238(3) 252 253(3) 254(4) 268(2)
283(2) 318(2) 319(3) 330 336 338
declaration-component 63
declaration-contain 63
388
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 407

<declaration-type) 84(2)
<declaration-type> 51(2) 108(4) 254
DECLARE 36 50 72 73 80 82
declare-parameters 72 '71
Cdeclare-statement) 36(2) 33(2) 68
69(2) 71 72 73(2) 80 82 109(2)
135
declare-statement-names 72 71 73
qdeclared-statement-names+ 71(2) 72
73(2)
deduce-in-option 193 192(2)
DEF 50
defactor-declarations 71(2) 68
DEFAULT 39 50 86(21,
Cdefault-attrilzutesb 39(2) 63 82(2)
$defaZt-.Zlecification) 39(2)
qdefault-statement* 39 33 36 68 69(2)
86(2) 87(2)
defaults [system defaults, PT./I text3 86
define-program 31(3)
DEFINED 37 39 50 84 86 93(3) 109(2)
116
<defined> 52(2) 109 125 137(2) 139
156(4) 185(2) 205 213(2) 216 217
21813)
<defined-directory+ 143(2) 154 155 185
<defined-directory-entry, 144 143 156(2)
185 213
defining production-rule 21
delete 241 240
delete [instruction] 27
DELETE 41
*delete-flag* 145 226 230 239 241(2)
245(4)
{delete-statement) 41 36
<delete-statement> 56 54 240(2)
Qdeliaiterp 47(3) 64 65(6)
qdelimiter-or-non-delisitelp 65(4)
qdelimiter-pair? 47(2) 64
derived <kase> 295
derived <mode> 295
derived <scale> 295
derived <string-type> 297
derived common <base> 295
derived common <rode> 295
derived common <scale> 295
derived common <string-
type> 297
<description* 37(2) 38 39 63(2) 66
75(3) 76(2) 77(2) 78(2) 82(2)
83(4) 84(2) 87(5) 88(2) 90(3)
93(3) 99 llO(2) 112(2) 113(6) 129
131(2)
designate 18
desisnator 18
tdesignator, 144(2) 145 226(2) 236(2)
241 244(21 245(4) 263(2) 276 281(2)
determine-structure 76 75
DFT 50
fdigit) 47(6) 48 251 358 373(2) 375(2)
376
digit-position 132
$digitsg 132(4)
DIM 50 330
DIMENSION 37 38 39 50 70(3) 74 84
85 93(2) 127(2)
$dimension-attritutep 37(2) 70(3) 73
74 88 108 llO(2)
dimension-bif 330
<dimension-bif> 59
Cdinensicn-suffix) 37(3) 36 70 74
fdimensioned-aggregate-type, 146(2) 222
289 290(5) 291(3)
<diaensioned-data-description> 52(3) 111
117(3) 118(3) 120(3) 122 127(2) 128
138 176 177(2) 188(4) 189 210 211(3)
212 215(3) 216 217 253 282 284(2) 263
292 330 332 336 338 349 356
CIRECT 37 39 41 85 86 108
<direct> 53 56 108 145 229(3) 230(3)
236 239 240 241 244
<direct-set> 85(2)
<disabled> ' 55 99
<disabled> 163(Z) 170(2)
$disabled-computational-
conditicnt 36(2) 99 lOO(2)
<divide> 58 115 140 303
divide-bit 331
<divide-Dif> 59 -~ DO 40(3) 42 43(2)
$.do-spec) 40(2) 42 43(2) 97
<do-spec> 54(2) 57(3) 97 137 158(2)
159 (3) 161(~) 162(2) 281
qdo-StateKent) 40 34 36 46 66 97
dot-bit 332
<dot-bif> 59
<dot-name> 283(6) 281(2) 26413) 285(2)
sdrifting-dollar-field? 132(3)
{drifting-field) 132(2)
Fdrifting-sign-field) 132(3)
<dummy> 59 124 125(2) 165(2)
<dunmy> 144 156 165 193
E
e 28
EDIT 42 43
/edit-directed-input* 42(2)
<edit-directed-input> 57(2) 159 248(2)
257
fedit-directed-output+ 43(2)
<edit-directed-output> 57(2) 159 265 266
270
$.edit-inpIt-Fair) 42(2)
<edit-input-Fair> 57(2)
edit-nuK,eric-Output 273(4)
edit-numeric-picture 375 274 275(2) 365
369(3) 370 376(2)
edit-numeric-picture-field 376(2) 375(2)
379( 2) 381
Cedit-output-pair* 43(2)
<edit-output-pair> 57(2)
felenent-aggregate-type> 146(2) 222 289
290(S) 291t4)
<element-data-description> 52(2) 111
117(2) 120(2) 122 127 176 177 188(3)
189 210 211(3) 212 215 216(2) 253 282
284 289 292 332 349 356
ELSE 34(2) 35 105(2)
Celse-part* 35(2)
<else-unit> 55(2) 105 106 163(2)
Index 389
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 408

*empty> 146 183 186 192 332 366(2)
empty-bif 332
<empty-bif> 59 140 314
<enabled> 55 99(2)
&enabled+ 169 170(3)
END 40 66
send-statement? 40 34 36 66(3)
<end-statement> 54 97 98 107 148(2)
153(2) 156 157 164 168
ENDFILE 40
<endfile-condition> 55 226 227 245(2) 250
255 256 264 278
{ending* 34(3) 33 83
ENDPAGE 40
<endpage-condition> 55 174 226 275 277 278
<entry) 85(2)
ENTRY 37 38 39(2) 69 71 73 74
85(2) 86(5) 88 90(2) 91(2)
92(2) 93(3) 108 112 113 128(2) 129
<entry> 53(3) 108 112 113(2) 119(2)
122(2) 124 137 138(3) 139(2) 141
164(2) 165 196 220(2)
*entry-information) 39(3) 69 71 73(3)
74 75
<entry-information> 54(2) 98 (2) 107
<entry-or-executable-unit> 54(2) 51
95(2) 96 97(6) 107(4) 154 157(4)
16812) 220 222
<entry-point> 54(2) 32(2) 95(2)
97(3) 98(6) 107 144 146 154(2)
155(2) 157 164(2) 167(4) 168(2)
172(2) 173(2) 220(4)
entry-reference 164
*entry-statement* 39 33 36 69(3)
72(2) 73 79 83 89 97 98(4)
<entry-values 146(2) 32 144 145 148 164
172 173 220
enumerated-tree 20
ENV 50
{environment) 38 37 41(2)
ENVIRONMENT 38 39 50 85 108
<environment> 53(2) 56(2) 108(2)
142(4) 145
<environment-specification.+ 38(2) 65
epilogue 156 153(3) 154(2) 166(2)
167(2) 168(2) 175
<ecJ= 58 115
equal [trees3 17
<equals 306(3) 307(3) 308(2) 309
erf-bif 333
<erf-bif> 59
erfc-bif 333
<erfc-bif> 59
ERROR 39 40 87 100
<error-condition> 55 145 173 174(2) 227
234 236 237 239 240 247(2) 248 250 251
256(2) 257 263 264 265 276 312(2) 313
establish-argument 165 164 175
establish-controlled-group 159 158 162 281
establish-next-data-item 279 249 258 267
269 270
establish-next-format-item 285 258 270 286
287
establish-next-spec 161 159 162
establish-truth-value 163(2) 158 161
*established-argument, 144(3) 155(2)
164(4) 165(3) 193
<established-on-unit> 145 143 171(2)
172(4) 173(3)
evaluate-argument 300
evaluate-huiltin-function-
reference 314 290 299
evaluate-by-name-parts-list 213 205 214 218
evaluate-constant 300
evaluate-current-column 278 259 267 268(2)
269 270 271 275 276 277
evaluate-current-line 279 275 277 278 339
evaluate-data-description-for-
allocation 184 151 156 165 180 181 186
196 238
evaluate-data-description-for-
reference 209 208
evaluate-defined-reference 213 205
evaluate-entry-reference 164 163 300
evaluate-expression 299 140 159 160(2) 162
163 165 167 189 190(2) 194 200 203(2)
216 218 231 242(2) 248 280 290 293 295
300(2) 332(2) 334 339 348(2) 349 354
355 356 366
evaluate-expression-to-integer 295 92(2)
125(2) 142 149(2) 164 184 189 190 210
212 217 220 223 228(2) 230 243 247
265(2) 283 287(2) 330 336 337 338 341
evaluate-file-cption 227 171 228 232 233
235 237 238 240 247(2) 248 265
evaluate-filename 231 229
evaluate-format-expression 287 286(2)
evaluate-format-item 286 285
evaluate-from-cption 241 235 238
evaluate-ignore-option 243 233
evaluate-imag-pv 198
evaluate-in-option 182 181
evaluate-intix-expression 303
evaluate-into-option 242 233
evaluate-isub 300
evaluate-isub-defined-reference 215 214
evaluate-key-OPtion 242 233 239 240
evaluate-keyfrcm-option 242 235 237
evaluate-keyto-option 243 233
evaluate-named-ccnstant-reference 220 287
299
evaluate-named-io-condition 171(3) 172
evaluate-onchar-pv 199
evaluate-cnsource-pv 200
evaluate-pagenc-pv 200
evaluate-parenthesized-expression 300
evaluate-pointer-set-option 242 233 237
evaluate-prefix-expression 301
evaluate-real-ccnstant 373 135 372(4)
evaluate-real-Pv 203
evaluate-restricted-expression 140(2)
92(2) 125(2) 138 139(3)
evaluate-simply-defined-reference 214(2)
216
evaluate-size 245 234(2) 238 239(2) 244
evaluate-string-overfay-defined-
reference 218 213 214
evaluate-string-pv 202
evaluate-substr-pv 203(2)
evaluate-tab-option 230 228
evaluate-target-reference 194 (3) 159 198
243 266 280
evaluate-title-option 231 228
evaluate-unspec-pv 204
390
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 409

evaluate-value-reference 299 164 166 207
227
evaluate-variable-reference 205 165 181
182(2) 192 193(2) 194 198 201 202 203
204 213 237 241 242f2) 254 281 287 300
318 330 336 338 360 365 3'66
fevaluated-condition, 145(2) 171(2)
172(2) 173 174
<evaluated-data-description3 14714) 144
146(2) 151 156 165 175(3) 176 180
181(2) 182(2) 183 184(3) 185(2)
186(2) 189 192 196(5) 197 199(3) 201
202(4) 203(2) 204 205 206 207 208(3)
209(2) 210(2) 212 213(3) 21413)
215(S) 216 217 218(2) 225 233 234(2)
235 239(2) 243 244 245 260 261(2)
281(3) 282(11) 283 307 308 330 336 338
366(2)
<evaluated-delete-statement+ 240(2) 241
244
<evaluated-entry-reference> 144 148 154 155
163 164(3) 173 300
fevaluated-file-description, 145(2)
228(6) 229(7) 233 234(2) 236(2) 237
239 240 247(2) 248 265 339 348
*evaluated-from-option> 24X(3) 235(2) 236
238(2) 239 (2)
*evaluated-ignore-option, 243(3) 233(2)
234 245(2)
<evaluated-initial-element, 189(5) 190(2)
fevaluated-initial-items 189 (2) 190(3)
fevaluated-into-option, 242(3) 233(3) 234
<evaluated-io-condition, 145(2) 169
171(3) 172(2) 173 174 227
*evaluated-iteration-factor, 189(3)
190(2)
<evaluated-keyfrom-option, 242(2) 235(2)
236(2) 237(3) 243
*evaluated-keyto-options 243(3) 233(3)
234(3) 235(3)
devaluated-linesize, 145(2) 228 229(3)
268 269 271 275 276
fevaluated-locate-statements 237(2)
*evaluated-pagesize, 145(2) 228 229(3)
275 277 278
*evaluated-pointer-set-options 242(3)
233(3) 235 237(4) 238
fevaluated-pseudo-variable-
reference+ 147(2) 195 196 198(3)
199(3) 200(4) 201(4) 203(3) 204(2)
266 280(2)
<evaluated-read-statement, 233(2) 234 244
245(2)
*evaluated-rewrite-staterent> 238(2) 239
244
tevaluated-tab-options 145(2) 228 229(3)
230 231 267 268 276 277
<evaluated-target> 147 144(2) 159 162(2)
181 186 189 190 194(S) 195(2) 196 234
235 238 243(2) 249 255 258 259 266
280(2)
<evaluated-title, 145(2) 228 229(2) 230
231(S)
tevaluated-write-statement+ 235(2) 236
everv-bif 334
<eve&bif> 59
sexecutable-single-statement* 36(2)
34(3) 69 98 106 107
{executable-unit? 34(3) 33 69 72(2)
97 98(2) 105(2)
, <executable-unit> 54(5) 55(2) 96
97(4) 98(6) 105(3) 106(2) 107(Z)
144 146 148(4) 154(2) 155(6) 157(22>
158(4) 162 163(4) 166(7) 167(2)
168ClO) 169(2) 170 222(3) 246 253(2)
execute-allocate-statement 180 166(2)
execute-assignnent-statement 194
execute-begin-block 158
execute-call-statement 163
execute-close-statement 232
execute-delete-statement 240
execute-end-statement 168
execute-executable-unit 157(3) 155
execute-free-statement 191
execute-get-file 247(2)
execute-get-statement 247
execute-get-string 248 247
execute-goto-statement 166
execute-group 158
execute-if-statement 163
execute-input-control-format 258(2)
execute-input-data-format 259 258
execute-locate-statement 237 166(2)
execute-null-statement 167
execute-on-stateKent 171
execute-open-statement 228
execute-output-control-format 271 270
execute-output-data-format 272 270
execute-put-file 265(2)
execute-put-statement 265
executequt-string 266 265
execute-read-statexent 233
execute-return-statement 167
execute-revert-statement 172
execute-rewrite-statement 238
execute-signal-statement 170
execute-single-closing 232(3)
execute-single-0Fening 228(2)
execute-stop-statenent 153
execute-write-statement 235
exit-from-i0 246 227 238 240 244
exp-bif 334
<exr;-bif> 59
expand-edd 282(4) 281
expand-generaticn 282 280(3)
expand-like-attribute 76 75
expand-list-of-Subscripts-
lists 217 216
expand-name-and-subscript 284(3) 283
$exFonentp 47(3) 37312)
{expression) 44(4) 35 37(2) 38
40(6) 41(7) 42(4) 43(15) 45
52(2) 63 68 69(2) 92 94 96(3)
lOl(2) 102 104(3) 105 106 109(3)
llO(4) 112(4) 114 115(4) 116 121(3)
123
<expression> 58(15) 28 51 52(3)
54(5) 55(3) 56(8) 57(7) 53(2)
92(2) 96 lOl(2) 104 105 106 114
115(6) 116(b) 121(2) 123 124 125 127
137 138 139(3) 140(3) 141 142(2)
156(2) 158(S) 159(2) 160 161 162
163(2) 164(3) 165 167(2) 175 184(3)
189(2) 190 (2) 194 196 198 201 202 203
204 210(2) 212 216 217(2) 218 220
223(3) 228(2) 230(2) 231 242(2) 243
Index 391
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 410

<expression> (continued)
247 248 253 265(2) 280 283 286(5)
287(3) 291 293 295 299C3) 300(3)
301(2) 303(2) 315 318 319 330 332 336
338 345 348 349 356 360 363 374
sexpression-five) 44(4) 115(3)
{expression-four) 44(4) 115(3)
{expression-one) 44(2) 115(2)
{expression-seven* 44(4) 115(3)
*expression-six) 44(4) 115(3)
{expression-three* 44(4) 115(3)
Cexpression-two) 44(5) 115(2)
EXT 50
Cextent-expression? 37(5) 74 90 91(2)
92 110
<extent-expression> 52(3) 53(2) 125<2)
137 138(3) 139C3) 142(2) 147 156
164(2) 165(2) 175(2) 184(5) 186(2)
204 209 212(2) 214
EXTERNAL 37 39 50 71 84(2) 86(2)
93C3) 107
<external> 51 107 138 142 143(3) 149C2)
150(6) 151(4) 173 185(2) 220 223(2)
229
extract-slice-of-array 219 212 215 223
{extralingual-character) 48(5) 47 49
251
tfail* 138 140(5) 181(3) 182(2) 183(2) -
192(2) 193(2) 200(2) 228 229(2) 230
234 236 237 239 240 247(2) 248 265
315(2) 345 346(3) 34713)
*false> 82(2) 83 84(6) 85(4) 87(2)
88(8) 92C4) 124 125(8) 126 127(6)
128(6) 129(9) 130(4) 141(8) 159(4)
161(6) 162(3) 163<4) 168 218(2)
382t4)
field 374
{field-element-l) 257(2)
ifield-element-2j 257(3)
FILE 37 38 39 41 68(2) 80 85(2)
86(5) 108 112 129
<file> 53(2) 108 112 149 200 221 339 348
<??i&descriotion> 53(2) 145 150 229
$file-description-set)
108
85(2) 93
*file-directory, 143(21 149(2) 223(2)
tfile-directory-entry+ 143(2) 149 150(2)
223 229
ffile-information> 145 143(2) 146 150(4)
151 169 173 193 194 201 223 226(2)
228(2) 229C2) 232(2) 233(2) 234
236(2) 237 239(2) 240 241 244(2) 246
247(2) 248 250(2) 255 256 258 263 265
267 268 271 275 276(2) 277(2) 278(3)
279 339 348
*file-opening+ 145(2) 226(g) 228(2) 230
232 235 238 241 245
{file-option) 41(6) 42(5) 68(6) 80
<file-option> 56(g) 57 106 225 226(3)
227 228 232 233 235 237 238 240 247 265
*file-value> 146<2) 144 145(2) 151 169
171 173 174 195 201 223 226(3) 227(33
228 229 232(2) 233(3) 234 235(2) 236
237(2) 238(2) 239 240(2) 241 244(2)
*file-value> (Continued)
246 247(2) 248 249(2) 250(3) 252 254
255(3) 256(2) 257 258(2) 259 260 261
263(2) 264 265 267(5) 268(4) 269(2)
270 271(2) 272 275(4) 276(37 277(3)
278(2) 279 339 348
*filename+ 145(2) 150 169 173 227 229(2)
231 233
find-applicable-declaration 78(2) 72 77
81 82 83 88 89(2) 91 92 100
102(2) 103 105 106 114 117 126
find-block-state-of-declaration 187 185 205
213 220 221 222
find-by-name-parts 103(2) 102(2)
find-directorv-entrv 185 156 180 191 205
318 319 * -
find-fully-qualified-name 79(4) 92(2)
114 117
find-item-data-description 176(3) 177(2)
183 184(2) 185 186 196 197 203 206
209(2) 308
FINISH 40 100
<finish-condition> 55 145 153 167 168 174
first [tree1 18
<first-comma* 144(2) 145 226 230 248(2)
250(4) 251(4)
FIXED 33 37 38 39 70(3) 85 86(3)
94 113 129 135(2)
<fixed> 53 113 133 135(2) 160(2) 260(4)
262(2) 273 274 235(2) 296(4) 298(3)
299 304 305 310 312(3) 313 315(3) 324
326 335 336 341 342 343 349(2) 351(3)
356 360 363 364 366 367(2) 368(4)
369 (2) 371(2) 372 373 375 380
fixed-bif 335
<fixed-bif> 59
{fixed-point-format+ 43(2)
<fixed-point-format> 57(2) 262 273(2)
274
Cfixed-point-picture) 132(2) 133(3)
<fixed-point-picture> 60(2) 133 374 381
FIXEWVERFLOW 36 50
<fixedoverflow-condition> 55 173 298C2)
299
FLOAT 33 37 38 39 85 86(2) 129
135(2)
<float> 53 113 133 135 160 260(2) 274(2)
295c 2) 298(2) 299 304 305 310 312(3)
313 317 319 320 321(2) 324 326 327
328(2) 332(2) 333(2) 334 335 336 339
340(2) 341 342 343 349 351(3) 352
353(2) 354 356 357(2) 358 360 363
364(2) 367 368(2) 369(2) 370 373 375
380
float-bif 335
<float-bif> 59
ifloatins-point-format> 43(2)
<floating-boint-formats 58 57 262 273(2)
274(2) 370
*floating-point-Ficturep 132(2) 133(2)
<floating-point-Ricture> 60(2) 134
floor 28
floor-bif 336
<floor-bif> 59
FOFL 50
follow 17
for each [instruction1 27
form 20
392
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 411

FORMAT 37 38 39 43 73 85(2) 92(2)
93(3) 108 112 129
<format> 53(2) 108 112
*format> 221
*format-control, 144(2) 169(2) 246
258(5) 270(2) 271(3) 285(2) 286(E)
287(S)
{format-item) 43(3) 104 131
<format-item> 57(2) 104 258 270 285(3)
286(5) 287 374(4) 382(2)
<format-iteration) 43(2) 94 104
<format-iteration> 57(2) 104(2) 286
{format-iteration-factor* 43(2) 104
<format-iteration-factor> 57(2) 104 286
*format-iteration-index, 144(2) 285 286
<format-iteration-value, 144(2) 285 286
<format-list-index, 144(2) 258(2) 270 271
285 286 287
$format-specification* 43(4) 42 104(2)
<format-specification> 57(4) 58 104(4)
144 258(4) 270(2) 271(2) 285(2)
286(4) 287(3)
#format-statement* 43 33 36 68 72(2)
96 104
<format-statement> 58 51 54 96(2)
104(2) 144 146 169(2) 221(3) 286(2)
287(3)
*format-value> 146(2) 221 287(2)
free 193( 2) 156(2) 175 191 232 234 236
238 240 241 246
FREE 41
free-based-storage 192 175 191
free-controlled-storage 191(2)
*free-statement) 41 36
<free-statement> 55 54 175(2) 191(2)
*freed+ 146 192(2)
<freeing* 41(2) 82 94 105
<freeing> 55(2) 105(2) 191(3) 192(2)
193
FROM 42
*from-option* 42(3)
<from-option> 56(3) 235 238 241
G
<s> 50 115(2)
generate-aggregate-result 293(3) 301(2)
302(2) 303 304(2) 305(2) 306 308
309(2) 310(2) 311(2) 312 313 316
317(2) 318 319 320 321(2) 322(2)
323(2) 324 325 326(2) 327(2) 328(2)
329 330 331 333(2) 334 335(2) 336 337
338 339(2) 340(2) 341 342 343 344 345
348(2) 350(2) 351 352(2) 353(2) 354
355 356 357(2) 358 359 360 361(2)
tqeneration, 147(3) 143(3) 144 145 146
151 156(3) 165(2) 175(7) 176(2) 177
179 180(5) 181(E) 182(6) 183(S) 184
186(3) 188(3) 189(3) 190 191(2)
192(3) 193(5) 194 195(7) 196 197(2)
198(2) 199(3) 201(2) 202(5) 203(3)
204(3) 205(12) 206(2) 207(7) 208(3)
209(3) 210(3) 211 212(4) 213(9)
214(15) 215(6) 216(3) 218 (9) 226 232
233(2) 234(2) 235(3) 236(3) 238(3)
239(2) 240 241(3) 242(4) 243 246 254
*generation3 (Continued)
266(2) 280(10) 281 282(4) 283 287
289(2) 291 292 300 318(2) 319 (2) 330
336 338 360 365 366(4)
GENERIC 38 39 84(2) 93 96
Cgeneric-attribute? 38 37 82 83(2) 117
118 126(2)
<generic-data-attribute* 38(Z) 70(Z)
84 85 128(2) 129 (4)
<generic-description* 38(2) 63(2) 66
70 75(3) 76(2) 77(2) 78(2) 82
83 84(2) 110 112 113 126(E) 127(4)
128(3) 131(2)
*generic-element) 38(2) 126(6)
Cgeneric-Rrecisicn) 38(E) 129 130
GET 42
get-data 252 248(2)
get-edit 257 248(2)
get-established-cnvalue 315 200(3) 345(2)
346(3) 347(3)
*get-file) 42(2)
<get-file> 56(2) 247(2)
get-list 249 248(2)
*get-statement2 42 36 68(2)
<get-statement> 56 54 175 247
*get-string* 42(2) 68
<get-string> 57 56 247 248 374
go to [instruction] 27
GOT0 40(2)
Cqoto-statement) 40 36
<goto-statements 55 54 156 157 166
fqreater-than, 306 307(3) 308 309
Cqroup) 34(3) 94 97 106(2)
<grou$ 54(2) 97(4) 157 158 175
fqrouu-control, 144(2) 154 155
<&> - 58 115
H
hbound-bif 336
<hbound-tif> 59
hiqh-bif 337
<hiqh-bif> 59
high-level-parse 66 36 64
I
iidentifizr8) 47(3) 36(2) 39(2)
40(2) 41(3) 44(2) 60 65 66(5)
71(5) 72(4) 74(2) 78(7) 79(12)
80(2) 81(3) 82(2) 83 88(7)
89(2) 90 92 94 99 100 103(2) 105
106 107 lil 114(2) 117 129(2) 135
252(4)
<identifier> 60 51 52(2) 54(2)
59(2) 99 103(2) 107 ill(2) 114(3)
117(6) 120(4) 122 123 124 125 138(3)
142(2) 143(4) 144(2) 149 (2) 150(5)
151(3) 155(2) 156 164(2) 173 184
185(4) 188(3) 205(3) 207(2) 208 209
210(3) 211 213t4) 214 218 220(4)
221(2) 222(2) 223(2) 252(3) 253(4)
254(10) 268 283(3) 284(3) 285 289 319
Index 393
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 412

identifier-to-d&name 285 283(2) 284
if Cinstructionl 27
IF 35
$if-clause) 35(2) 34(2) 105 106
*if-statement) 34(3) 35 94 105
<if-statement> 55 54 105(3) 106 157 163
IGNORE 41
<ignore-option) 41(2)
<ignore-option> 56(2) 233 243
imaq-bif 337 195
<imaq-bif> 59
imaq-pv 198
<imaq-pv> 59 195 198 280
Cimaqinarv-constant> 47(2) 38 66 136
372(3j
immediate component 17
immediate subnode 17
immediate subtree 17
immediately contain 17
immediately follow 17
immediately precede 17
implementation 49 232 236(2) 238(2)
244(2) 314
implementation-defined 48(3) 53(2) 65
86 108 113 142(2) 164 165 173(2)
174(2) 20412) 225(4) 229(3) 230
231(2) 232 240(2) 244(5) 245 268
274(2) 295 325 346 351 358 360 370
379(2)
implementation-dependent 95 99 173 174
298(2) 299 367(2) 371
in any order 25
IN 41
<in-option> 41(3) 80 103(2) 105(2)
<in-option> 55(3) 103 105 182(3) 192
193 (3)
+includeP 48 47 65
INCLUDE 48
INDEX 338
index-bif 338
<index-bif> 59
infix-add 304 290 298
infix-power 312
infix-and 304
infix-cat 305
infix-divide 305 298
infix-subtract
infix-eq 306
<infix-expression>
313 298
58(2) 115 298 303(3)
infix-qe 308
infix-qt 309
infix-le 309
INIT 50
infix-lt 310
infix-multiply 310 298
infix-ne 311
<infix-operator> 58(2) 115(3) 140
303(5)
infix-or 311
initialize-array 189 188
initialize-generation 188(2) 151 156 180
238
initialize-interpretation-state 149(2) 148
initialize-refer-options 186 181 238
initialize-scalar-element 189 188
initialize-spec-options 159(2) 161
INPUT 37 39 41 85(2) 86 108
<input> 53 56 108 145 226 22913) 230(2)
234(2) 247(2) 278
{input-specification) 42(3)
<input-specification> 57(2) 56 248
input-stream-item 263 250 251(3) 256(3)
257(4) 259(2) 264 278
input-stream-item-for-edit 264 259(2) 260
<input-target? 42(4)
<input-target> 57(4) 249 257 258(2)
insert-record 244 232 236(2) 238
<insertion-character> 60(2) 378
instal-arguments 155 154
instruction 27
INT 50
$.inteqer* 47(3) 36(2) 37 43 44 48
60 66 73 89 104 116 130(6) 131(2)
134 135 252 263
<integer> 60(3) 53t2) 116 130 217
integer-type 295
*integer-value) 146(2) 52(2) 57(3)
58(10) 59 92 125 142 143 144(4)
145(6) 147(3) 149(3) 164 165(2) 170
173 175(2) 176 183(3) 184 (9) 186(4)
189(2) 190(2) 192 196(2) 197(2) 201
204 206(5) 209 210(3) 212(3) 214(4)
215(2) 216(2) 217(3) 219 (2) 220(2)
223 226 228(2) 229 (3) 230(3) 231 233
234(2) 238 239(2) 243(2) 244 245(4)
247 252 253(3) 254 258(2) 259(7) 260
261 262(2) 263 264 265(2) 266 267(2)
268(3) 269 270(2) 271(6) 272(S) 273
274(3) 275(2) 276(7) 277(S) 278(S)
279(3) 283 284(3) 285 286(5) 287(5)
289 292 295(2) 315(2) 339 348 365 373
380(2) 381(2) 382
INTERNAL 37 39 50 71 74 84 86
92(2) 107
<internal> 51 107 185 220
interpret 148 32 143 153 293 313
interpretation
Cio-conditionT
23
interpretation-phase
40(2) lOO(3)
32 28 31
*interpretation-state+ 143 31(2) 148
<io-condition>
149(2) 166
interrupt
55(2)
172 153 154 169(6)
lOO(3) 145
171(3)
171
174
INTG
226
41
Cinto-option)
227
41(2)
<into-option> 56(2) 233 242
<invalid> 261(7) 26014) 262(3) 263
*initial) 38 37 52 96 97(2) ill(2)
INITIAL ‘38 39 50 84 86 93(4)
<initial> 52(3) 97 122 125 137 138
142(2) 151 156(2) 180 188 208
<initial-constant-one) 38(2) 114
qinitial-constant-two) 38(3) 115
<initial-element) 38(3) 94 114 115
<initial-element> 52(S) 114(4) 115(2)
137 139 142(4) 189(8)
$.isubp 48 44 47 66 116
<isub> 60 58 116 125 139 213 214 217 299
300(3)
<item-data-description> 52(5) 101 111
116(2) 117(2) 119(4) 120(2) 121 122
128 138(3) 141(2) 142(2) 162 176(7)
177(4) 181 183 184(2) 185 186 188(3)
189 196(2) 197(2) 202 203 204 206
209( 2) 250 253(2) 254 255 260 261(2)
394
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 413

<item-data-description> (Continued)
282 284(3) 289(3) 292 308 332 349 356
360
<iteration-factor) 38(2) 114
<iteration-factor> 52(3) 142 189(2) 190
<iterative-group> 54(2) 97(2) 166(2)
K
KEY 40 41
tkey3 147(2) 145 225(2) 226 230 232(4)
233(3) 234(5) 236(5) 237 238(4)
239(10) 240(7) 241(3) 242(2) 243(2)
244(6) 245(4)
<key-condition> 55 171 226 227(2) 232
236(2) 238 244 245
fkey-option) 41(3) 42
<key-option> 56(4) 233 239 240 242
KEYED 37 39 41 85(2) 86 108
<keyed> 53 56 108 145 229(2) 230(3) 234
236(3) 237(3) 238(2) 239 240 244(2)
<keyed-dataset, 147(2) 230
*keyed-record, 147(3) 226 230(2) 233
234(2) 235(2) 238 239 240(3) 241(2)
243(2) 244(4) 245(3)
*keyed-sequential-dataset, 147(2) 230
RBYFRCM 42
skeyfrom-option* 42(2) 41
<keyfrom-option> 56(3) 106 235 237 242
KEYTO
{keyto-oG:on) 42 41
<keyto-option> 56(2) 233 243
keyword 65
known [operation1 29
LABEL 37 38 39 73 85(2) 92(2)
93(3) 108 112 129
<label> 53(2) 108 112 222
*label-value, 146(2) 166(2) 222
last [tree1 18
lbound-bif 338
Ubound-bif> 59
<le> 58 115(2)
&ading-delimiter? 251(4) 250(2) 256(S)
257(5)
length 297
lenath-bif 339
<length-bif> 59
<less-than+ 306(2) 307(2) 309 310
let [instruction3 27
(letter) 47(4) 39(2) 48 66 88(3)
251
<level* 36(2) 37 38 71(2) 72 76(10)
77(8) 78 79(2) 89(2) 90 93(2)
103(2) ill(2) 127(3) 128(2)
LIXE 37 75(2) 76(6) 77(2) 82
LINE 42 43
{line-f ormat) 43(2)
<line-format> 58(2) 258 271
Cline-option) 42(2)
<line-option> 5'(3) 265(2)
flinemark> 147 251(4) 256 257(4) 259(2)
263(2) 264 275(2) 277(2) 278(6)
279 (2)
lineno-bif 339
<lineno-bif> 59
LINESIZE; 41
Flinesize-option? 41(2)
<linesize-option> 56(2) 228
*linkage-part+ 144 143 154 155 156(2) 167
168 169(2) 170 185
LIS? 42 43
Clist-directed-input> 42(2)
<list-directed-input> 57(2) 159 248(3)
249
{list-directed-output? 43(2)
<list-directed-output> 57(2) 159 265 266
267
LOCAL 37 39 85(2) 112
<local> 53(2) 112 122 125 138 142 166 208
287
local-goto 166(3)
local-tree 25
local-variable-name 25
LCCAIE 41 106
*locate-statement+ 41 36 82 94 106
<locate-statement> 56 54 106(2) 175(2)
226(2) 237(2)
{locatorg 85(2)
<locator> 53(2) 102(2) 112 119 158(2)
181 182 306(2) 311(2)
$locator-qualifier? 44(2) 41 81 102
105(2) 118 119
<locator-qualifier> 59(2) 55 57(2) 101
105 119 192(2) 193(3) 207(2)
log 28
log-bif 339
<loq-bif> 59
loqlO-tif 340
<loqlO-tif> 59
log2-bif 340
<loq2-tif> 59
low-bif 341
<low-bif> 59
low-level-narse 64(2) 65
flower-bound) 37(2) 74 llO(4)
<lower-bound> 52(2) 110 149 176 184 196
210 212(2) 214 215 216 217 219(2) 220
222 282 284 290 292 330 338
<1t> 58 115 -
M
fmachine-state> 31(3) 25(3) 29(4) 30
32 148 149 175 185 225 230
make-allocation-unit 185 182 183
make-name-and-subscript-list 283 281
rax 28
max-bif 341
aax-bif> 59 314
$maximum-length? 37(3) 112
<maximum-length> 53(2) 112 122 125 134
135 137 138(3) 163 165(3) 178 184 186
192 197 202(2) 204 206 209(2) 218(2)
231 234 242(2) 248 250 255 261 266(2)
267 269 272(2) 307 308(2) 365(2) 366
Index 395
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 414

MEMBER 37 38 39 76(7) 77I2) 78
79(2) 82 84(2) 85 86 93(2) 96
103(2) 105 106 111 113(2) 117(2)
126(3) 128
*member-aggregate-type> 146(2) 289 290(6)
291(3)
-ember-description> 52(2) ill(2)
117(2) 120(3) 138(4) 176(3) 184
188(4) 207 208(3) 210 211(2) 213(2)
214(2) 253(2) 284 289 292
middle-level-parse 65 64
min 28
min-bif 342
<min-bif> 59
<minus> 132(6)
ininusj 58 116
mGGZf 343
<mod-bif> 59
<mode> 53(2) 113 133 159 160 185 198(2)
199 20113) 260 295(4) 296 301 302
304(2) 305(2) 306(3) 307 308 309(2)
310(3) 312(12) 313(4) 316(5) 317(7)
319(3) 320t4) 321(4) 322(2) 324(2)
326(2) 327(2) 328(4) 330(2) 331(6)
332(3) 333(4) 334(2) 335(4) 336(2)
339(5) 340(2) 341(2) 342(2) 343(3)
344(4) 348(2) 349(2) 351(g) 352(4)
353(4) 354(2) 356(6) 357(4) 358(2)
360 363(3) 366 368 373 374 375(2)
376(2) 380(2) 381(2)
modify-statement-names 69 68
multiple-constraint 140(4) 141(4)
<multiply> 58 115 140 303
multiply-bif 344
Unultipiy-bif> 59
must not [instruction1 28
must [instruction3 28
N
Cn-3 71(3) 72 7317) 74
NAME 40 41 101
fname-and-subscript> 283(5) 281(2)
284clO)
-ame-condition> 55 170 174 226 227 255
{named-constant) 85 84
<named-constant> 53 51 108(3) 118 122
137 149 164 220
<named-constant-reference> 59(2) 58 118
121(2) 122(2) 140 141 220(2) 223(2)
287 299
warned-io-condition? 40(2) 80 100
<named-io-condition> 5S(2) 100 169 170
171(4) 172(3) 174 227
<ne> 58 115
n& [tree1 18
NOCONV SO
NOCONVERSION 36 50
node 17
NOFIXEDOVERFLOW 36 50
NOFOFL SO
<non-blank-comma) 251(3) 249 254
$non-blank-comma-quote) 251(3) 249 254
{non-computational-type) 85(2)
<non-computational-type> ' 53(2) 102(2)
112(5) 122 141 142 158(2) 250 255 268
<non-CorrFutaticnal-type> (Continued)
306(2) 307 311(2)
{non-delimiter? 47(2) 64 65(4)
{non-drifting-field? 132(2)
<non-iterative-group> 54(2) 97 158 168
non-terminal 21
NONE 39 87
<none+ 94(2) 95 249(2) 258(2) 267(2)
269(2) 270(2) 279(2) 281
NONVAR 50
NCNVARYING 37 38 39 50 85 86(2) 128
129
<nonvarvinq> 53 112 122 128 134 135 138
178(2) 183 184 186 192(2) 202(3) 204
206 208(2) 218(2) 231 234 242(2) 248
250 255 266 267 269 272 365 366
NOOFL 50
NCOVERFLCW 36 50
normal-sequence 157I2) 158(3) 163(2) 167
168(2) 170 171 172(2) 180 131 194 228
232 234 236 238 239 240 246 247 265
NOSIZE 36
NOSTRG 50
NOS'IRINGRANGE 36 50
NOS!IRINGSIZE 36 50
NOS'IRZ 50
NOSUBRG 50
NOSUBSCRIPTRANGE 36 50
<not> 58 116
<z-dummy+ 144 165
*not-equal+ 306(4) 307(E) 308(41 309(2)
310 311
fnotrin, 91(3)
NCUFL 50
NOUNDERFLOW 36 50
NOZDIY 50
NOZERODIVIDE 36 50
<null, 146(2) 207 307 308 318 345 365(2)
366(2)
null-bif 345
<null-bif> 59 140 314
<null-bit-strinq+ 134 135 146 178 186 198
264 297(2) 302(2) 304 311 334 368(2)
370 371
fnull-character-strinq, 134(2) 146 170(2)
171 178 186 198 231 249 251 25213) 254
256 257 260(3) 262(2) 263(2) 264 266
276 297(2) 337 341 346(2) 347(3) 361
368 370
Cnull-statement* 40 36 69(2)
<null-statement> 54 lOl(4) 167
null-string 297
Cnumber-of-digits* 37(3) 38(2) 130(5)
<number-of-digits> 53(2) 113(2) 130
133(2) 135 138(3) 160(6) 260(4)
262(2) 273 274(3) 295 296(g) 298(2)
303(2) 312 314 315(3) 316(2) 324(2)
326(3) 336(2) 341(3) 342(3) 343(2)
349 (3) 351(3) 356 360(2) 366 367
368(2) 369I2) 370 371 372 373(3)
375(2)
number-of-scalar-elements 289
<numeric-picture-element> 60(4) 133(2)
374 375(3) 376(5) 377 379 380(2) 381
@nnr,eric-picture-specification2 132 133
<numeric-picture-specification> 60(2) 133
374(6)
<numeric-string+ 372(4) 371
396
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 415

0
<occupancy+ 146(2) 183 308
<off, 144 251 376 377(5)
OFFSET 37 38 39 80 85
112 129
<offset> 53(2) 55 96
125 138 139 142 158(4)
193t2) 197 205(3) 207
311(2) 345(2) 348(2)
offset-bif 345
<offset-bif> 59
<offset-value> 146(2) 307
366(2)
OFL 50
foperation, 31(6) 29(15) 30 144 153(2)
154(2) 155(2) 156(2) 166(10) 167(2)
232(4) 244(2) 246(3)
optionally 25
378(3) Captions) 38 37 39t2) 75(2) 90(2)
90(2) 91(4) 91 95 99
CFTIONS 38 39 85 113
102(2) 112 119 <options> 53(2) 54(2) 95 99 113 122
164 182(2) 125 142(4) 164t4)
208 306(2) 307 (options-specification) 38(2) 65 91
363(3) 365 366 <or> 58 115
OEPUT 37 39 41 85(2) 86 108
<output> 53 56 108 145 229(6) 230 232
308 363(3) 365 236(4) 237(2) 247(2) 248(21 265(2)
278
{output-source> 43(4)
*omega+ 147(3) 225 226(2) 245(3) 2S9(2) <output-sources 57(4) 267 270(2) 271
263(5) 264(2) 278(2) Coutput-specificationg 43 42(2)
ON 34 35 <output-specification> 57(3) 265
fE, 144 230 248(2) 250(3) 251(3) 376 output-stream-item 276 27513) 277(2)
377(4) 378(3) 278(4)
son-statement? 34(3) 94 106(2) 107 output-string 275 268(3) 269 270(4)
<on-statement> 54(2) 107(2) 171(2) 227 271(2) 272(4) 273(2) 277
Con-unit) 34(2) 107 output-string-item 275(2) 263 267 269
ton-unit> 54(2) 107 154(2) 172 270(2)
onchar-bif 345 195 output-tat: 276(2) 277
<onchar-bif> 59 314 CVERFLCW 36 50
onchar-pv 199 <overflow-condition> 55 169 173 298(2)
<onchar-pv> 59 194 195 199 266 280 299t2) 367
fonchar-value, 145(2) 170 200 227 266 315 overlay-strings 208(2) 218
345 365 369 372
oncode-bif 346
<oncode-bif> 59 314 foncode-value, 145(2) 173 315 346 P <one-bit, 136 146 163 302(2) 304(2) 306
307 308 309(2) 310 311(2) 329 334(2) PAGE 42 43
354(2) 361 368(2) 370 371 <m> 57(2) 58 258 265 271
onfield-bif 346 *page-number3 145(2) 201 226 230 276 348
<onfield-bif> 59 314 fpaqemark, 147 225 230 276(2) 277(2)
fonfield-value> 145<2) 170 227 315 346 278(2) 279(2)
onfile-bif 346 pageno-bif 348 195(2)
<onfile-bif> 59 314 <paqenc-tif> 59
tonfile-value, 145(2) 169(2) 173(2) 227 pageno-pv 200
315 346 <pageno-pv> 59 194 195 200 280
onkey-bif 347 PAGESIZE 41
<onkey-bif> 59 314 Cpagesize-opticng 41(2)
fonkey-value, 145(2) 171 227 315 347
onloc-bif 347
<nasesize-outicn> 56(2) 228
PARSPETER - 37 39 50 72(2) 82 84 86
<onloc-bif> 59 314 89 93(3) 109
fonloc-value, 145(2) 173 315 347 <parameter> 52 109 137 155 185 205
onsource-bif 347 195 <parameter-descriptor> 53(2) 113(2)
<onsource-bif> 59 314 124(2) 125(2) 129(4) 137 139(2)
onsource-pv 199 164(2) 214 218
<onsource+> 59 194 195 200 266 280
fonsource-value, 145(2) 156 170 200(2)
227 266(2) 315 345 347 365 369 372
open 229 225(2) 228 234 236 237 239 240
247(2) 248 265
OPEN 41
fopen, 145 151 201 226 228 230 232 233 234
236(2) 237 239(2) 240 241 247(2) 248
265 339 348
<open-state* 145(2) 150 230 232
<parameter-directory* 144 143 154 155 185
193 205
*parameter-directory-entry, 144(2) 155(3)
156 185 205
Cparameter-name? 39(2) 72(2) 89(2)
90 99
<parameter-name> 54(2) 32 99 155
(parenthesized-expression) 44(3) 38 114
115 116
<parenthesized-expression> 58(2) 52
sopen-statement) 41 36 114(2) 115 116 137 141 189 190(2) 300
<open-statement> 56 54 228 PARE? 50
operand 25 parse 64(5) 33 63 68(4) 69t2) 87
operand-name 25 250 251 256(3) 257(2)
operation 24 parse-data-input-name 255 252
Index 397
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 416

__- _-.- --
parse-data-input-valw 256 252
parse-list-input 25012) 249
partial [tree] 23
perform [instruction1 27
Pi 28
PIG 50
spit-digit) 132(10) 133(2)
{pit-exponent> 132(2)
{pit-mantissa) 132(2) 133
<pit-status, 376 377
{picture) 37(2) 38 43 94 112 129
131(5) 274 275(2) 369(3) 370
PICTURE 37 38 39 50 85 86(2) 93 112
{picture-content) 131(2) 133
{picture-element) 131(5) 133(2) 369(2)
<picture-exponent> 60(2) 134 374 375 376
380 381
Cpicture-format) 43(4)
<picture-format> 58(3) 57 259 260
261(3) 272 380(2) 382(2)
<picture-invalid, 380(7) 261 262 361
381(4) 382
<picture-item? 131(4) 133(4)
<picture-mantissa> 60(2) 134 374 375(2)
380 381
{picture-scale-factor) 131(2) 133
<picture-scale-factor> 60(2) 133 374
4picture-valid, 380(7) 261 361 365 368(2)
371 381t4) 382
picture-validation [syntax] 132
fpicture-validity, 380(5) 261 262 361 365
368 371 381(5) 382(3)
$picturedg 8512)
<pictured> 53(2) 58 112 129(4) 131(4)
138 178(4) 183 184 185 186(2) 197
202(2) 208 218 272 273(2) 361
<pictured-character> 60 53 58 131(2)
262 267 269 272 296 307 361 365 368 370
371 382(Y)
<pictured-numeric> 60 53 131(2) 133(2)
198 199(3) 201 202(2) 261 273t4)
295(2) 296 306 308 309(2) 310 361
363(5) 364(2) 366(2) 368(2) 369(2)
370(2) 371 374(10) 375 380(2)
*pli-text? 47 64(2) 6513)
$plus) 132(6)
<e> 58 116
POINTER 37 38 39 50 81 85 112 129
<pointer> 53 102 112 158(2) 181 182 197
207 235 237 238 306 307 311 318 345(2)
348(2) 363(2) 365 366
pointer-bif 348
<pointer-bif> 59
#winter-set-option> 41(3)
<pointer-set-obtionj 56(3) 106 226 233
237 242
*pointer-value> 146(2) 175 181 207 235 238
307(2) 318(2) 345 363(3) 365 366(2)
POS 50
POSITION 37 39 50 84 86(2) 93
109 (2)
<position> 52(2) 109 139 213 218(2)
position-file 244 234 239 241
?position-index, 147(2) 178 183 184
197(2) 198 199(2) 204(2) 206(3) 208
209 (2)
<power> 58 115 303
PREC 50
precede 18
{precision) 37(7) 70(3> 113 135
PRECISION 37 38 39 50 70(3) 85
86(4) 93 128(2) 129
<precision> 53(2) 113(2) 129(2) 130 133
135(2) 159 160(4) 162(3) 260(Q)
296(g) 301 302 303(5) 304(2) 305(Z)
306(2) 310(2) 312(3) 313(2) 314
315(3) 316(3) 317(5) 319(2) 320(2)
321(4) 322 324(4) 326(2) 327(2)
328(4) 330 331(3) 332(4) 333(4)
334(2) 335(2) 336(4) 337(2) 339( 2)
340(4) 341(2) 342(2) 343(5) 344(3)
348 350(3) 351(4) 352(3) 353(4)
354(2) 356(6) 357(4) 358(2) 360(3)
368 369(2) 373 374
precision-bif 348
<precision-bif> 59
Cpredicate-expression) 39(4) 87(3) 88
spredicate-expression-onq 39!2) 87
$predf;;;;-ex;$ssion-three* 39(4)
spredicate-ex;;;;;ion;z;T 39(4) 87 88
<prefix? 34(2) 36
69(4) 72 79 83(2) 89 95 98(3)
99(2) 104 106
Cprefix-expression) 44(2) 115 116
<prefix-expression> 58(2) 116 301(3)
prefix-minus 301
prefix-net 302
sprefix-operator? 44(2) 38(2)
<prefix-operator> 58(2) 116(3) 301(3)
prefix-plus 302
Cprefixed-clause) 35(3)
{primitive-expression) 44(3) 115 116
PRINT 37 39 41 85 86 108
<print> 53 56 108 145 201 226 229(6) 230
267(3) 268 269 275 276(2) 27812) 339
348
PROC 50
$procedure) 33(6) 31(2) 34(2) 35
63(4) 64 69(2) 71(4) 72(3)
73(2) 79 81 82 83 86 87 89 95
96(3) 98(2) 135 137
PROCEDURE 39 50
<procedure> 51(3) 32 54(2) 63(2)
95(4) 96(3) 9712) 98 107 137(3)
140(2) 148 154(4) 157 164(3) 167(2)
168 170 220(3) 221 222 253(4)
<procedure-function-reference> 59(2) 101
121 124 125 154 164(4) 300
*procedure-stateaent) 39 33 36 66 69
72 83 89 95 98(2)
processor 29
prod-bif 349
<prod-bif> 59
production-rule 21
<program> 51 31(3) 32(2) 63(2) 142
148 149(2) 150 151 157 298
<program-directory+ 143(Z) 149
program-epilogue 151 148 153 232 244
progran-run 11
*program-state, 143(2? 149 153(3)
fpro;;muer-nared-condition) 40(2) 81
<programmer-named-condition> 55(2) 100
145 169(2) 170 172(2) 173(2) 174(3)
prologue 156(3) 154 155(2) 166(2) 175
398
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 417

*prologue-flaq, 144 156(4) 169(2) 170 185
Dromotable 196
promote-and-convert 196(2) 165 167 199 201
propagate-alignment 78 75
proper for assignment 102
<pseudo-variable 59(2) 124(2) 147
194(2) 195 196(2) 198(8) 199 200(2)
201 203(2) 204
<pseudo-variable-reference> 59(2) 118 119
124(2) 194 195 198
PTR 50
PUT 42
put-data 268 265 266
put-edit 270 265 266
Cput-file) 42(2)
<put-file> 57(2) 265(2)
p&-line 277 265 271
Dut-list 267 265 266
-+-page 277(2) 174 265 271
Cput-statement) 42 36 68(2)
<put-statement> 57 54 265
$put-string) 42(2) 68
<put-string> 57(2) 265 266 374
sputative-data-constant) 257(3)
sputative-list-constant) 251(3)
*putative-name-field) 257 256
R
+radix-factor) 48(2) 43 66(3) 69(2)
94 134 136 264(2)
<radix-factor> 58(2) 94 262(2) 272
raise-condition 169(4) 153 167 168 174 181
182 197(2) 204 212 215(2) 216 223
227(2) 234 236 237 239 240 247(2) 248
250 251 256<2) 257 263 264 265 272 274
276 298(6) 299( 5) 305 312(2) 313 331
355 365(2) 367(3) 368 369 372 376(2)
raise-io-condition 226 228 232 234 235
236(3) 237 238(2) 239 240(2) 244(2)
245(3) 247(2) 248 249 250 255(3) 256
260(2) 261(2) 264 265 275 277 278(2)
RANGE 39 86(3)
*range-specification* 39(2) 87 88
read 234(2)
41
@cad-statement) 41 36
<read-statement> 56 54 175(3) 226
233(2)
REAL 37 38 39 85(2) 86 113 129 131
135(2)
<real> 53 113 131 133(2) 136 158 159 195
--i98 199 201(2) 260 262(2) 273(4)
274(3) 295 (2) 298 (2) 306 307 312(4)
313 316(3) 317<2) 319<2) 320(3)
321(3) 324(2) 326(2) 328(2) 331
333(4) 336(2) 337 339 340(4) 341(2)
342(2) 343(2) 344 350 351(4) 353 354
357 360 363(6) 364(5) 365 366 367
368(2) 369(4) 370(3) 371(3) 372 373
374(21 375(2) 376 380(2) 381
real-bif 350 195
<real-bif> 59
<real-constant) 47<3) 38 66 135 136
372(5) 373
$real-format) 43(4)
<real-f ormat> 57(2) 58(2) 259(2)
260(3) 261(2)
<real-number, 146(S) 28(2) 260(2) 273
real-pv 201
<real-pv> 59 195 201 280
<real-value> 146(2) 135 136(2) 144(2)
160(3) 162(3) 201 253 260(2) 262(3)
263 272 273(3) 274(2) 275(2) 295
298( 5) 319 330 336 338 346 348 352(3)
363(6) 364(4) 365(4) 366(2) 367(S)
368(3) 369(2) 370(4) 371(4) 372(6)
373(3) 374(3) 375 380(4) 381(2)
RECORD 37 39 40 41 85 86 108
<record> 53 56 108 145 226(2) 229(7)
230(4) 234(2) 236(2) 237(2) 239 (2)
240(2)
<record> 147(3) 225(S) 226(2) 230
233(3) 234 235(2) 238 239 240(4)
241(2) 243(2) 244(S) 245(2)
<record-condition> 55 225 226 227 235 238
240 244
*record-dataset, 147(2) 225(2) 226 230
233(3) 235 236 238(2) 240 241 244
(record-set? 85(2)
RECURSIVE 39 69 73 95
<recursive> 51 95 154
REFER 37
*refer-OFtion) 37(2) llO(2) 112(2) 114
<refer-oitionj 52(2) llO(2) 112(2)
114(2) 125 137 138(3) 139(2) 149 181
184 186(5) 208(2) 209(2)
Creference) 44(3) 37(4) 38(2) 40(4)
41(6) 42(5) 43(2) 52(2) 53 63
68 78(2) 80 81(2) 82(2) 90
91(6) 92 94 96(3) 100 10114)
102(3) 105 109(5) 112(2) 116 117 118
119(3) 121 126(2)
*remote-block-state, 144 (3) 169 (4) 187
246 286(2) 287(6)
{remote-format) 43(2)
<remote-format> 58 57 286(3) 287
reorganize 68(2)
REPEAT 40
<repeat-option> 40(2)
<repeat-option> 54(2) 158 162(Z)
frepetition-factor? 131(4)
replace [instruction] 27
replace-concretes 96(2) 63 99 129(2)
$rep;;;a;;z-string-constanq 44(2) 38
result 25
return [instruction1 27
RETURN 40
sreturn-statement* 40 36
<return-statement> 55 54 153(2) 156 157
164(2) 167
*returned-onsource-value, 145 144 156 249
255 260(2) 261(2) 365 369 372
*returned-value, 145 144 167 300
RETURNS 39(2) 85 86 93 113
*returns-descriptor2 39(2) 37 38
74(2) 83 87(3) 90(3) 91 99 129
<returns-descriptor> 53(2) 32 54 98
99 113 125(2) 129(3) 137 139 167(2)
168 214
reverse-bif 350
<reverse-bif> 59
REVERT 40
Index 399
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 418

$revert-statement)
<revert-statement>
rewrite 239(2)
REWRITE 42
{rewrite-statement?
<rewrite-statement>
root-node 17
round-bif 351
<round-bif> 59
40 36
55 54 171 172 227
42 36
56 54 238(2)
S
scalar 293
*scalar3 101 140 146(2) 162 181 183 186
190 199 200(3) 202 204(3) 222(2) 223
250 255 261 289(2) 290(3) 291(2) 318
319 323(2) 325(2) 327 328 329 330(Z)
332(2) 334 336(2) 337(2) 338(2)
339(2) 341(2) 345(3) 346(3) 347(3)
348(3) 349 354 355 356 358 360
scalar-element 175 289
scalar-elements-of-data-
description 176(4) 175 177 182 184 185
196 208(2) 209 210 211 212 215 289
<scalar-facts, 252 253(3) 254(4)
scalar-result 293
scalar-result-type 293
scalar-value 293
<scale> 53(2) 113(21 133(2) 135(3) 159
160 260(3) 295(4) 296(6) 298 301 302
303(3) 304(4) 305(4) 306 310(4)
312(8) 313(4) 314 315(2) 316(3)
317(5) 319 320 321(21 322(21 324(4)
326(6) 327 328(2) 330(2)
332(6) 333(2) 334 335(2)
337(2) 339 341(4) 342(Y)
344(4) 348(2) 349 350(3)
352(2) 353(2) 354 356(8)
360(4) 366 368 373(4) 374
380(2)
Cscale-factor? 37(2) 38(2)
<scale-factor> 53(2) 130(3)
160(3) 262(3) 263 273 274
298 303 312 315(3) 316(2)
331(4)
336(4)
343(6)
351(9)
357(2) 358
375(2)
70 130(7)
133 135(4)
295 296I6)
324(2)
326(3) 336(2) 341(2) 342(2) 343(2)
349(2) 351(2) 356(2) 36OC2) 366 367
368(2) 369 371 372 373 375 380
*scale-type) 47(3) 373
{scaled-digits-field) 13215)
CscopeP 84(21
<scope> 51(2) 107(2) 185
search-file-directory 223 221
select-based-generation 207 205
select-generic-alternative 126 118
select-qualified-reference 210 188 205 213
214 218
select-subscripted-reference 212 205 214
216 218
{sentence? 35(2) 64(2) 66(2)
SEQL 50
SEQUENTIAL 37 39 41 50 85 86 108
<sequential> 53 56 108 145 229(4)
230(2) 234 239(2) 240(2) 241 244(2)
*sequential-datasets 147(2) 230
*sequential-set) 85(2)
SET 41(2)
+set-option) 41(2) 81 103(3)
<set-option> 55(2) 103 181(3)
set-storage 197 165 175 196 199(2) 202(2)
204(2) 234 235(2)
Csign) 132(5)
sign-bif 352
<siqn-bif> 59
tsisn-i, 372(2) 373
*sign-r, 372f2) 373
SIGNAL 40
Csignal-statement) 40 36
<signal-statement> 55 54 169 170 227
$signed-integer) 36(2) 37 66 71
74(5) 88 99 131 133
<signed-integer> 60(2) 53 54 99(2)
130 133 221(2) 222
*significant-allocationP 146(4) 183(3)
192(2) 308(2) 366(3)
@igas+ 132(2)
simple component 17
simple subnode 17
simple subtree 17
$simp;;-CE-;I&;:-constant, 48 44 47
254 255 264
Csimple-character-string-
constant) 48(2) 37 44 47 66 134
249 (2) 254 255 257 264
+simple-string-constant* 44(3) 38
simply ccntain 17
sin-bif 352
<sin-bif> 59
sind-bif 353
tsind-bif> 59
*single-closing) 41(2)
<single-closing> 56(2) 232(2)
*single-opening* 41(2)
<single-cpening> 56(2) 228(2)
fsingle-statement) 36 35(2) 66(3)
sinh-bif 353
<sinh-bif> 59
SIZE 36
<size-condition> 55 170 173 274 367 368
376(2)
skip 278 2r)7
271 272 276
SKIP 42 43
{skip-format)
<skip-format>
Cskip-option)
<skip-option>
265(2)
SNAP 34 35
<snap> 54 107
some-bif 354
259(2) 263 265 268 269 270
277
<some-bif> 59
source-type 366
Cspace-format)
<space-format>
fspec? 40(2)
<spec> 54(2)
162(2)
sqrt-bif 354
69(2)
43(2) 69(2)
58(2) 259 271
42(3) 6912)
56(2) 57(2) 247(2)
107
145 172(2) 173
43(2)
58(2) 25912) 271(2)
144 158(3) 159 (4) 161(11)
<sort-bif> 59
estatement-control, 144(2) 29(4) 153(2)
154 155 156(2) 166(8) 167(2) 246
<statement-name) 36(3) 33(2) 34 35
66 69(10) 70(3) 71 72(3) 73(4)
74 79 83(3) 88 89(2) 98 99(2)
400
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 419

<statement-name> 54(3) 58 98 99(S)
104 106 164 173 220(2) 221(2) 222(2)
STATIC 37 39 84 86(2) 93(2) 109
<static> 52 109 137 142 151 185 205
*static-directory> 143(2) 149 151 185 192
*static-directory-entry, 143(2) 151(3)
185(2)
step 26
STOP 40
stop-program 153(2)
Cstop-statement? 40 36
<stop-statement> 54 153(2) 156 157
STORAGE 40 86 100
{storage-class) 84(2)
<storage-class> 52(2) 109(4) 142(2)
180(2) 191(2)
<storaqe-condition> 55 145 174 180 182
*storage-index+ 147(2) 146 175(2) 176(4)
177(2) 178 182 183(2) 184(3) 186(2)
189 190(2) 192(2) 197 199(3) 201
202(4) 203(3) 204 206(3) 207 208(4)
209(3) 210 212(2) 213(2) 214 215(4)
216(2) 218 219(6) 235 281 282(2) 289
308 366(2)
+Zstorage-type) 84(2)
<storage-type> 52(2) 109 (6) 185 205(5)
213
STREAM 37 39 41 85 86 108
<stream> 53 56 108 145 226(3) 229 (7)
230(3) 247(4) 248(2) 265(2) 276
*stream-dataset, 147(2) 226 230 374
*stream-item, 147(2) 225(4) 226 230
250(8) 251(2) 255(8) 256(12) 257(3)
259(4) 260 263(8) 264(2) 267(4)
268(2) 269(7) 270(2) 271(2) 272(4)
273(2) 275(5) 276(3) 277(4) 278(5)
279
{stream-setp 85(2)
STRG 50
string 297
$string) 85(2)
STRING 42(2) 86
<string> 53(2) 112 138 163 165 178(3)
183 184 186(4) 202 203 204 206 209(2)
250 255 295 296(2) 297 307 318 322 355
365 366
strina-bif 355
<strinq-bif> 59
{string-constant) 44(2)
*string-format) 43(2)
<string-format> 58 57 259 261(2)
<string-io-control, 144(2) 246 248 250
256(2) 263 266(3) 276(2)
*string-limit> 144(2) 266 276
sstring-or-picture-symbol* 48(4) 131
134(5) 136 251 257
strina-Dv 202
<str&&v> 59 280
tstring-symbol-or-linemark, 251(3)
tstrina-tvDe> 53(2) 112(2) 138 163 202
204 2% 255 295 296(2) 297(8) 305(4)
307(2) 318(2) 322 327 329 338 339 350
355(3)
STRINGRANGE 36 50
<strinqranqe-condition> 55 170 173 204 355
STRINGSIZE 36 50
<strinqsize-condition> 55 174 272 365
STRUCTURE 37 38 39 75 76(7) 77
78(2) 82 84(3) 102(2) 103 111(2)
127
<structure-aggregate-type> 146(3) 186 238
289 290(2) 291
<structure-data-description> 52(3)
ill(2) 117(3) 120(5) 121 125 127 128
138(4) 142 176(3) 177 188(4) 207
208(2) 210 211(2) 213 253 282 284(3)
289(2) 292
{structure-qualification2 44(2) 120(2)
S'IRZ 50
SUB 48
suballocate 183 175 181
subnode 17
SUBRG 50
<subroutine-reference> 59 55 94 109 117
118(2) 119(5) 124 125 163 164(4)
$subscript+ 44(2) 118(4) 120(5) 121(6)
123(4)
<subscript> 59(3) 57 101 121(5) 192
205(2) 212(2) 213(2) 214(2) 215(3)
216(6) 217(9) 218 222 223(4) 253(2)
254(2) 283(3) 319
<subscript-ranqe-ccndition> 212
subscript-to-ccmna-subscript 285 283 284
fSUbSCriFt-Value> 143(2) 149(2) 150
223(2)
SUBSCRIPTRANGE 36 50
<subscriptranqe-condition> 55 170 173
215(2) 216 223
substr-bif 355 195
<substr-Hf> 59
substr-pv 202
<substr-pv> 59 195 203(2) 266 280
substring 297
<subtract> 58 115 140 303
subtract-bif 356
<subtract-bif> 59
subtree 17
<succeed, 229 230
sum-bif 356
<sum-bif> 59
<suppression> 376(2) 377(9) 378(6)
*suppression-type+ 376
*suppression-type+ 376(2) 377(S) 378(3)
379(2)
Csymbol? 48 32 60 63(2) 64(3)
65(4) 86 134(2) 136(4) 144 146 147
150 225(2) 229 234 250(2) 251(4)
252 (9) 254(4) 255(3) 256(3) 257(3)
259 (2) 260(2) 262(3) 263(4) 264(4)
266 267(8) 268(2) 269(13) 270(2)
272(4) 273(4) 275 276(3) 278(3) 279
281(10) 283(g) 285(10) 307(2) 325 358
369 370(3) 375(2) 376(4) 377(4) 378
379 (7)
syntactic-expression 23
syntactic-unit 23
syntax 21
SYSIN 68
SYSPRINT 68(2) 229
system defaults CPL/I text1 86
SY SIEM 34 36 39 86(2) 87 107
system-action 174(2) 169 173(2)
<systeu-action> 54 107 145 172 173
Index 401
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 420

T ftrue, 83(3) 84(3) 85(2) 87(6)
88(8) 91 92(27 125(2) 126 127(3)
128 129 130(3) 141(9) 158(2) 159(S) tab 276 267 269 271 161(6) 162(S) 163(3) 168 214 216 TAB 41 43 68 218(3) 279 281 382(4) $tab-format* 43(2) 68(3) trunc-bif 360
<tab-format> 58(2) 258 271 <trunc-bif> 59 {tab-optiong 41(2) type [of a ncdel 17 <tab-option> 56(2) 228 230
tan-bif 357
<tan-bif> 59
tand-bif 35-l u
<tand-bif> 59
tanh-bif 358 UFL 50
Ctanh-bif> 59 UNAL 50
<target-reference> 59 51 54 55 56 UNALIGNED 37 38 39 50 78(6) 84 85 S7(2) 94 lOl(7) 109 117 118 119(2) 86(2) 111 128 129
141 158 159 194(9) 198 243 266 280 374 <unaliqned> 52 111 192(2) 202 208 218 target-type 366 <undefined, 144(2) 145 146(S) 155 183 185 terminal 21 186(4) 195(37 197 203 204 205 219 terminate IinstructionJ 27 220(3) 221(4) 222(2) 223(2) 226(2) <test> S5(2) 105(2) 106 163 230 235 239 241(2) 245 249 257 258 267 test-attribute-consistency 83 82(2) 87 268 270 271 279 300 360 363
test-char-pie-char 382(2) UNDEFINEDFILE 40 50
test-constraints 141(8) <undefinedfile-condition> 55 226 228 234 test-default-applicability 87(4) 88(4) 236 237 239 240 247(2) 248 265
test-descriptor-extent- UNDERFLOW 36 50
expressions 92 91 <underflow-condition> 55 174 298(2) test-enablement 169(3) 170 299(2) 367
test-generic-aggregation 127(2) 128
test-generic-description 128(2) 129
test-generic-matching 127 126
test-generic-precision 130 129
test-invalid-duplicates 85 83(2) 84(2)
test-matching 125 124 218
test-offset-in-description 91(2) 90
test-spec 161 159 162
test-termination-of-controlled-
group 162 168 279
<text-name) 48t4) 65
THEN 35
<then-unit> 55(2) 105(2) 106 163
time-bif 358
<time-bif> 59 314
TITLE 41
UNDF 50
unique-nane [of a node1 17
*unit> 33(27 34(2) 6q!7) 70(2) 71
i2(5) 73(8) 79 80 81 82(2)
83(2) 86(2) 87(4) 89 96(2)
97(4) 98(3) 104 135
+unmatched) 36(4) 66(S)
unspec-bif 360 195
<unspec-bif> 59
unspec-pv 204
<unspec-pv> 59 195 204 280
$unsubscripted-reference) 44(2) 37(2)
42 75 76 78(2) 82 92 95 114 117
UPDATE 37 39 41 85 86 108
<update> 53 56 108 145 229(3) 234(2)
236(2) 239(2) 240(2)
stitle-option? 41(2) supper-bound) 37(2) 74 llO(3) <title-option> 56(2) 228 231 <upper-bound> 52(2) 110 149 176 184 196 ;:o-b;; 40(2) 210 282 284 212(2) 290 292 215 330 216 336 217 219(2) 220 222
<to-by> 54(2)
Cto-option) 40(3)
<to-option> 54(2) 158 159 160(2) 162(2)
*to-value, 144(2) 160<21 161(2) V transform-subscript-list 217 216
translate 63 32 137 valid-bif 361 380 382
TRANSLATE 359 <valid-bif> 59
translate-bif 359 validate-automatic-declaration 137(2)
<translate-bif> 59 139(2)
translation-phase 32 31 validate-based-declaration 138 137 184
gtranslation-state, 31(2) 32(2) 63 validate-character-pictured-value 382(2) TRANSMIT 40 262 361 365
<transmit-condition> 55 226(2) 227 validate-concrete-declarations 92 68 tree 17 validate-controlled-declaration 138 137
4trim3 90 91(2) validate-declaration 137(2) ti-dd 122 119 121 validate-defined-declaration 139 137
trim-group-control 167 166 246 validate-descriptor 139 129(2) 137
trim-io-control 246(2) 166 247 265 validate-external-declaration 142(2) 138
402
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.

## Page 421

validate-field-of-pictured-value 381
380(3)
validate-input-format 261(3) 260(2) 380
382
validate-numeric-pictured-value 380 261 361
368 371 381(2)
validate-parameter-declaration 139 137
validate-procedure 137 63
validate-program 142 32
validate-static-declaration 139 137(2)
<value-and-type+ 372(10) 135 253 260(2)
261(S) 262(4) 263(2) 272 273 364(5)
371(S) 373(3)
value-of-evaluated-target 195 161 162
value-of-aeneration 177 183 195(3) 203 300
value-of-korage-index 178 177
<value-reference> 59(4) 51 52 55(2)
56(2) 58 96(2) lOO(2) 103 105(2)
109 116(2) 117 118(2) 119(6) 121(2)
124(2) 126(2) 127 137 139(2) 140(2)
141 154 164 166 171 181 i93(3) 198 201
202 203 204 207(2) 227 228 232 237
247(2) 248 265 299 318 319 330 336 338
345 348 360
VAR 50
Ivariable? 84(2)
VARIABLE 37 39 80 81 84 86(4) 108
<variable> 52 51 108 109(4) 117 119 139
141 156 181 188(3) 207 210 213 215
218(3) 238 253(2) 254 268
<variable-reference> 59(3) 52 53
55(4) 56(3) 57(4) 58 94 95
96(2) 102 103 117(2) 118 119(9)
120(2) 121(3) 124 125(2) 127 137 139
141(2) 142 158(2) 164 165 17512)
181(3) 182(4) 192(2) 193(3) 194(3)
<variable-reference> (Continued)
198(2) 201(2) 202(2) 203(2) 204(2)
205(3) 207(3) 208 213(S) 214(2)
215(2) 216 217 218(S) 237 241 242(2)
252 254(4) 268 281 283 287(2) 300 306
311 318 319(2) 330(2) 336(2) 338(2)
345(2) 348(2) 360(2) 363 365 366
VARYING 37 38 39 50 85 112 128 129
<varying> 53 112 122 128 139 203 365 366
VERIFY 362
verify-nif 361
<verify-bif> 59
W
WHEN 38
WHILE 40
<While-Only-grCUF> 54(2) 97 158 168
FhiIe-OFtion) 40(3) 97
<while-oFtion> 54(3) 97 158 161
write 236(2)
WRI'IE 42
*write-statement-q 42 36
<write-statenent> 56 54 235(2)
Z
ZDIV 50 <zero-bit> 136 146 272 302(2) Tlnr, 7nc. 7n-l
308 309(2) 310 311(3) 77u 0-7
368(2) 370 371
ZERODIVIDE 36 50
JV-I a”” J”,
354 361 365
Index 403
Licensed to Tim Sneddon. ANSI order X_72077. Downloaded 7/16/2008 9:58 AM. Single user license only. Copying and networking prohibited.
