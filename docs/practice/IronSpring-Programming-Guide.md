# Iron Spring PL/I Compiler Programming Guide

> **Role:** Pragmatic modern subset implementation notes (not normative for our dialect).
> **Source file:** `refs/literature/IronSpring_Programming_Guide.html`
> **Extraction:** HTML to plain text (BeautifulSoup)

---

Iron Spring PL/I Compiler Programming Guide


Iron Spring PL/I Compiler


Programming Guide

Version 1.4.1


Introduction


Summary of Changes


Restrictions


Compiler limits


Internal Data Representation


Documentation


Compiler differences


Running the compiler


Linking PL/I Programs


Compiler input


Compiler output


Language differences


Syntax chart format


THE PROCEDURE, ENTRY, and PACKAGE statements.


Input and Output


File Declarations


Input/Output Statements


DISPLAY statement


Multitasking


IBM-format packed decimal data


Preprocessor


Builtin Functions and Pseudovariables


The PLISRTx builtins


User-callable library procedures


Preprocessor Facilities


Accessing unprocessed comand arguments


Efficient programming


Debugging PL/I programs


Introduction

PL/I is a powerful programming language suitable for a wide range of
problems.  PL/I has been used to write operating systems and compilers,
simulations, real-time programs, mathematical software, spreadsheets,
GUI programs, and for general computing.


PL/I is widely used on mainframes and midrange systems today, but since
the demise of Digital Research, Inc. has not been easily available on
personal computers.
Iron Spring Software was founded in 2007 with the intention of bringing the
flexibility, reliability, and portability, and security of PL/I to a
wider market.


Iron Spring PL/I is the leading cross-platform PL/I compiler supporting
both Linux and OS/2.  Both native and cross-compilation are supported.


This version of Iron Spring PL/I is still missing some features
of full PL/I.
The section
Restrictions
 lists many of the
missing features.


Summary of Changes for 1.4.1


Array expressions are now fully supported.

The HEX builtin has been added. HEX is similar to HEXIMAGE, but
    uses the length of its first argument as size to convert.

see
Builtin Functions and Pseudovariables
.

Several problems involving arrays of structures have been fixed,
    which caused either incorrect output or compiler errors.

Declaration of arrays with adjustable elements now allowed.

Keywords: Allow UNS as synonym for UNSIGNED, allow CHARVAL as
    synonym for BYTE.

Compatibility options: allow OPTIONS(CONSTANT) as
    synonym for VALUE, GLOBALDEF and GLOBALREF as synonyms for EXTERN,
    and OPTIONS( IDENT('string') ) for compatibility with KEDNOS PL/I.

Parameter for main procedure may be incorrect on Linux. See the

README file
 for details.

Improve disambiguation of variable references to differentiate
    scalar from array references.


Summary of Changes for 1.4.0


Array expressions are now supported for non-adjustable arrays.

    A+S, S+A, and A+A are allowed where A is an array, S is a scalar,
    and + can be any operator. Currently builtins are not allowed, so
    A=ABS(A), for example, will return the scalar ABS(A(1)). The array
    bounds must be constant and the elements cannot be adjustable strings.

The %REPLACE preprocessor statement has been enhanced to optionally
    replace a sequence of tokens rather than a single token. This is
    enabled by a compiler option.


    %replace word by "fixed binary(31)";
    declare a word;


 effectively generates the following declaration: declare A fixed bin(31,0);


    See desciption of the
%REPLACE statement

    for details.

The compiler option -ln will show a statement's nesting level
    within a block and do-group.

The ISO-8859-15 "Not Sign" 'AC'x (¬) is now a builtin default
     in addition to the Codepage 850 value 'AA'x.
     This reduces the number of additional alternates that can be
     specified to three.

The builtin ENTRYADDR can now be used as an initializer for
    STATIC or EXTERNAL pointer variables.

The
Guide
 section
Internal representation
 has been
    updated to include some program control data types previously omitted.


Summary of Changes for 1.3.2


This version now supports hexadecimal binary fixed-point literals
    (from IBM Enterprise PL/I). The syntax is:


"<0 to 8 hex digits>"XN (for signed values)

"<0 to 8 hex digits>"XU (for unsigned values)


    These constants are big-endian REAL FIXED BINARY, either signed
    with precision(31,0) or unsigned (32,0). Note that these are
    written as strings enclosed in single or double quotes, followed
    immediately by "xu" or "xn" (case not significant), with no
    intervening spaces. Underscores may be used to separate groups
    of digits as desired. Values shorter than eight digits are padded
    on the left with zeroes.

    Examples:


put skip list('80'XU);         ->  128

put skip list("FFFF_FFFF"XN);  -> -1

put skip list('FFFFFFFF'xu);  ->  4294967295


Summary of Changes for 1.3.1


The ITERATE statement is now supported


"The ITERATE statement transfers control to the END statement that delimits
its containing iterative
do-group. The current iteration completes and the next iteration, if needed,
is started.
The ITERATE statement can be specified inside a non-iterative do-group
as long as that do-group is
contained by an iterative do-group.


If ITERATE <label-constant> is specified, label-constant must be the label
of a containing iterative do-group.
If label-constant is omitted, control transfers to the END
statement of the most recent iterative do-group containing the ITERATE statement."


The operation and description of the STRING builtin function and pseudovariable
    have been changed slightly. See the description in

Builtin Functions and Pseudovariables

    STRING is not completely compatible among different compilers, and
    this version is not completely compatible with any.


Summary of Changes for 1.3.0


iSub defined arrays are now supported. This is an alpha feature
    in this release.

Undeclared user-defined conditions will now produce warning
    message 557. To avoid this declare the name as a CONDITION. This
    is not required with standard PL/I conditions.

Improvements to handing arrays of structures.

(Linux) Compiler changed to use "large filesystem" calls fstat64


Trademarks


Iron Spring is a trademark of Iron Spring Software.

Gnu is a trademark of the Free Software Foundation.

IBM is a trademark of IBM, Inc.

Intel is a trademark of Intel Corporation.

Linux is a registered trademark of Linus Torvalds.

VAX is a trademark of Hewlett-Packard, Inc.

WATCOM is a trademark of Sybase, Inc. and its subsidiaries.


Restrictions


Unicode
Unicode is not supported.

INITIAL
INITIAL attribute generates incorrect results for arrays
        of structures of unaligned bit elements.

STREAM INPUT

GET DATA is not yet implemented.

    C and E format codes and FLOAT pictures not implemented for GET EDIT.

    Due to a difference in data formats Edit-directed
    stream input may not be compatible with mainframe PL/I. See note on

stream input compatibility.

    The COPY option of the GET statement, the NAME condition, and the
    DATAFIELD builtin functions are not supported.


RECORD I/O
The statements REWRITE, LOCATE, DELETE, and UNLOCK, and are
    not supported for non-ISAM files.

    For ISAM files (Linux only) the statements LOCATE and UNLOCK, and the
    builtin functions ONKEY, and SAMEKEY are not supported.

For better integration with 'ld' on Linux, the external
    procedure name 'main' is reserved and should not be used.

Interlanguage

PL/I procedures can call functions coded in other languages as
    long as they use the "SYSTEM" or "CDECL" calling convention.  The "OPTLINK"
    and other calling conventions are not supported.
    Programs in other languages can call PL/I procedures with restrictions.
    The main program
must be
 PL/I.
    See
Program Linkage
 for more information.

Preprocessor

The only preprocessor statements
    fully implemented by the
built-in
 preprocessor are %INCLUDE and
    the listing
    control statements %PAGE, %SKIP[(n)], %PRINT,
    %NOPRINT, and %REPLACE.  %PROCESS (*PROCESS) is implemented incompatibly
    from the IBM compilers.
    All other preprocessor statements will generate a diagnostic.

    See
Preprocessor
 for more information.

    The beta version of the
full preprocessor
 is described in

Iron-SpringPLIPreprocessor.pdf
.

COMPLEX attribute is not yet fully supported.

GENERIC attribute is not implemented. If anyone wants to use
    GENERIC, please let us know.

DEFAULT statement is recognized but ignored.

DISPLAY
The EVENT option of the DISPLAY statement is not
    currently supported.

The FETCH and RELEASE statements are not currently implemented.


Arithmetic builtins

The TRUNC arithmetic builtin function is not implemented for FLOAT
    data in the current version of the compiler.

COMPLEX pseudovariable

The COMPLEX pseudovariable (not builtin function) is not supported. This corresponds with
    the IBM Enterprise PL/I compilers. The REAL and IMAG builtins can be
    used instead. This is a permanent restriction.

Builtins PLICKPT, PLICANC, PLIREST, PLITEST

These builtins are not implemented. This is a permanent restriction.

PLISRTx

The sort builtin function PLISRTD is supported. PLISRTA, PLISRTB, and PLISRTC
    are not currently implemented.  It is anticipated that versions of these functions,
    and possible interfaces to one or more sort products will be added in the future.

Aggregate Results

Builtin functions will not return agregate results in the current
    version.  This is similar to
Subset G
 support for builtins.
    PL/I procedures may return non-adjustable arrays, but procedures
    returning structures are not yet supported.

GRAPHIC data

GRAPHIC data support will not be included in this compiler.  This is a
    permanent restriction.
    A future version will include WIDECHAR data for UTF-16 support.
    The features not supported include the GRAPHIC attribute and
    the GRAPHIC and MPSTR builtin functions.

LIKE

The LIKE attribute is implemented with one restriction.  If a structure is declared
    as
DECLARE a LIKE b;
 and some elements of "b" have the INITIAL
    attribute, the initialization is not performed for "a".  This will be fixed in a future version.

UNION

The UNION (CELL) attribute may not be used in adjustable structures.


Array cross sections

Array cross sections are not currently supported, but will be in an
    upcoming release.

Array expressions

Array expressions are not allowed with adjustable arrays.


Other restrictions

This list may not be exhaustive at the present time.
The compiler error message 995 ("Unimplemented feature xxx") and the
run-time condition "UNIMPLEMENTED" are used to flag unimplemented
features.  Except for "permanent restrictions" noted above,
these will be removed in future releases.  Some infrequently-used
conversions may still be only partially implemented.


Compiler Limits


Compiler Limits


Maximum number of dimensions
15


Maximum number of levels in a structure

15


Maximum level number in a structure
255


Maximum number of picture characters in a picture

(after expanding all repetition factors)
511


Maximum length of a CHAR or BIT string

(after expanding all repetition factors)
~32000


Maximum nesting depth of %INCLUDE files
4


Maximum precision of FIXED DECIMAL data
18


Default precision of FIXED DECIMAL data
5


Maximum precision of signed FIXED BINARY data
31


Maximum precision of UNSIGNED FIXED BINARY data
32


Default precision of signed FIXED BINARY data
15


Default precision of UNSIGNED FIXED BINARY data
16


Maximum precision of FLOAT DECIMAL data
20


Default precision of FLOAT DECIMAL data
6


Maximum precision of FLOAT BINARY data
64


Default precision of FLOAT BINARY data
52


Minimum/maximum scale factor
-128 / 128


Maximum length of an internal or

external label
31


Internal Data Representations


Internal Data Representations


FIXED BIN, precision 7 or less

UNSIGNED FIXED BIN, precision 8 or less
BYTE (8 bits)


FIXED BIN, precision 15 or less

UNSIGNED FIXED BIN, precision 16 or less
WORD (2 bytes)


FIXED BIN, precision 31 or less

UNSIGNED FIXED BIN, precision 32 or less
DWORD (4 bytes)


FIXED DEC, any precision
Intel x87 BCD format
TBYTE (ten bytes)


FIXED DEC OPTIONS(IBM)
ceil( (prec+1)/2) ) bytes


FLOAT BIN, precision 23 or less
Intel x87 short floating-point
REAL4 (4 bytes)


FLOAT BIN, precision 49 or less
Intel x87 long floating-point
REAL8 (8 bytes)


FLOAT BIN, precision>50
Intel x87 extended floating-point
REAL10 (10 bytes)


FLOAT DEC, precision 7 or less
REAL4 (4 bytes)


FLOAT DEC, precision 15 or less
REAL8 (8 bytes)


FLOAT DEC, precision>15
REAL10 (10 bytes)


PTR, OFFSET
NEAR PTR (4 bytes) on a 4 byte boundary


CHARACTER(n)
n bytes


BIT(n) ALIGNED
(n+7)/8 bytes


BIT(n) UNALIGNED
n bits


VARYING strings will be preceded by a two-byte length prefix

which will force WORD alignment.


FILE
8 bytes aligned on a 4 byte boundaty


ENTRY
12 bytes aligned on a 4 byte boundaty


AREA
Declared size or default of  1000 bytes on a
                     4 byte boundary

                     area overhead is 20 bytes + 8 bytes per element.


TASK
24 bytes aligned on 4 byte boundary


EVENT
16 bytes aligned on 4 byte boundary

                      EVENT data is always initialized.


Documentation

Complete documentation for Iron Spring PL/I will
be available as part of a future release.  In the meantime, documentation
for the IBM "PL/I for MVS and VM Compiler 1.1" is available

online
.


Compiler Differences

The reference implementation for this compiler is IBM PL/I for MVS and VM 1.1.
some features from other compilers have been incorporated.
Most of the differences  from IBM PL/I 1.1 are in the area of Input/Output.
See the section
Input and Output
 for a description of
the input/output facilities.


See
Restrictions
 for information on major language
features that are not currently implemented.


Character representation

Multiple alternate characters can be used to represent
OR and NOT.  Either the single or double quote can be used as a string delimiter.

DECIMAL FLOAT

At the present time, DECIMAL FLOAT data is simulated using BINARY FLOAT.
    This is similar to S/370 PL/I implementations that simulate both
   DECIMAL FLOAT and BINARY FLOAT using the hardware Hexadecimal floating point formats.

FIXED BINARY

FIXED BINARY data with precision 7 or less is stored in a single byte.

E Format

E Format output will print four digits for exponent instead of two.
Output field widths may need to be adjusted.

STRING

The STRING builtin function and pseudovariable restrict the range of data
allowed as arguments.  This should not affect most normal use of STRING.  See
the section
Builtin Functions and Pseudovariables
 for the definition
of STRING.

DATE, TIME, DATETIME

Most Linux systems maintain the date and time in UTC. These builtins return
the value supplied by the system. It is possible to instruct the OS to
maintain the date and time in localtime, if desired.

ONCODE

The ONCODE builtin function returns only the base value of the error code.
For example, IBM PL/I can return codes 600 to 639 for CONVERSION.  This compiler will
return 600 in all cases. See
list of oncodes
.

Additional builtins

See the section
Builtin Functions and Pseudovariables

 for the definitions of all supported builtin functions.

ATTENTION

The ATTENTION condition is enabled by default.  It is raised by pressing CONTROL/C while running
a text-mode application or otherwise sending XCPT_SIGNAL_INTR/SIGINT to the PL/I process.
If no ON-Unit is established for ATTENTION, the implicit action is the same as for ERROR:
print an error message and raise the FINISH condition.
On normal return from an established ON-Unit, execution continues unaffected.
Note that ATTENTION will be raised
only
 in the initial task/thread

RETURNS keyword

Procedure and Entry statements (
not
 ENTRY
declarations
)
may specify "RETURNS( CHARACTER(*) [VARYING] )", or
"RETURNS( BIT(*) [VARYING] )".  The length of the returned string is determined by the entry declaration
in the calling program, e.g.:


DECLARE a ENTRY RETURNS( CHARACTER(20) );
.

IBM compilers require an explicit length and will not allow '*'.

EXTERNAL Attribute and Program Linkage

The EXTERNAL attribute has been enhanced to allow the provision of a non-PL/I name visible to procedures
outside the PL/I program.  The syntax is:


DECLARE x EXTERNAL( 'environment_name' )

See
Program Linkage.
 for more details.


Running the Compiler

The "plic" command is used to invoke the compiler.


The syntax of the plic command is shown.  The case of the option switches is significant.  Lower-case options
have parameters, upper-case do not.


   plic [<options>] <input files> [-o <output file>]

        <options> = <output option>    [<include options>]         [<listing options>]
                    [<source margins>] [<character substitutions>] [<version info>]
                    [<error option>]   [<misc options>]
                    [<optimization option>]

        Options may be entered in any order.
        The Linux shell requires that options containing () be enclosed in double quotes (").

        <
output option
> = -S | -C | -L
                          -S = generate assembler (symbolic) output.
                          -C = generate compiled (object) output.
                          -L = generated linked (EXE or DLL) output.
                               (this option not currently implemented.}
                          -N = generate statement number table to provide
                               information for run-time error messages

        <
include options
> = -i<directory>
                            where <directory> is the absolute or relative path
                            to a directory to be searched for %INCLUDE files.
                            This option may be used more than once on the command line,
                            and directories will be searched in the order listed.

        <
listing options
> =  -l[siaxgmovn]
                            one or more of [siaxgmovn] may be entered, in any order as -lsx
                            -ls = list source
                            -li = list insource
                            -la = list attributes
                            -lx = list cross-reference
                            -lg = list aggregates
                            -lm = list generated code in a format similar to an
                                       assembly listing
			    -lo = list procedure map (statement offset table)
			    -lv = list additional warning messages
			    -ln = include statement nesting level in source listing.

        <
source margins
> = -m(start[,end])
                           This option defines the first and last positions of each
                           input line that contain input for the compiler.  If this
                           option is omitted the source is assumed to be the entire line.
                           This option is included for compatibility with mainframe compilers
                           which would use, for example, -m(2,72).

        <
character substitutions
> = -cn(<list>) and/or
                                    -co(<list>)
                            This option defines up to four characters each to be used as
                            substitutions for the NOT(¬) [-cn()] and/or OR(|) [-co()]
                            operator IN ADDITION TO the defaults.  The caret (^) is
                            a metacharacter for the OS/2 command processor; if the
                            caret is to be used, code two consecutive carets,
                            for example -cn(^^).

        <
version info
> = -V
                         The compiler prints version and copyright information on stderr.

        <
error option
> = -e<option>
                         This option sets the errorlevel returned by the compiler for warning
                         and error messages.  Normally compiler returns 4 if only warnings were
                         issued, and 8 for any errors.
                         -ew tells the compiler to return 0 if only warning messages were issued.
                         -es tells the compiler to return 0 if any errors
or
 warnings were issued.
                           This option is useful when the compiler is run from a script or makefile.
                         -ed tells the compiler to display messages on stderr as well as stdout.

        <
misc options
> = -d<option>
                         <option> is a character string, with or without enclosing quotes.
                         -dLIB tells the compiler it is compiling a standard run-time library procedure.
                         -dELF causes the compiler to generate ELF object files.
                         -dOMF causes the compiler to generate OMF object files.
                         The default is to generate the standard object format for the host OS;
                         for OS/2 -dOMF is the default.

        <
optimization option
> = -O enables optimizations

        <
input files
> and <
output files
> are absolute or relative path names.  Only one input and one
                         one output file are currently allowed.  If the output file is omitted the name
                         is generated.  For example, PLIC -S abc.pli will create a file named abc.asm.


Linking PL/I Programs

The "Readme" file for your environment -
Linux
 or

OS/2
 contains information on linking PL/I programs.


Compiler input

Compiler input is a standard text file [ENV(TEXT)]. Conventionally, the source
program might have the extension ".pli" or ".pl1", but this is not required. Lines are delimited by Newline
characters ('0A'x).  There is no limitation on line length, although individual
tokens
 such as
character-strings are limited to approximately 32,000 characters. Other than newlines, characters
lower in the collating-sequence than spaces ('20'x) are ignored.  This means, therefore, that at the
present time they should not be coded in character string constants.  To compile a character string
constant containing one or more of these characters concatenate the hexadecimal value, for example:


declare two_line_constant char(24) static initial( ('Line one' || '0D0A'x || 'Line two.') );


This type of expression is resolved at compile-time and does not generate any additional code.


The compiler character set is Codepage 850.
The default representations for the
OR
 character is '|', '7C'x,
and for
NOT
 '¬', 'AA'x.  The corresponding unicode characters are
U+007C [bar, vertical], and U+00AC [NOT SIGN].
The character or characters specified above under

Running the compiler
, the options -cn and -co can be used to indicate
additional

encodings used to represent these characters.


String data can be delimited by either the single (')
or double (") quote characters.  The same character must be used to begin and end the
string.  The character
not
 used as a delimiter may be part of the string.
If portability is a concern, use only the single-quote to delimit strings.  The character used as a
delimiter can be included in the string by specifying it twice in succession, for example,
'Let''s dance' will be compiled as
Let's dance
, """Fine"", he said" as
"Fine", he said
.


Source lines are logically continued from the right margin of one line to the left margin of the next,
with no whitespace assumed between the two character positions.
(see the -m compiler option).  Any characters outside the margins, as well as any characters lower
than space in the collating sequence, are ignored.  For example:


         Left Margin            Right Margin                  Line End
             |                       |                            |
             "This is a continued charsome stuff outside the margin
 extra stuff acter string"


will be compiled as
This is a continued character string
.


If a line ends with a word or a number with no trailing blank, and another
word or number begins in the left margin of the next line the two  will be considered a single
token.  A future release will cause words and numbers, but not strings, operators, etc.,
to break at the end of a line.


INCLUDE
 files are identical in format to the source file.  Currently INCLUDE files
must

have either no extension or the extension ".inc" or ".cpy".  The compiler option -i, which can be
repeated multiple times,
provides a list of directories to be searched, in the order written, to locate an INCLUDE file.
The current working directory is always searched last if the file is not found sooner.  INCLUDE files
use the same list of alternate characters as the source, and must use the same margins,
although a future release will allow these to be overridden for each individual file.


Compiler output

If any of the listing options (-l
x
) is specified, compiling a program xxx.pli
will create a source listing file named xxx.lst.


Compiler output currently consists of the optional

source
,

assembler
,

statement offset table
,

aggregate map
,
and
symbol attribute, and cross-reference
 listings,
and an optional
object file
 or
assembler file
 of the compiled program.


All pages of the listing have a heading similar to the following showing the date and time of compilation
and the compiler version used.  The title is taken from the first line of the source file.
The subheading identifies the "Source Listing", the "Assembly Listing", the "Procedure Map",
the "Aggregate Listing",
and the "Symbol Table and Cross-Reference Listing."


Mar  5, 2010  10:28     /* _pli_Sig                                     */
0.8d                                                Source Listing           ...         Page 1


Source listing:

If the compiler option "-ls" is specified a source listing will be generated.


Here are a few lines from a listing:  The letters in blue are a key to the descriptions below, and are not part of the listing.


A   B  C

  340
  341  %include DSA;
  342

  D     E   F

341 (INF104)Processing include file DSA.

G


    1  /********************************************************************/
    2  /*                                                                  */


'B'
 is the line number.  The source file and each %include file
are numbered starting from line one.  A future compiler version will also provide a file
number for each include file to be used on error messages.


'C'
, the next 100 characters of the listing, show the input line.
If the line is longer than 100 characters additional unnumbered lines will display the overflow.


The line following line 342 above shows the format of error, warning, and information messages.

'D'
 is the source line number.

'E'
 is the severity and message number (xxxyyy)
'xxx' is 'INF' for information-only messages as shown,
'WRN' for warnings, and 'ERR' for errors. 'yyy' is a unique message identifier for this error.

'F'
 is the message text.

Currently messages generated by the parser appear intermixed in the source listing, while code
generator messages appear at the end of the source, or intermixed in the object code listing if
one is produced.  In a future version all messages will appear at the end.  Due to parsing considerations
the error message may appear one line before or after the line it references.


Source listing with nesting levels:

If the compiler option "-ln" is specified the source listing will include
information on statement nesting within package/procedure/block and DO-Group.


    1       pk: package;
    2  1      p1: proc;
    3  2        i = 0;
    4  2        do;
    5  2  1       i = i+1;
    6  2  1       end;
    7  2      p2: proc;
    8  3        return;
    9  3        end p2;
   10  2      end p1;
   11  1      end pk;


The first number following the line number is the block level, 1 for the
outermost - statements within package pk, 2 for statements within procedure p1, etc. The second
number is the depth of nesting of Do-Groups in a block, 1 for statements
immediately contained in the DO on line 4, increasing for each nested DO.


Assembly listing:

If the -lm option is specified, a listing of the generated code will appear after the source listing.
This is similar in format to the listing generated by MASM or ALP.  The major difference
is that ALP displays immediate operands in big-endian order.  This listing will always display
immediate operands
as they are stored in memory
, i.e. little-endian. Note that the
assembly code shown is for illustrative purposes and may or may not be assembleable
by any particular assembler.

Here is a small sample of the assembly listing:


                                       ; Stmt 14, Line 17 (DO)
00000313 89 65 [CC]                      mov dword ptr -52[ebp],esp
00000316 8B 75 [D8]                      mov esi,dword ptr -40[ebp]
00000319 8B 3E                           mov edi,dword ptr 0[esi]
0000031B 83 FF 00                        cmp edi,00h
0000031E 90 90 90 90 90 90               db 90h,90h,90h,90h,90h,90h
00000324 8B 65 [CC]                      mov esp,dword ptr -52[ebp]
                                        ; Stmt 15, Line 18 (Assignment)
00000327 8B 75 [D8]                      mov esi,dword ptr -40[ebp]
0000032A 8B 3E                           mov edi,dword ptr 0[esi]
0000032C 89 7D [D4]                      mov dword ptr -44[ebp],edi
                                        ; Stmt 16, Line 19 (DO)
0000032F 89 65 [C8]                      mov dword ptr -56[ebp],esp
00000332 8B 75 [D4]                      mov esi,dword ptr -44[ebp]


Aggregate listing:

The -lg option causes a map of each aggregate (structure and array) to be printed.
For example, the mapping of an aggregate describing the output of the TIME() builtin
from the following declaration will appear as shown.


  dcl   1 rTime,
          5 Hours             pic'99',
          5 Minutes           pic'99',
          5 Seconds           pic'99',
          5 Hundredths        pic'999';

 Def Name            Lvl Dims       Off         Len       Tot Len
  40 RTIME            1             0           9           9
     HOURS            2             0           2           2
     MINUTES          2             2           2           2
     SECONDS          2             4           2           2
     HUNDREDTHS       2             6           3           3


'Def' is the line number of the declaration,
'Lvl' is the normalized level number,
'Off' is the offset of the member from the base of
the structure. 'Off' will be shown as "B.b" for bit members, for example a
one-bit field beginning in bit 1 of byte 0 will be shown as 0.1.
'Dims' is the number of dimensions for array elements.
 'Len' is the member length, again as "B.b" for bit members.
'Tot Len' is the total length of the member, for non-adjustable arrays this
will be the array length.
Adjustable lengths or bounds will cause 'Len' and or 'Tot Len' to be
displayed as "*".


Statement offset listing:

The -lo option causes a procedure map to be printed listing the assembled offset
of each statement, its starting line number, and the statement type.
Here is the procedure map for the procedure containing the code shown above:


  Offset  Line Statement type             Offset  Line Statement type             Offset  Line Statement type
      08     2 PROCEDURE                ENV
      23    10 Assignment                     35    11 PUT                            E4    12 PUT
    0180    13 Assignment                   0197    14 PUT                          0246    15 PUT
    02C3    16 PUT                          0313    17 DO                           0327    18 Assignment
    032F    19 DO                           0343    20 PUT                          03BF    21 Assignment
    03C8    22 END                          03D0    23 PUT                          0420    24 Assignment
    0429    25 END                          042E    27 END


'Offset' is the location of the beginning of the statement relative to the start of the compilation unit
(zero).
'Line' is the starting line number of the statement.
'Statement type' describes the type of statement.


Symbol table and cross-reference listings:

If the compiler options "-la" or "-lx" are specified,  the attribute list (symbol table) or
cross-reference listing is printed as shown below.
"-la" causes the variable attributes to be printed,
"-lx" causes the cross-reference listing to be generated.
The example below shows the results of specifying "-lax".


H                    I    J                                                     K

ADDR                 124  Builtin
BSW                  309  Entry Unaligned                                       [ Code+'13C0'x ]
                            265
BYTE                 332* Builtin
                            737   738
C                     77  Char(1) Unaligned                                     [ DSA-'51'x(1) ]
                            266   278   300   321   346   362   370   446   452   453
                            478   507   510   512   514   516   531   553   557   566
C4                   109  Char(4) Unaligned Based                               [ +'00'x(4) ]
                            297   298   533   615   679
C4                   327  Char(4) Unaligned Parameter                           [ Loc @DSA+'08'x(4) ]
                            723   724   725   726   727
CONDS                 97  (9) Char(6) Var Unaligned Static Init()               [ Static+'12'x(8) ]
                            407   635
DSA_BELOW_EBP         27  Unaligned Structure In(PLI_DSA)                       [ +'00'x(32) ]
                            635   636   642   643   644   646   647
DSA_CHC               33  Ptr Aligned In(PLI_DSA.DSA_BELOW_EBP)                 [ +'08'x(4) ]
                            643
PLI_DSA              340  Unaligned Based Structure Level(1)                    [ +'00'x(40) ]


'H'
 is the name of the data element.
If the name is longer than twenty characters it will appear
on a line by itself, with the remaining data shown on the next line.

'I'
 is the line
number of the declaration of this item.  Undeclared data, like 'BYTE' above show the line number where this
element is first used, followed by '*'.

Each
generation
 of data will be listed separately.  The two occurrences of 'C4' above represent
two data elements with the name 'C4' in two different blocks.  Usage of each is cross-referenced separately.


'J'
 lists the data attributes, which should be self-explanatory.
This field may occupy more than one line.


'K'
 shows the address of the item in [].  The addresses are displayed as follows:


Code labels:
Code+'oooo'x. 'oooo' is the hexadecimal offset from the first code byte.  See 'BSW' above.

STATIC data:
Static+'oooo'x(ll). 'oooo' is the offset from the first data byte.  'll' is
the (decimal) length of the data element.  The length of UNALIGNED BIT elements is shown
as (x.y) where 'x' is the number of whole bytes, and 'y' is the number of additional bits.
For an array,  'll' is the length of one member.  See 'CONDS' above.

AUTOMATIC data:
DSA-oooo'x(ll). 'oooo' is the offset from the start of the DSA containing the data.
Register EBP always points to the current DSA.  Lexically containing DSAs (static scope) are chained from
offset -8 in the contained DSA.  All offsets are negative.
See the file 'DSA.INC' distributed with the library.
The variable 'C' above is an example of AUTOMATIC data.

BASED data:
+'oooo'x(ll).  'oooo' shows the offset from the beginning of the BASED variable or
structure.  See the first occurrence of 'C4' above.

Data addressed via locator:
Loc @DSA±'oooo'(ll).  This is the address format for parameters,
adjustable data, and CONTROLLED data.  'oooo' is the offset of the locator/descriptor which points to
and describes the data.  See the second 'C4' above.


If there were any errors or warnings, a message showing the number of them appears at the end
of the listing.


Object output:

Compiling a program xxx.pli with the -C option will create an object file xxx.obj for OMF-format
output, or xxx.o for ELF.  The layout of the generated code is described under "Program Structure"
below.


Assembler output:

Compiling a program xxx.pli with the -S option will create an assembler file xxx.asm in a
format suitable for the IBM ALP assembler or the open-source JWASM assembler.
Other assemblers will most likely not be able to assemble this file correctly.

The compiler generates assembler labels for level-1 EXTERNAL data and for each
EXTERNAL ENTRY.  Most assemblers have a list of
reserved
words
 which cannot be used for labels, typically including all
register names (e.g. EAX), all instruction mnemonics (LEA), and
most pseudo-ops (ORG).  Using any of these as PL/I external
labels will cause an assembly error.


A few lines of assembler output are shown:


; Stmt 2, Line 3 (PUT)
 .code
 sub esp,64
 mov word ptr 0[esp],1
 mov dword ptr 12[esp],ebp


The source statement and line number that generated each piece of code, along with
the statement type is shown as indicated.  The line number ties this code to the source
listing.


Program Structure

The generated program consists of a single code segment(.text) identified by the
 label
'_pli_code'
, a single static initialized data segment(.data)
 identified by
'_pli_data'
, a single static unitialized data segment
 (.bss,
'_pli_bss'
), an unnamed stack segment for OS/2,
 and zero or more external common segments.


External segments are generated for each variable or structure declared EXTERNAL, as well as for
each EXTERNAL FILE.  If the variable has the INITIAL attribute the data is always generated, otherwise the
declared amount of storage is reserved.  Here is the generated code for an uninitialized
external structure named 'abc':


ABC segment dword common 'ABC'
_ABC equ $
 org _ABC+0Ch
ABC ends


Here is the code for the initialized external segment 'sysprint':


SYSPRINT segment dword common 'SYSPRINT'
_SYSPRINT equ $
 dd 0
 dd 0
 dd _SYSPRINT+12
 dw 8
 db 'SYSPRINT'
SYSPRINT ends


The linker combines identically-named external segments.  The resulting length
will be the length of the largest such segment. All declarations of EXTERNAL data should be identical in length.
It is permissible to have only one declaration of EXTERNAL data have the INITIAL attribute, and
all procedures declaring it will reference that data. If the initial values of the
elements differ between declarations the results are unpredictable.

Warning:
 The Gnu linker has a bug linking addresses
declared with the INITIAL attribute in external data. Only
one
 such
declaration can have the INITIAL attribute. The linker will add together the
values of all initialized addresses.


Compiler Work File:

The compiler creates a temporary work file named PLI-xxxxxxxx-yyyyyyyy-zzzzzzzz.tmp,
in the current working directory on OS/2, or the /tmp directory on Linux.
Normally this file is deleted at the end
of compilation.  Errors in the compiler causing
certain traps may prevent automatic deletion of this file.
If it not automatically removed it may be deleted manually.


Syntax Chart Format

In the syntax charts that follow, the characters '-', '.', '+', 'v', '^', '|', '<', and '>' are used only for
drawing the chart, and are not part of the syntax.  Other punctuation such as parentheses, semicolons, commas,
 etc. must be entered as shown.Words shown in
CAPITALS
 are PL/I keywords.
Words shown in
lower-case
 represent names chosen by the programmer or variable information.
(The actual case of the word in the program is not significant.)
'>>' identifies the beginning of a diagram, and '><' represents the end.
Entities appearing in '|' data '|' represent a syntax fragment that is expanded later.
Elements on the main line of the diagram as '--- stuff ---' are
required
.  Elements appearing above or
below the main line represent
choices
. Elements shown all below the line are optional with no default.
Where one element is shown above the line it represents the default if none of the choices is specified.


The PROCEDURE and ENTRY and PACKAGE statements


PROCEDURE
 statement.


PROCEDURE-+--------------------+-+-----------------+-+-----------------+-+-------------+-+------------------+-- ; ><
          |                    | |                 | |                 | |             | |                  |
          +-| ( parameters ) |-+ +-| returns-opt |-+ +-| options-opt |-+ +- RECURSIVE -+ +-| external-opt |-+


ENTRY
 statement.


ENTRY--+--------------------+-+-----------------+-+-----------------+-+-------------+-+------------------+- ; ><
       |                    | |                 | |                 | |             | |                  |
       +-| ( parameters ) |-+ +-| returns-opt |-+ +-| options-opt |-+ +- RECURSIVE -+ +-| external-opt |-+


Other keywords defined by The IBM PL/I for MVS and VM compiler 1.1 and various other compilers are accepted for
source compatibility, but are otherwise ignored.


|parameters|

  +------- , ----------------+
  v                          |
|-.- parameter-description --+----|


Each
parameter-description
 supplies the attributes for one parameter, therefore the entry expects as many
arguments as there are items in the list.  A parameter-description is a list of data attributes that describes that parameter.  For example:

FIXED BINARY(7)
,
CHARACTER(8)
,
POINTER
, or
ENTRY
 would all be valid parameter descriptions.


|returns-opt|

|-- RETURNS ( returns-description ) ---|


The
returns-description
 describes the attributes of the data which will be returned by this entry
if it is invoked as a function.  If the
returns-opt
 is omitted the attributes are determined by the
first character name of the first or only name prefix — I thru N: FIXED BINARY(15), other: FLOAT DECIMAL(6).


|options-opt|

                +--- , --------+
                |              |
                +--------------+
                v              |
|---- OPTIONS ( . entry-option +  ) ---|


Each
entry-option
 is one of the following:


FROMALIEN
Indicates that this entry may be called from a non-PL/I program.
A FROMALIEN
procedure should be considered to be
non-nested
 —  that is, it may not access any AUTOMATIC
data in the containing procedures, if any.  Also, any condition-handling established in a
previously-called procedure will be ignored on entry to the FROMALIEN procedure.


LINKAGE( linkage-type )
'linkage-type' indicates the type of calling sequence with which
this entry will be invoked.  If LINKAGE is omitted, a standard PL/I convention will be used.
(see
Run-time Considerations
).
Currently the only valid linkage type is SYSTEM.
This can be used to call functions written in C, including OS/2 API functions,
but is also a valid  convention among PL/I procedures, with some restrictions.

For source compatibility with IBM PL/I for OS/2, specify
'OPTIONS(
BYVALUE
 LINKAGE(SYSTEM) )'.

BYVALUE
 is assumed by this compiler and the keyword is ignored, but IBM PL/I defaults
to
BYADDR
 unless otherwise specified.


LINKAGE(OPTLINK)

will be implemented in a future version to specify the "Optlink" calling convention that uses
registers for some arguments.


MAIN
Indicates that this is the initial or 'main' procedure.  Iron Spring PL/I allows only one
OPTIONS(MAIN) procedure to be included in an executable program.


REENTRANT
This is included for compatibility only.  All code compiled by Iron Spring PL/I is reentrant.


RECURSIVE is supported for compatibility only.  It indicates that this entry may call itself, either directly
or indirectly through a chain of calls.  Iron Spring PL/I assumes all entries are potentially recursive.


|external-opt|

|-- EXTERNAL --+----------------------+---|
               |                      |
               +- ( external-name ) --+


The
external-name
 is a character string, enclosed in quotes, representing the name by
which this entry will be known to external callers.
External 'external-name's beginning with the string "_pli" are reserved for use by the
run-time library.  If the 'external-name' is not specified, the default external name is
an uppercase translation of the first label on the external procedure or entry
statement.  For example,

x: y: PROCEDURE;
 will use "X" as the external name for the linker.


ENTRY Declarations

The following attributes can be used in the declaration of an external entry:


DECLARE-- decl-name --ENTRY-+--------------------+-+-----------------+-+-----------------+-+------------------+-- ; ><
                            |                    | |                 | |                 | |                  |
                            +-| ( parameters ) |-+ +-| returns-opt |-+ +-| options-opt |-+ +-| external-opt |-+


Parameters, returns-opt, and external-opt are described under "PROCEDURE and ENTRY Statements" above.
An entry declared
without
 a parameter list —
DECLARE a ENTRY;
 will accept any
number of arguments with no conversion.  An entry declared with an
empty
 parameter list —

DECLARE a ENTRY();
 does not allow any arguments.


|options-opt|

                +--- , --------+
                |              |
                +--------------+
                v              |
|---- OPTIONS ( . entry-option +  ) ---|


The following options are valid for entry declarations:


ASSEMBLER (abbreviated ASM)
Indicates that the entry is a non-PL/I procedure (assembler, C, etc.)
The compiler generates code to save and restore the x87 Floating-point control word, since programs
in different languages may change this unpredictably.

LINKAGE( linkage-type )
'linkage-type' is as described above under "PROCEDURE and ENTRY Statements".


PACKAGE
 statement.


PACKAGE-+-------------------------------------+----+-------------------------------------+--+-------------------+- ; ><
        |                                     ^    |                                     ^  |                   ^
        |               +----- , ------+      |    |                +---- , ------+      |  |                   |
        |               |              |      |    |                |             |      |  |                   |
        |               v              |      |    |                v             |      |  |                   |
        +- EXPORTS ( -+-+- procedure --+-+- )-+    +- RESERVES ( -+-+- variable --+-+- )-+  +- OPTIONS(options)-+
                      |                  ^                        |                 ^
                      |                  |                        |                 |
                      +------- * --------+                        +------- * -------+


A package is a block which can immediately contain only declarations, DEFAULT statements, and PROCEDURE blocks.
The package forms a name scope that is shared by all declarations and procedures it contains.  Some or all
of the level-1 procedures can be exported and made known outside the package as external procedures.


EXPORTS
Specifies that all [EXPORTS(*)] or the named procedures are to be made known outside the
package.  Procedures not exported are known only within the package.  If the EXPORTS keyword is not
specified, EXPORTS(*) is assumed.

RESERVES
This keyword is not used by the Iron Spring compiler, and is treated as comments only due
to differences in handling initialized and uninitialized EXTERNAL data.

OPTIONS
This keyword is not used by the Iron Spring compiler, and is treated as comments only for
compatibility with other compilers.


All declarations with package scope (outside of procedures within the package) must have STATIC, BASED,
CONTROLLED, or EXTERNAL storage class, AUTOMATIC data is not allowed outside of procedures.


Input and Output Statements

In the following discussion,
delimiter
 signifies either a linefeed character (ASCII '0A'x),

or
 a carriage-return/linefeed combination (ASCII '0D0A'x).

Iron Spring PL/I can read and write:


Standard text files, with lines of text terminated by delimiters.

Files of fixed-length records with or without delimiters.

Files of variable-length records where the length of each record
is indicated by a two-byte binary prefix, with or without delimiters.

Files containing arbitrary streams of binary data with no record boundaries.


File Declarations


Input/Output Statements


Stream Input/Output


Record Input/Output


INDEXED Input/Output
See
Iron-SpringUsingIndexedFiles.pdf


File Declarations

The general form of a file declaration is shown below.  As usual, keywords can be specified in any order.  Many
keywords may be specified either in the declaration or in the
OPEN
 statement for the file.  If they are
specified in both, they must not conflict.  For example, a file declared
INPUT
 cannot be opened as
OUTPUT

or the UNDEFINEDFILE condition will raised when the file is opened.


>>-DECLARE-
name
-+------+-+--------+-+--------+-+----------------------------------+-+------------------+-><
                |      | |        | |        | |                                  | |                  |
                +-
FILE
-+ +-
RECORD
-+ +-
INPUT
 -+ +-
ENVIRONMENT(environment options)
-+ +-
other attributes
-+
                         |        | |        |
                         +-
STREAM
-+ +-
OUTPUT
-+
                                    |        |
                                    +-
UPDATE
-+


"name" is the name by which the file is known to the PL/I program.  It must follow the rules for PL/I
identifiers.

The keyword
FILE
 is required if none of the other file declaration keywords are coded.  Otherwise it
is implied.


RECORD and STREAM Files


STREAM
 indicates that the file is a
continuous stream
 of data items in character form.
STREAM
 files
are read or written without regard to record boundaries.  The PL/I statements
GET
 and
PUT
 are used to
process
STREAM
 files.


RECORD
 indicates that the file is logically divided into
records
 which are composed of one
or more data items in any form.  A
record
 is always read or written as a unit.  The PL/I statements
READ
,

WRITE
,
REWRITE
, and
LOCATE
 are used to process
RECORD
 files.


INPUT, OUTPUT, and UPDATE


INPUT
 indicates that the file is to be read only.  The file must exist or the UNDEFINEDFILE condition will
be raised when the file is opened.

OUTPUT
 indicates that the file is to be written only.  If the file does not exist, it will be created when
opened.  If it does exist, it will be deleted and a new file created, unless the
ENVIRONMENT
 option
APPEND

is specified.

UPDATE
 indicates that the file is going to be both read and written.


ENVIRONMENT Options

The
ENVIRONMENT
 attribute allows specification of implementation-dependent options for a file.
Iron Spring PL/I allows coding of all
ENVIRONMENT
 attributes allowed by
IBM PL/I for MVS and VM
.
Only attributes actually used are described here.


Record format keywords:
D, DB, F, FB, FS, FBS, V, VB, VS, VBS, U
.

One of these keywords is coded to indicate the type of records to be processed.
Only the
D, F, V, and U
 are significant, the others
are provided for compatibility.
F
 indicates that the file consists of records having all
the same length.
V
 or
D
 indicate that the records in the file can have different lengths.

The options
TEXT
,
CRLF
,
LF
, and
VARLS
 provide additional information about the external
representation of
F, V, and D
 files.


Record format
U
 allows reading and writing files without regard to formatting or record boundaries.
When reading
INTO
 a character string, the amount of data available up to the maximum
length of the string is returned.  ENDFILE is raised only on the
next
 read after the last byte has been returned.
See the sample program 'readu.pli' for an example of using
ENV(U)
 to read a file with no
predetermined maximum record length.  Writing
FROM
 a character string writes
n
 bytes of data from the
string, where
n
 is the current or constant length of the string.


BLKSIZE(n)
 specifies the size of the buffer used to transfer data
to or from the file or device.
If this keyword is not specified, a default value is used.


RECSIZE(n)
 specifies the uniform record length for
F
 files, or the maximum record length for

V or D
 files.  If this keyword is not specified, the default is
LINESIZE
.  The
RECSIZE
 value
describes only the data portion of the record, exclusive of delimiters, length prefixes, etc.


TEXT
,
CRLF
 or
LF
 specifies the presence of record delimiters.
If none of the three options is specified, no delimiters will be written on output or expected on input.

TEXT
 indicates that the default record delimiter for the system will be used:
CRLF for OS/2 or LF for Linux.  The
TEXT
 option should be used in preference to either
CRLF
 or

LF
 unless a specific delimiter is required, since code specifying
TEXT
 is portable.

LF
 indicates that a single newline character (ASCII '0A'x)
delimits records in the file (Unix convention).
CRLF
 indicates that a carriage-return/linefeed combination
(ASCII '0D0A'x) is used as a delimiter (OS/2, DOS convention).  The delimiters will be written as specified on output, but
either is always acceptable for input.


VARLS
 specifies that records in the file will be written with a FIXED BINARY(15) prefix containing the
length of the following data.


APPEND
 specifies that an
INPUT
 or
UPDATE
 file will have data appended to the end if it
already exists.  When the file is opened, the current file position will be set to the end of the file.


Other Attributes


Storage attributes
ALIGNED, UNALIGNED, AUTOMATIC, BASED, DEFINED, STATIC, INTERNAL, and EXTERNAL
  may be
used with declarations of
file variables
.
INTERNAL and EXTERNAL
 are also applicable to
file constants
.
The default for a
file constant
 is
EXTERNAL
.  A
file constant
 declared INTERNAL
is automatically made STATIC.

BUFFERED
 or
UNBUFFERED
 are used to control the amount of storage that will be used for buffers
for this file.
BUFFERED
 requests that a storage area of length
BLKSIZE
 is to be used to handle
transfers to or from the file or device, minimizing interaction with the Operating System.
UNBUFFERED
 specifies
that each record is read or written immediately, and no additional storage is used.

KEYED
 indicates that
keys
 may be used to select a particular record to be read or written, bypassing
normal sequential processing.  A
key
 for this implementation is a CHARACTER(12) data item which uniquely identifies
a record in the file.  This is not intended to be user-modifiable data, except that key of (12)'00'x indicates the beginning
of the file, and (12)'FF'x indicates the end. The
KEY
,
KEYFROM
, and
KEYTO
 keywords may be used
on I/O statements to receive or supply the identifier of the record.


This implementation of KEYED is unique to Iron Spring PL/I.


PRINT
 is an additional attribute for
STREAM OUTPUT
 files indicating that this file is ultimately
destined to be printed.  The compiler keeps track of the current "page" and "line" in the file, and raises the

ENDPAGE
 condition when starting a new page.
PRINT
 also enables print-related processing of the
PAGE

option on the
PUT
 statement and the
PAGE
 format item.
Specifying
PAGESIZE(0)
 on the
OPEN
 statement will
suppress page breaks for
PRINT
 files.See
PLITABS
.

VARIABLE
 indicates that this declaration is a
file variable
 rather than a
file constant
.

File variables
 can be assigned the value of a
file constant
 at run time, for example, to pass a file
as a parameter to a procedure, but cannot be used for I/O unless a value has been assigned.


STREAM
 files default to
ENV(V TEXT)
 unless other attributes are specified.
RECORD

ENV(V)

files default to
ENV(V VARLS)
.


Standard Files


Unless overridden by
ENVIRONMENT
 or
OPEN
, the
standard files SYSIN and SYSPRINT default to stdin and stdout with the following attributes
respectively.


SYSIN
ENV(V TEXT RECSIZE120))


SYSPRINT
PRINT ENV(V TEXT RECSIZE(120)) LINESIZE(120) PAGESIZE(60)


If stdin and stdout are the current terminal(VIO) window, these files default
to ENV( BLKSIZE(0) ).


Interactive options


ENVIRONMENT(INTERACTIVE)
 Specifying the "INTERACTIVE" environment
option for a STREAM OUTPUT file forces the file to be UNBUFFERED and will
immediately write each output data item followed by a new line.
This will cause prompts to be
written immediately rather than buffered.  Normally a complete line is accumulated
and only written when a SKIP, PAGE, or COL which forces a line advance is written.

L-Format item
 On input the L-Format item will read all characters up to
the end of the current line and assign it to the corresponding data list item,
converting as necessary.
On output the current item will be converted to character and written, padded
on the right with blanks if necessary to fill the current line.  The current line
will then be written.  Declaring output files UNBUFFERED may be required to insure
the completed line is not buffered.


Input/Output Statements


OPEN
 and
CLOSE
 apply to both
STREAM
 and
RECORD
 files.  The
GET
 and
PUT

statements operate on
STREAM
 files.  The
READ
,
WRITE
,
REWRITE
, and
LOCATE

statements operate on
RECORD
 files.  The assumption is made that the reader is familiar with, or has access
to a reference for, the I/O statements of
IBM PL/I for MVS and VM
.  Only the syntax accepted by Iron Spring
PL/I is shown here, no explanation is provided.


OPEN
 statement.


         +----------------------- , -------------------------------------+
         v                                                               |
>>-OPEN--.---FILE(file_ref)-+------------------+----+---------------+----+---- ; ----><
                            |                  |    |               |
                            +-|stream-options|-+    +-
TITLE(string)
-+
                            |                  |
                            +-|record-options|-+


|stream-options|

              +-INPUT ---------------------------+
              |                                  |
|----STREAM---+----------------------------------+-+-------------+-----|
              |                                  | |             |
              +OUTPUT ---+-----------------------+ +-LINESIZE(m)-+
                         |                       |
                         +-PRINT-+-------------+-+
                                 |             |
                                 +-PAGESIZE(n)-+

|record-options|

              +-INPUT -+
              |        |
|---RECORD----+--------+-+------------+-+-------+------|
              |        | |            | |       |
              +-OUTPUT-+ +-BUFFERED---+ +-KEYED-+
              |        | |            |
              +-UPDATE-+ +-UNBUFFERED-+


"file-ref" identifies a PL/I file, the reference may be subscripted or qualified, but cannot represent an
aggregate such as an array of file variables.


The
TITLE(string)
 option provides the operating system "filespec" of the file to be read or written.
"String" is any valid character string expression.
If the
TITLE
 is omitted, the file name used by the system will be the declared PL/I filename
in uppercase, except for SYSIN and SYSPRINT which default to stdin and stdout.
The TITLE may be a full filespec [TITLE('C:/test.dat')], a device name
[TITLE('LPT1:') TITLE('/dev/nul')], a relative filespec, or the name of
an environment variable containing the filespec as "DD_ddname".  To reference an environment
variable, the "DD_" is required. "DD_ddname", where "ddname" is any valid value,
will be the name of the environment variable, and the value of the variable
will be interpreted as the filespec.


Unlike IBM OS/2 PL/I, file attributes
may not
 be specified in the title string.


CLOSE
 statement.


          +----- , ---------+
          v                 |
>>-CLOSE--.--FILE(file_ref)-+-- ; --><


STREAM Input/Output


GET
 statement.


>>-GET-FILE(file_ref)-+-----------------+-+----------------------+- ; --><
                      |                 | |                      |
                      +-
SKIP
----------+-+ +-|
data_specification
|-+
                             |        |
                             +-(expr)-+


PUT
 statement.


>>-PUT-FILE(file_ref)-+---------------------+-+----------------------+- ; --><
                      |                     | |                      |
                      +-
PAGE
-+------------+-+ +-|
data_specification
|-+
                      |      |            | |
                      |      +-LINE(expr)-+ |
                      |                     |
                      +-
SKIP
-+--------+-----+
                      |      |        |     |
                      |      +-(expr)-+     |
                      |                     |
                      +-
LINE(expr)
----------+


Data Specification


|--+------+---( |data-list| )---------------------+----|
   |      |                                       |
   +-LIST-+                                       |
   |                                              |
   +-DATA---+-----------------+-------------------+
   |        |                 |                   |
   |        +-( |data-list| )-+                   |
   |                                              |
   |        +-----------------------------------+ |
   |        v                                   | |
   +-EDIT---.-( |data-list| )-( |format-list| }-+-+


The
SKIP
 option causes a new line to be started
in the data stream prior to transmitting any data from the current or subsequent
GET
 or
PUT
 statement.
The
(expr)
 is converted to an integer
n
.  On output,
n
 newlines (linefeed or
carriage-return/linefeed) are inserted.  On input the remainder of the current line and the n-1 following
lines are skipped.
If
(expr)
 is omitted or not greater than zero, 1 is used for the value of
n
.
For
PRINT
 files, the ENDPAGE condition may be raised if
PAGESIZE
 is exceeded.


The
PAGE
 option is valid only for
PRINT
 files.  It causes a page eject (ASCII '0C'x)
to be inserted in the output stream prior to transmitting any data from the current or subsequent

PUT
 statement.  If both
PAGE
 and
LINE
 are present, the PAGE option is
applied first.


The
LINE
 option is valid only for
PRINT
 files.  It  causes a new line to be started
in the output stream prior to transmitting any data from the current or subsequent
PUT
 statement.
The
(expr)
 is converted to an integer
n
.  If
n
 or more lines have been written on the
current page, or if
n
 exceeds
PAGESIZE
, a new page is started.  newlines are inserted in
the output strean to position the next data written at line
n
.
If
(expr)
 is less than or equal to zero, 1 is used for the value of
n
.


GET EDIT compatibility

STREAM INPUT data for Linux or OS/2 normally consists of variable-length
lines with LF or CRLF delimiters.
Many existing IBM mainframe PL/I programs make the assumption that these records
are fixed card images, 80 characters long. Therefore you may see code like the following:

    DECLARE card CHARACTER(80);
    GET EDIT(card)(A(80));

With this code Iron Spring PL/I will attempt to read 80 characters from SYSIN,
ignoring record boundaries, and will garble the input stream.


To ensure that record boundaries are observed, code the GET statement as:

    GET EDIT(card)(A);

WITHOUT the length specification. In this case a maximum of one record will
be read for each GET, and the resulting data assigned to "card" left-justified
and space-filled, which is probably what the program expects.


Alternatively, a program "encard" is supplied in the "samples" directory which
will expand a text file to 80-byte records with blank padding, so that standard
Linux or OS/2 files can be used with unmodified PL/I programs expecting card
 images.
You can use "encard" run your program with a command simular to:

encard<YOUR.DATASET|YOUR.PROGRAM

"encard" should point to the directory where you have placed the compiled program,
for esample "/usr/local/bin/encard", or "~/bin/encard" (if in your home
directory). replace YOUR.DATASET and YOUR.PROGRAM with the name of the input dataset
and the full pathname of the program you want to run, respectively.


The
PLITABS
 structure provides a description of a printed page used by the I/O routines.
A definition of this structure is provided in "lib\include\plitabs.inc."  This defines the pagesize
and linesize used for PRINT files, and the tab positions for list-directed and data-directed output.
If PLITABS is not used the defaults are PAGESIZE(60) LINESIZE(120) and four tabs at columns
25, 49, 73, and 97, the same as the defaults for the IBM compilers.  To change these values include
the PLITABS structure in your program and set the desired values before opening any PRINT files.
Space is reserved for up to 255 tab values.


RECORD Input/Output


READ
 statement.


>>-READ-FILE(file_ref)-+-----------------------------------+----------- ; --><
                       |                                   |
                       +-INTO(ref)-+-------------+---------+
                       |           |             |         |
                       |           +-KEY(expr)---+         |
                       |           |             |         |
                       |           +-KEYTO(ref)--+         |
                       |                                   |
                       +-SET(pointer-ref)-+-------------+--+
                       |                  |             |  |
                       |                  +-KEY(expr)---+  |
                       |                  |             |  |
                       |                  +-KEYTO(ref)--+  |
                       |                                   |
                       +-IGNORE(expr)----------------------+


WRITE
 statement.


>>-WRITE-FILE(file_ref)--FROM(ref)--+-------------------------+----------- ; --><
                                    |                         |
                                    +-KEYFROM(expr)-----------+
                                    |                         |
                                    +-KEYTO(ref)--------------+


LOCATE
 statement.


>>-LOCATE--based-variable--FILE(file-ref)--+------------------+-------------------+---- ; --><
                                           |                  |  |                |
                                           +-SET(pointer-ref)-+  +-KEYFROM(expr)--+


REWRITE
 statement.


>>-REWRITE-FILE(file_ref)--+-----------+--+-----------+----------- ; --><
                           |           |  |           |
                           +-FROM(ref)-+  +-KEY(expr)-+


INDEXED Input/Output


See
Using Indexed Files
 for information
about indexed (ISAM) files. (currently Linux only)


DISPLAY statement

The DISPLAY statement writes text to stderr and optionally accepts a reply.


>>-DISPLAY (expression)--+----------------------------------------+----------- ; --><
                         |                                        |
                         +-REPLY(char-ref)-+-------------------+--+
                                           |                   |
                                           +-EVENT(event-ref)--+


"expression" is converted to a character string and written to stderr.  If the "reply" option
is not specified, this is followed by a newline.  If the "reply" option
is specified it must be a character reference which will receive the
response from stderr.

The "event" option is currently ignored, and the
statement is executed synchronously; control does not return until the
response has been read.

If the "display" expression ends with a '$' neither the '$' nor the newline
will be displayed. This allows output from multiple display statements to
appear on one line.


Multitasking

The Compiler currently supports the "multitasking" API
similar to
 IBM PL/I
for MVS and VM.  "Multithreading", as supported by IBM Enterprise PL/I,
will be implemented in a future release.  Iron Spring PL/I will then
continue to support both feature sets.


The multitasking API adds a TASK data attribute, the
TASK, EVENT, and PRIORITY options of the CALL statement, and the
PRIORITY builtin function and pseudovariable.  The EVENT attribute and
the builtins/pseudovariables COMPLETION and STATUS, and the WAIT
statement, which IBM does not consider part of the multitasking
feature, are also documented here.


The multitasking CALL is indicated by the appearance of one or more
of the following options on a call statement:


  TASK (task reference)
  EVENT (event reference)
  PRIORITY (expression)


The task priority is currently determined by the operating system, so the TASK and
PRIORITY options are otherwise ignored.  If the EVENT option is coded, the
event variable referenced will be set to incomplete (COMPLETION='0'b) by the
call, and will be set to complete (COMPLETION='1'b) when the subtask terminates.

COMPLETION builtin function and pseudovariable:

An event variable may be in one of two states, incomplete or complete.  The COMPLETION
pseudovariable is used to set the state, assigning an expression to it which
evaluates to '0'b for incomplete or '1'b for complete.  The COMPLETION builtin
function returns the state of the event as a bit(1) value.  assigning
COMPLETION(e)='1'b will wake up any tasks waiting on event e.

The WAIT statement will cause the current task to wait for completion of
an event.

The syntax of the WAIT statement is


>>-WAIT (event reference)-- ; --><


IBM PL/I will allow a task to wait for completion of multiple events.  Iron
Spring PL/I currently allows waiting for one.  It the event is complete, the wait
statement is a no-operation.  If the event is incomplete, the task will be
suspended until completion.

EVENT variables are always initialized to COMPLETION='0'b and STATUS=0. (exception, for EVENT variables in an array
of structures only the first occurrence will be initialized). If event variables
are reused they must be reinitialized prior to use.


Tasks constitute a hierarchical structure with the initial task of the procedure,
the "major task", at the root.  If the major task "A" attaches tasks "B" and "C",
these are then subtasks of "A".  If subtask "B" attaches task "D", "D" is
then a subtask of "B" and so on.  When a task terminates, any subtasks which
are not complete will be terminated.  Otherwise a task will terminate when
any of the following occurs:


Control reaches a RETURN or END for the initial procedure of the task.

The task executes an EXIT statement.

Implicit action for the ERROR condition or normal return from an
ERROR On-unit in the task.

The block in which the task was attached terminates.


Execution of a STOP statement in any task will terminate the major task
and all subtasks.


Multitasking considerations:


IBM PL/I frees all storage allocated by a subtask and closes all files
opened by the subtask when the subtask terminates.  Currently Iron Spring
PL/I does not do this, although that will be implemented in a future
release.  In the meantime a task should close its own files if desired and free any
storage it allocates.

Access to memory shared between tasks should be controlled by mutex
semaphores unless the memory is truely read-only by all tasks.
See
User-callable library procedures
 for mutex semaphore procedures.

Memory caching and
cache coherence is handled differently by various CPUs and PL/I multitasking
has been tested on only a small number.  To report bugs related to multitasking
please include CPU information such as can be obtained from Linux
"About this Computer."

All PL/I library procedures should be
thread-safe
.  If a subtask calls
subroutines written in other languages, it is the programmer's responsibility to
insure that they are thread-safe.


IBM-format packed decimal data

IBM mainframe PL/I stores FIXED DECIMAL data as"packed decimal",
big-endian two digits per byte with a trailing sign.  Signs 'A'x, 'C'x, 'E'x
and 'F'x indicate positive numbers; 'B'x and 'D'x are negative.  Packed
decimal numbers can be up to 16 digits.


Iron Spring PL/I supports this format for compatibility purposes, for
data transfer to and from mainframe systems.  To declare a packed decimal
field the attributes are
FIXED DECIMAL(p[,q]) OPTIONS(IBM)
 where
p and q are the precision and scale as usual. The field will occupy ceil((p+1)/2) bytes
with byte alignment only. OPTIONS(IBM) may be specified on a major or minor
structure to indicate that all FIXED DECIMAL data in the structure is in packed format.


Use of IBM-format packed decimal data for other than input and output is

not recommended
, because this data is converted by a library procedure
each time it is used. Move packed decimal data to and from standard FIXED DECIMAL fields
(without "OPTIONS(IBM)") before using in computations.


Example
:


DECLARE x FIXED DEC(7) STATIC INIT( 443322 ) OPTIONS(IBM);

will generate a four byte field containing '0443322C'x.


Builtin Functions and Pseudovariables

Iron Spring PL/I implements the following builtin functions.  Functions
flagged with an '*' are described below, all others are as described in the
IBM PL/I for MVS and VM PL/I Language Reference. Some builtins may have temporary
restrictions on arguments which are not documented here. Aggregate arguments are
not allowed.


Arithmetic builtins


ABS
DECIMAL
IMAG
PRECISION


ADD
DIVIDE
MAX
REAL


BINARY
FIXED
MIN
*ROUND


CEIL
FLOAT
MOD
SIGN


COMPLEX
FLOOR
MULTIPLY
TRUNC


CONJG


String-handling builtins


*AFTER 
*CENTRERIGHT 
LOW 
*SEARCH 


*BEFORE 
CHAR 
*LTRIM 
*STRING 


BIT 
*COPY 
MAXLENGTH 
TRANSLATE 


BOOL 
HIGH 
REPEAT 
*TRIM 


*CENTERLEFT 
INDEX 
*REVERSE 
UNSPEC 


*CENTERRIGHT 
*LEFT 
*RIGHT 
VERIFY 


*CENTRELEFT 
LENGTH 
*RTRIM 


Mathematical builtins


ACOS 
COS 
EXP 
SIND 


ASIN 
COSD 
LOG 
SQRT 


ATAN 
COSH 
LOG10 
TAN 


ATAND 
ERF 
LOG2 
TAND 


ATANH 
ERFC 


d>
SIN 
TANH 


Array-handling builtins


ALL
HBOUND
PROD


ANY
LBOUND
SUM


DIM
*POLY


Condition-handling builtins


ONCHAR
ONCOUNT
ONSOURCE


ONCODE
ONFILE

ONLOC


Storage control builtins


ADDR
CURRENTSTORAGE
POINTERVALUE


*ALLOCATE
ENTRYADDR
SIZE


ALLOCATION
NULL
STORAGE


BINARYVALUE
POINTER
SYSNULL


CURRENTSIZE
POINTERADD


Event builtins


COMPLETION
STATUS


Input-output builtins


COUNT
LINENO


Integer manipulation builtins


*IAND
*INOT
*ISLL


*IEOR
*IOR
*ISRL


*IBSW


Miscellaneous builtins


*BYTE 

*CS 

*HEX 

*HEXIMAGE 


*SIGNED 

*COLLATE 

*DATE 

PLIRETV 


*TIME 

*COMPARE 

*DATETIME 
*RANDOM 


*UNSIGNED 


Builtin subroutines


PLIDUMP
*PLIFREE
PLIRETC


*PLIFILL
*PLIMOVE
*PLISRTD


Additional Builtin Functions and Pseudovariables

This section lists only builtins in Iron Spring PL/I either not present in
IBM PL/I for MVS and VM or implemented differently.
The definitions are taken from IBM PL/I for OS/2 [OS2], IBM Enterprise PL/I [ENT], VAX PL/I [VAX],
Multics PL/I [Multics], or ANSI PL/I [ANSI].


AFTER(s,c) 
AFTER returns a nonvarying string containing all
  the characters or bits in the string value of "s" that follow the first occurrence of the string
  value of "c".
  If "c" does not occur within "s", if "s" is the null string, or if no
  characters or bits in "s" follow "c", the result is the null string.
  If "c" is the null string the result is "s".
  In the current version "s" and "c" must be or convert to character strings.
   [Multics][ANSI]

 


ALLOCATE(n) 
ALLOCATE allocates n bytes of storage and returns a pointer to
    the first byte. [OS2][ENT]

 


BEFORE(s,c) 
BEFORE returns a nonvarying string containing all
  the characters or bits in the string value of "s" that precede the first occurrence of the string
  value of "c".
  If "c" does not occur within "s", if "s" is the null string, or if no
  characters or bits in "s" precede "c", the result is the null string.
  If "c" is the null string the result is the null string.
  In the current version "s" and "c" must be or convert to character strings.
   [Multics][ANSI]

 


BYTE(n) 
BYTE returns a character string of length one equivalent to the
    following:

SUBSTR( COLLATE(), MOD(n,256)+1, 1 )
  [ENT][VAX]

 


CENTERLEFT(x,y[,z]) 


    CENTERLEFT returns a character string of length y, with character
    string x centered (or one position to the left of center),
    padded on the ends with the character z as needed.
    Y must be able to be converted to a FIXED BINARY value. Z should be
    a single character.
    If z is omitted, a space will be used. [OS2][ENT]

    CENTRELEFT is a synonym for CENTERLEFT.

 


CENTERRIGHT(x,y[,z]) 


    CENTERRIGHT returns a character string of length y, with character
    string x centered (or one position to the right of center),
    padded on the ends with the character z as needed.
    Y must be able to be converted to a FIXED BINARY value. Z should be
    a single character.
    If z is omitted, a space will be used. [OS2][ENT]

    CENTRERIGHT is a synonym for CENTERRIGHT.

 


COLLATE 
COLLATE Returns a character string of length 256 containing all 256 possible
    character values. [OS2][ENT][VAX]

 


COMPARE(x,y,z) 
COMPARE compares "z" bytes at locations pointed to by "x" and "y".
    It returns 0 if the strings are identical, <0 if x<y, and >0 if x>y.
    [OS2][ENT]

 


COPY(x,y) 
COPY returns a string consisting of "y" concatenated copies of string "x".
    [OS2][ENT][VAX]

 


CS(p,q,x) 
CS executes a hardware "CMPXCHG" instruction and returns a
    FIXED BINARY(31) value indicating whether or not the exchange was accomplished.
    q is a pointer expression which yields the address of a FIXED BINARY(31) variable q'
    to be set by the exchange,
    p is a pointer expression which yields the address of a FIXED BINARY(31) variable p'
    which is to be compared with q',
    and x is an expression which yields the new FIXED BINARY(31)
value
 to be set.
    If p' and q' are equal x replaces q',  and the CS builtin returns 0 (success).
    If they are unequal q' replaces p' [to set up for another call to CS] and the builtin
    returns 1 (failure).
    This provides an atomic "compare and swap" operation for thread or process synchronization.
    [ENT]

 


DATE, DATETIME, TIME 
IBM PL/I for
    MVS & VM 2.3 states "The time zone and accuracy are system dependent."
    These builtins return the system date and time as determined by the
    operating system, usually either UTC or local time.

 


HEX(x[,y]) 
HEX returns a character string that is the hexadecimal
    representation of the storage that contains 'x'.  If 'y' is omitted, the length of the returned
    string is 2*currentsize(x).  If 'y' is present it must be CHARACTER(1) nonvarying and the value of 'y'
    is inserted between every set of eight characters in the output string.  In this case the length of
    the returned string is (2*currentsize(x)) + floor((currentsize(x)-1)/4).
    Note: This function may not return an exact image of x in storage. If an exact image is required,
    use the HEXIMAGE built-in function.
    [OS2][ENT]

 


HEXIMAGE(x,y[,z]) 
HEXIMAGE returns a character string that is the hexadecimal
    representation of 'y' bytes of storage at location 'x'.  If 'z' is omitted, the length of the returned
    string is 2*y.  If 'z' is present it must be CHARACTER(1) nonvarying, and the value of 'z' is inserted
    between every set of eight characters in the output string.  In this case the length of the returned
    string is (2*y) + floor((y-1)/4). [OS2][ENT]

 


IAND(x,y...) 
IAND returns a FIXED BINARY(31) value representing
    the logical AND of its arguments.  All arguments are converted to FIXED BINARY(31).
    [OS2][ENT]

 


IBSW(n) 
IBSW returns a FIXED BIN(32) UNSIGNED value obtained
    by swapping the bytes in its argument, n. The argument is converted to UNSIGNED FIXED BINARY(32)
    using the normal rules of PL/I and the bytes are swapped to reverse the endianness. This builtin is
    not available with any other PL/I compiler.

 


IEOR(x,y) 
IEOR returns a FIXED BINARY(31) value representing
    the logical EXCLUSIVE OR of its arguments.  All arguments are converted to FIXED BINARY(31).
    [OS2][ENT]

 


INOT(x) 
INOT returns a FIXED BINARY(31) value representing
    the logical NOT of its argument.  The argument is converted to FIXED BINARY(31).
    [OS2][ENT]

 


IOR(x,y...) 
IOR returns a FIXED BINARY(31) value representing
    the logical OR of its arguments.  All arguments are converted to FIXED BINARY(31).
    [OS2][ENT]

 


ISLL(x,n) 
ISLL returns a FIXED BINARY(31) value representing
    x logically shifted left n bits.  All arguments are converted to FIXED BINARY(31).
    The result is undefined if n is greater than 31 or less that 0.
    [OS2][ENT]

 


ISRL(x,n) 
ISRL returns a FIXED BINARY(31) value representing
    x logically shifted right n bits.  All arguments are converted to FIXED BINARY(31).
    The result is undefined if n is greater than 31 or less that 0.
    [OS2][ENT]

 


LEFT(x,y[,z]) 


    LEFT returns a character string of length y, with character
    string x left-justified and
    padded on the right with the character z as needed.
    Y must be able to be converted to a FIXED BINARY value. Z should be
    a single character.
    If z is omitted, a space will be used. [OS2][ENT]

 


LTRIM(x[,y]) 
LTRIM returns the character value
    of 'x' with all leading characters in the character value of 'y' trimmed.  If 'y'
    is omitted a string consisting of a single blank character is used.  If 'y' is the
    null string no trimming occurs.  If 'x' is the null string a null string is returned.
    [Multics][VAX]

 


MAXLENGTH(x) 
MAXLENGTH returns the maximum length of the
    string value of "x".  "x" must be computational; if it is not a string it is
    converted to a
CHARACTER
 string before the length is calculated.  This is different
    from the LENGTH builtin that converts BINARY arguments to
BIT
 strings, but is
    consistent with IBM Enterprise PL/I. [OS2][ENT][VAX]

 


PLIFILL(x,y,z) 
PLIFILL moves "z" copies of a single byte "y" to a
    location pointed to by "x" with no conversion, padding, or truncation.
    No checking is done. [OS2][ENT]

 


PLIFREE(p) 
PLIFREE frees storage at location "p" allocated by the ALLOCATE builtin.
    No checking is done as to the validity of the area being freed. [OS2][ENT]

 


PLIMOVE(x,y,z) 
PLIMOVE moves "z" bytes to the location pointed to by "x"
    from the location pointed to by "y". No checking is done. [OS2][ENT]

 


POLY(x,y) 
POLY forms a polynomial expansion from its arguments.
    x must be a one-dimensional array (vector), y may be a vector or a scalar element.
    the result is x1 + x2*y1 + x3*(y1*y2)...

    If y is a scalar, its value is reused, giving x1 + x2*y + x3*y**2 ...

    If there are more elements in y than x, the excess will be ignored. If there
    are fewer, the last element will be reused as needed.
    x and y must be computational, and will be converted to FLOAT. The result is FLOAT with the
    base and mode determined by the rules for expression evaluation, and the
    precision of the longest argument.


    This is an IBM-only builtin function, and various IBM compilers function
    differently; in particular, several only allow y to be a scalar.

 


RANDOM[(s)] 
RANDOM returns a pseudo-random number
    between 0e0 and 1e0 as a FLOAT BINARY(52) value.  "s" is an optional

seed
 to start or restart the
    sequence. The value of "s" will be converted to FIXED BINARY(31). If "s" is omitted the number returned is based on the seed
    generated by the previous call, or 1 if a seed was never specified.  "s" must be
    greater than zero and less than 2,147,483,646 or the ERROR condition will be
    raised. RANDOM maintains one seed per program, regardless of the number of tasks.


    RANDOM uses the "multiplicative congruential method" (compatible with IBM
    Enterprise PL/I) to generate values:

       seed(x)   = mod( 950706376 * seed(x-1), 2147483647 )

       random(x) = seed(x) / 2147483647

    [OS2][ENT]

 


RANK(c) 
RANK returns a FIXED BINARY(15) result.  "c" is a
    character string of length one.  RANK(c) is equivalent to the following:

    INDEX( COLLATE(), c ) - 1
  [ENT][VAX]

 


REVERSE(s) 
REVERSE returns a nonvarying string containing all
    the characters or bits in the string value of "s" in reverse order.  In the
    current version "s" must be or convert to a character string.[OS2][ENT][Multics][ANSI][VAX]

 


RIGHT(x,y[,z]) 


    RIGHT returns a character string of length y, with character
    string x right-justified and
    padded on the left with the character z as needed.
    Y must be able to be converted to a FIXED BINARY value. Z should be
    a single character.
    If z is omitted, a space will be used. [OS2][ENT]

 


ROUND(x,y) 


    ROUND returns the value of x rounded in the y'th digit.
    If y is zero the last digit is rounded. The result has
    the same base, scale, and mode as x.


    If x is FLOAT, ROUND functions the same as in IBM Enterprise PL/I
    with the option "USAGE(ROUND(IBM))", that is, it is a no-op, and
    x is returned.

 


RTRIM(x[,y]) 
RTRIM returns the character value
    of 'x' with all trailing characters in the character value of 'y' trimmed.  If 'y'
    is omitted a string consisting of a single blank character is used.  If 'y' is the
    null string no trimming occurs.  If 'x' is the null string a null string is returned.
    [Multics][VAX]

 


SEARCH(x,y) 
'x' and 'y' are converted to character strings.
    SEARCH returns the position in the
    character value of of 'x' of the first matching character in the
    character value of 'y'. Characters in 'y' are taken left to right,
    and the position of the first match in 'x' is returned.
    If 'x' or 'y' is the null string or if none
    of the characters in 'y' occurs in 'x', zero is returned.
    [ENT][Multics][OS/2][VAX]

 


SIGNED(x[,p[,q]]) 
'x' is converted to
    SIGNED FIXED BINARY(p,q). If p is omitted it defaults to the maximum
    size of a signed fixed binary value. If q is omitted it
    defaults to zero. [ENT][OS2]

 


STRING(x) 
The STRING
builtin function
 returns a string that is the
      concatenation of all elements of 'x'. Note that this is
not
 identical to the STRING builtin from IBM PL/I
      for MVS&VM 2.3 for all arguments.


      'x' is reference to a variable that is suitable for bit-string or character-string
       overlay defining. A variable is suitable if it consists entirely of
       characters or bits, and these characters or bits are packed into adjacent
       storage locations, without gaps.


       The STRING
pseudovariable
 assigns a string to 'x' as if 'x' were a string scalar.
       'x' must be a reference to a variable that is suitable for character-string (or bit-string)
       overlay defining.

       'x' must designate one of the following:


A scalar string variable.

An aggregate variable whose components are all string variables of one
            type (either 'character' or 'bit', but not a mixture of 'character'
            and 'bit')

            If the total number of bytes or bits occupied by the variable designated
            by 'x' is m, then the pseudovariable is interpreted as follows:


     The value assigned to the pseudovariable is converted to
     'bit(m)' or 'char(m)', depending on the type of 'x'. If 'x' is a
     scalar varying string, the result is varying, otherwise it is a
     fixed-length string.

     The converted value is then assigned directly to 'x'.

    [Multics][VAX]

 


TRIM(x[,y[,z]]) 
TRIM returns the character value
    of 'x' with all leading characters in the character value of 'y' and all
    trailing characters in the character value of 'z' trimmed.  If 'y' and/or 'z'
    are omitted a string consisting of a single blank character is used.  If 'y'
    or 'z' is the null string no trimming at the corresponding end occurs.
    If 'X' is the null string a null string is returned. [OS2][ENT][VAX]

 


UNSIGNED(x[,p[,q]]) 
'x' is converted to
    UNSIGNED FIXED BINARY(p,q). If p is omitted it defaults to the maximum
    size of an unsigned fixed binary data value. If q is omitted it
    defaults to zero. [ENT][OS2]


The PLISRTx builtins

Iron Spring PL/I provides builtin functions to perform a sort function.
These are similar, but not exactly identical, to the corresponding routines
supplied by IBM PL/I.
Only the function PLISRTD is currently available
 to
perform a sort of data supplied by a user-written procedure and passing
 sorted data to a second user-written procedure.  Iron Spring PLISRTD
 performes an in-memory sort, so sufficient storage must be available.
 Full documentation of PLISRTD is available in the document

Iron-SpringSortRoutine.pdf
.


User-callable PL/I Library Procedures


Most of the procedures in the PL/I runtime library (prf.lib or libprf.a) are for use by the compiler
and are not user-callable.  The following, however, may be useful:


gmtime


mktime


osdelete


osgetenv


osgetprocinfo


oskillthread


syscall
 [Linux].

sysid


tempnam


Mutex semaphore procedures


hexdump


gmtime
 converts a calendar time
     (seconds since January 1, 1970)
     to date and time in a tm structure.

DECLARE gmtime ENTRY( PTR, PTR )
               RETURNS( PTR )
               EXT( '_pli_GMTime' );
DECLARE pTm   PTR;
DECLARE pTime PTR;
pTm = gmtime( addr(time), addr(tm) );


time
 a fixed bin(31) field containg the calendar time.

tm
 a tm structure which will contain the result.

pTm
 a pointer which will contain the address of the tm
field as passed, or sysnull if an error occurred.


The definition of tm can be found in the library include directory
as "tm.inc".

No check is made that the
area supplied is long enough to contain all the data.


Note that this procedure is the equivalent of unix' gmtime_r.


mktime
 converts time and date from a
     tm structure to calendar time.

DECLARE mktime ENTRY( PTR )
               RETURNS( FIXED BIN(31) )
               EXT( '_pli_MKTime' );
DECLARE pTm   PTR;
DCL     time  FIXED BIN(31);
time = mktime( addr(tm) );


time
 a fixed bin(31) field which will contain the calendar time,
             or -1 if an error occurred.

pTm
 The address of a tm structure containg the date to be converted.


The definition of tm can be found in the library include directory
as "tm.inc".

The last three fields- wday, yday, and isdst- are ignored.


osdelete
 deletes a file by name.

DECLARE osdelete ENTRY( CHAR(*) VAR )
                 RETURNS( FIXED BIN(31) )
                 EXT( '_pli_OSDelete' );
DECLARE rc FIXED BIN(31);
RC = osdelete( filename );


filename
 a character string variable or constant containing
the name of the file to be deleted.

rc
 a FIXED BIN(31) data element which will contain the
error code from DosDelete.

0 indicates no error.

2 indicates that the file was not found.

for other errors see the
OS/2 Operating System Control Program
Guide and Reference
.


osgetenv
 returns the address of the value
of a system environment variable.

DECLARE osgetenv ENTRY( PTR )
                 RETURNS( PTR ) )
                 OPTIONS( LINKAGE(SYSTEM) )
                 EXT( '_pli_OSGetEnv' );
DECLARE (p,q) PTR;
DECLARE EnvVarName char(16);
EnvVarName = 'TZ' || '00'x;
q = addr(EnvVarName);
p = osgetenv( q );


q
 a pointer to the null-terminated nonvarying name of an environment
variable whose value is to be returned.

p
 a pointer which will contain the address of the null-terminated
nonvarying value of the named environment variable, or sysnull() if the
variable was not found;


osgetprocinfo
 returns information about
 the current process and thread.

DECLARE osgetprocinfo ENTRY( PTR )
                 RETURNS( FIXED BIN(31) )
                 OPTIONS( LINKAGE(SYSTEM) )
                 EXT( '_pli_OSGetProcInfo' );
DECLARE ret_len FIXED BIN(31);
DECLARE pProcInfo PTR;
ret_len = osgetprocinfo( pProcInfo );

A structure PLIProcInfo.inc is declared in lib/include/pliprocinfo.inc
for use with this procedure. Note that the structure is declared BASED,
the caller should declare a STATIC or AUTO structure "like" PLIProcInfo.

before calling osgetprocinfo, the caller should set pi_len to the length
of the data to be returned, from 4 to 40 bytes. pProcInfo in the call may be
ADDR(your_procinfo_structure), or a pointer to the structure. On return, the
structure will be filled in up to the value of pi_len, and the length of data
supplied will be returned.

pi_pid and pi_tid will be available for both Linux and OS/2. pi_uid and
pi_gid will be available only for Linux.


 dcl    1 PLIProcInfo         based,
          5 pi_len            fixed bin(31),      /* Size of area    00*/
	  5 pi_pid            fixed bin(31),      /* Process id      04*/
	  5 pi_tid            fixed bin(31),      /* Thread id       08*/
	  5 fil1              char(20),           /* Reserved        0C*/
	  5 pi_uid            fixed bin(31),      /* User id         20*/
	  5 pi_gid            fixed bin(31);      /* Group id        24*/


oskillthread

Kills a thread using DosKillThread [OS/2] or SIGTERM [Linux].

DECLARE tid FIXED BINARY(31);
         DECLARE kill_thread ENTRY( FIXED BINARY(31) )
                             OPTIONS( LINKAGE(SYSTEM) )
                             EXT( '_pli_OSKillThread' );
         CALL kill_thread(tid);


tid
the thread id of the thread to be terminated.


syscall
 performs a Linux system call.


DECLARE syscall ENTRY
                   RETURNS( FIXED BINARY(31) )
                   OPTIONS( ASM LINKAGE(SYSTEM) )
                   EXT( '_pli_Syscall' );


Syscall is called with a variable number of arguments, and returns either a
FIXED BINARY(31) value or a POINTER, depending on the specific call.  The first
argument is always the call number.  A description of Linux system calls is
beyond the scope of this
Guide
.  Zero to six additional arguments for
the call may be provided, all either FIXED BINARY(31) or POINTER.  The kernel
returns a single result or an errno.  If the call returns an error, the result
is the negative value of "errno".
If a POINTER result is expected use the PTRVALUE builtin to convert from
FIXED BINARY to POINTER.


sysid
 returns a pointer to a varying string indicating
the operating system the caller is currently running on.


DECLARE sysid ENTRY
                RETURNS( POINTER )
                OPTIONS( LINKAGE(SYSTEM) )
                EXT( '_pli_Sysid' );


The returned value points to a varying character string that may have any declared length.  This
The string is guaranteed contain
at least "
OS/2
" or
"
Linux
" (without the quotes).  Additional
information may follow, but this is currently undefined.


tempnam
 generates a unique name for a temporary file.


DECLARE tempnam ENTRY( CHAR(*) VAR, CHAR(*) VAR, CHAR(*) VAR )
                RETURNS( CHAR(260) VARYING )
                EXT( '_pli_Tempnam' );
DECLARE temp_filename CHAR(260) VARYING;
temp_filename = tempnam( sDir, sPfx, sSfx );


sDir
a VARYING CHARACTER string identifying the directory in which the
file is to be located.  If this is the null string the file will be located
in the current working directory.

sPfx, sSfx
VARYING CHARACTER strings containing the prefix (sPfx) and
the suffix (sSfx) to be attached to the generated name.  Either or both may
be the null string.  The generated name will be:


   'sPfxpppppppp-tttttttt-uuuuuuuusSfx'

where 'pppppppp' is the hex value of the current process id (pid),
'tttttttt' is the hex value of the current thread id (tid), and 'uuuuuuuu'
is a unique number within this process.  If another file with this name
already exists, the unique number is incremented until the generated name
does not confilct with an existing file.

temp_filename
In the example this represents a VARYING CHARACTER
string which will contain the generated unique name.


Mutex semaphore procedures.
 The following procedures
implement "mutex" ( mutual exclusion) semaphores:


_pli_mutex_init
Initialize a mutex.  calling sequence is

declare <name> entry(pointer, fixed bin(31))
             options( linkage(system) )
	     external( '_pli_mutex_init' );
call <name>(phMutex,init);
 where <name> is the declared name
of the entry whose external environment name is '_pli_mutex_init'. 'phMutex'
is a pointer to a FIXED BINARY(31) data item where the handle assigned to this
mutex is to be returned, and 'init' is a fixed bin(31) initial value for the mutex.
'init' is currently ignored for OS/2.  For Linux 'init' should be 0 for an unowned
mutex and 1 for owned.  A mutex must be initialized before it can be used.


_pli_mutex_destroy
Free a mutex.  The calling sequence is

declare <name> entry(pointer) options( linkage(system) ) external( '_pli_mutex_destroy' );
call <name>(phMutex);
 where <name> is the declared name
of the entry and 'phMutex'
is a pointer to a FIXED BINARY(31) data item where the handle assigned to this
mutex is stored.  A mutex should be destroyed when it is no longer needed.
The mutex should not be active when destroyed.


_pli_mutex_wait
Acquire a mutex.  The calling sequence is

declare <name> entry(pointer) options( linkage(system) ) external( '_pli_mutex_wait' );
call <name>(phMutex);
 where <name> is the declared name
and 'phMutex'is a pointer to a FIXED BINARY(31) data item where the handle
assigned to this mutex is stored.  If the mutex is not owned by another caller
this task is given ownership and continues execution.  If it is owned, this
task waits until the mutex is posted.  An attention interrupt will also terminate
the wait and raise the ATTENTION condition.


_pli_mutex_timedwait
Acquire a mutex but only wait a specified time.
The calling sequence is

declare <name> entry(pointer,pointer) options( linkage(system) ) external( '_pli_mutex_timedwait' );
call <name>(phMutex,pTimespec);
 where <name> is the
declared name, 'phMutex'is a pointer to a FIXED BINARY(31) data item where the handle
assigned to this mutex is stored, and 'pTimespec' is a pointer to a structure defined by
'timespec.inc'.  If the mutex is not owned by another caller
this task is given ownership and continues execution.  If it is owned, this
task waits until the mutex is posted.  If the time expires while waiting the wait
is terminated; an attention interrupt will terminate the wait and raise the ATTENTION condition.


_pli_mutex_post
Release a mutex.  The calling sequence is

declare <name> entry(pointer) options( linkage(system) ) external( '_pli_mutex_post' );
call <name>(phMutex);
 where <name> is the declared name
and 'phMutex'is a pointer to a FIXED BINARY(31) data item where the handle
assigned to this mutex is stored.  Post is called  by the task owning
the mutex to release ownership and wake up any waiting tasks.


hexdump
 generates a hex/character
dump of a selected area of storage.

DECLARE hexdump ENTRY( PTR, FIXED BIN(31), CHAR(80) VAR )
                EXT( '_pli_Hexdump' );
call hexdump( pArea, lArea, sTitle );


pArea
 a pointer to the area to be dumped.


lArea
 the length of the area to be dumped, may be STG(area) if the area
is a structure.


sTitle
 the title to be used to identify this snapshot. sTitle="abc" will
use the heading "DUMP OF AREA 'abc'".


Preprocessor Facilities


This section describes the preprocessor facilities currently built in
to the compiler. The beta version of the full preprocessor is described
elsewhere.

The current built-in preprocessor is limited to a subset of the IBM preprocessor statements.
The listing control statements %PAGE, %SKIP, %PRINT, and %NOPRINT, the %INCLUDE statement,
and the %REPLACE statement are described below.


%INCLUDE:


The %INCLUDE statement instructs the compiler to read an external file of text and insert its
    contents in place of the %INCLUDE.  The syntax is: "%INCLUDE <filespec>;".

    <filespec> is any valid file specification.  If no file extension is coded the
    compiler will search for files having the extensions ".cpy", ".inc", or no extension.  The
    search path for the file is specified on the PLIC command (see
Running the compiler
).


    When running on Linux, the compiler will first search for an exact case match; if not found the first
    matching file in any case will be included.
    Examples of the %INCLUDE statement are:

    %INCLUDE ABC; [will include "ABC", "ABC.CPY", or "ABC.INC" in your search path -
               on Linux, if no file ABC[.x] is found, will include the first
               found of Abc[.x], abc[.x], etc.]
    %INCLUDE "def.pli"; [will include "DEF.PLI" in your search path]
    %INCLUDE "c:\pli\includes\ghi.pl1"; [will include the named file only]


    Nested includes are allowed, that is, the included file may itself contain %INCLUDE statements.
    The maximum depth of nesting is four levels.

 


%REPLACE:


The %REPLACE preprocessor statement provides limited text substitution
    capability. The syntax is "%REPLACE <identifier> BY <constant>;".

    <identifier> is any word which would be valid as an identifier in a PL/I program.
    PL/I keywords are not allowed.

    The scope of the %REPLACE statement is between its occurrence in the input stream to
    the end of the source program, including any included files.  The effect is to substitute
    the value of <constant> in the program anywhere <identifier> appears.

    In the default Enterprise PL/I compatibility mode,
    <constant> is any valid character-string, bit-string, or arithmetic constant.

    For example, the following sequence of statements:

 %REPLACE abc by 1;
     put edit(3+abc)(f(5));

    will result in the value 4 being output by the PUT statement.

    The %REPLACE statement above is equivalent to the following, but does not require a full
    preprocessor scan:

 %DECLARE abc CHARACTER;
     %abc = '1';


    In
extended replacement mode
, a replacement character string may be a sequence of
tokens

    words, strings, numbers, operators, etc. This string is processed as if it had appeared in
    place of <word>, If the replacement text contains strings, the must follow the rules
    for a quoted string within a string, for example '''stuff''' or '"stuff"'.


    %replace word by "fixed binary(31)";
    declare a word;


    effectively generates the following declaration: declare A fixed bin(31,0);

    To use extended replacement mode, use the following statement

%PROCESS OPTIONS(EXTENDED_REPLACE);

    The scope of this statement is from its occurrence in the source text to the end
    of the compilation unit, or until a

%PROCESS OPTIONS(NO_EXTENDED_REPLACE};

    statement is encountered.

 


%PRINT:


The %PRINT statement turns on (enables) printing of the source listing.  The %PRINT statement itself
    will not be printed if the listing was previously disabled.


%NOPRINT:


The %NOPRINT statement turns off (disables) printing of the source listing.  The %NOPRINT statement
    itself will be printed if the listing was previously enabled.


%PAGE:


The %PAGE statement causes a page eject in the source listing after printing of the
    %PAGE statement, if the listing is enabled.


%SKIP:


The %SKIP[(n)] statement will print 'n' blank lines (n=1 to 9) in the source listing following
    the printing of the %SKIP statement, if the listing is enabled.  If 'n' is omitted, the default
    is one line.


%CONTROL:


%CONTROL is a listing control statement from the IBM PL/I
    Checkout compiler. It is recognized and ignored.


Run-time considerations


Accessing unprocessed command arguments

Iron Spring PL/I converts command-line arguments passed by the Linux shell into a varying
character string for compatibility with other PL/I compilers. To access the
unprocessed argc and argv information include the member "argc.inc" from the
PL/I runtime library. For OS/2, COMMAND.EXE provides only the program name and
the unparsed argument string only as arguments.

     /* %include argc.inc includes the following declarations */
     dcl 1 argc_s             ext( '_pli_argc' ),
          5 argc              fixed bin(31),      /* Arg count        */
          5 ppargv            ptr,                /* **argv           */
          5 ppenv             ptr;                /* **envp           */

     dcl arg_addr        (0:1)ptr        based;   /* *argv[]          */


Run-time messages

All run-time messages are written to stderr:


cccc condition raised at address aaaa

  in procedure with entry eeee

  [at line number nnnn in compile unit pppppppp]

  Thread =           tttt

  [variable information]

Issued by: _pli_SIG

Cause: Condition "cccc" was raised, either by a PL/I SIGNAL statement or by
the occurrence of an error. "aaaa" is the address where the error occurred,
"eeee" is the name of the active entry in the procedure in which
the error occurred.


The next line is displayed only if the program was compiled with the "-N"
option. "nnnn" is the source line number where the error occurred, and "pppppppp"
is the external procedure or package name .


"tttt" is the id of the thread in which the error occurred.

The "variable information" differs depending on the condition and the
operating system.


condition=xxxx

If the condition was CONDITION(name), xxxx is the
name of the user-defined condition.

ONFILE=ffff

If the error is associated with an
input-output statement, "ffff" is the name of the file.

System Code=hhhhhhhh tttttttt

For OS/2 system exceptions, "hhhhhhhh" and "tttttttt" are the hexadecimal value
and the description of the exception respectively.

For Linux signals "hhhhhhhh" is the signal number, and "tttttttt" is the text description
of the signal.


No WHEN clause selected in SELECT statement

and no OTHERWISE clause present

Issued by: _pli_OTH

Cause: User error.  None of the conditions specified in a
SELECT-statement were satisfied and the statement did not include an OTHERWISE.


Unrecognized intrinsic function call

Issued by: _pli_IIC

Cause: Either a compiler error or a mismatch between
the compiled program and the version of the runtime library used.


See
Debugging
 for SNAP and PLIDUMP output.


The MAIN Procedure

The symbol '_pli_Main' is an alias for the first entry point of the main procedure.
The actual entry point of a PL/I program is the runtime library function '_pli_Start', which has
the alias 'main' for compatibility with C.

The MAIN procedure is called with one argument: a
CHARACTER(*) VARYING
 string containing
the [possibly edited] image of the command line that invoked the program.  Since a
"command line" as such doesn't exist for Linux, this parameter is manufactured by
concatenating all of the "argv" values, separated by spaces.


The sample program 'numwrd.pli' is an example of a program that uses the command-line.


Memory utilization

Generated PL/I code does not modify STATIC storage except during program initialization.
All code is
reentrant
 unless the user program modifies STATIC storage in such
a way as to invalidate this.


PL/I programs use the stack for activation records for procedures and BEGIN-blocks, including
all AUTOMATIC storage.  The
heap
 is used for all allocated BASED and CONTROLLED storage, for
control information for error-handling, and for file information and buffers.


For OS/2 the stack size is taken
from the value stored with the executable, either by the linker, by EXEHDR, or a default minimal
value of 16K bytes.  Stack storage is allocated when the program is loaded, but is not
committed

until actually used.
Stack probes
 are employed to prevent traps when more than 4K is requested
at one time.  Linux allows all available memory below the address where the program is loaded
to be used for stack, and doesn't require stack probes.

For OS/2 the heap size is currently fixed at 1MB (1024K), although a future release will allow this value
to be respecified at run time.  In the interim, if a larger heap is required, the library procedure
INIT can be re-assembled specifying a larger value for the data item 'def_heap_size'.  Linux allows
all available memory above the end of the BSS section to be used for the heap.


The layout of the stack frame for one procedure or Begin-block is illustrated below.


                    ^
                    | |                                          |
                  +08 |  Arguments (see program linkage below)   |  Higher addresses
                      +------------------------------------------+        |
                  +04 |  Calling procedure's EIP                 |        |
                      +------------------------------------------+        |
               EBP+00 |  Calling procedure's EBP                 |        |
                      +------------------------------------------+        |
                  -04 |  PL/I control information                |        |
                    | |  [see library include file 'dsa.inc'     |        |
                    v |   for layout (subject to change)]        |        |
                      |           ...                            |        v
                      +------------------------------------------+  Lower addreses
                      |  AUTOMATIC non-adjustable data           |
                      |  for this block                          |
                      |           ...                            |
                      +------------------------------------------+
                      |  AUTOMATIC adjustable data               |
                      |           ...                            |
                      +------------------------------------------+
                      |  Temporaries, argument lists             |
                                  ...


Program Linkage

The standard PL/I calling sequence passes all arguments
by reference
, that is by
passing their addresses and, optionally, the addresses of their
descriptors
.
This means that changes to the values of parameters in the called procedure are reflected back
to the caller.
Descriptors are passed for structure, array, and string arguments;
normally descriptors are not passed for arithmetic
arguments, although PL/I runtime procedures may require a descriptor.  The library include
file 'desc.inc' shows the layout of descriptors for various types of data [subject to change
in future releases].  When the argument is a constant, an expression, or
the data type doesn't match the corresponding parameter, a
dummy argument
 is created
and placed on the stack; the called procedure is passed the address of the dummy. In this case,
changes to the values of parameters change the dummy argument and don't modify the actual argument.


Descriptors for non-adjustable data are normally stored in STATIC storage, descriptors for adjustable
data are created during block initialization and stored in the current stack frame.  Descriptors for
expressions whose attributes are not known at compile-time (e.g. SUBSTR(s,1,j) will be created on the
stack as required.


All arguments are pushed on the stack right to left (leftmost arguments in lower addresses).
The last [highest-addressed] argument passed points to an area on the stack to receive a possible
result.    The caller cleans up the argument list following return.

The
called
 program is responsible for saving registers EBP, and
EBX, ESI, and EDI.  It is the
caller's
 responsibility to save any other registers used.
If the called entry is specified with OPTIONS(ASM), the caller saves and restores the 80x87
FPU control register.  The calling program passes the size of the argument list in DWORDs
in AL.


Arguments for 'SYSTEM' and "OPTLINK' are passed
by value
.
All arguments, normally limited
to arithmetic data, pointers, and nonvarying strings, are evaluated and the
values
 are
passed to the called procedure; descriptors are not used.
Changes to the values of parameters do not cause changes in
the values of the corresponding arguments.
Returned values from function procedures may be returned on the stack or in EAX or ST(0).


Two other pieces of information are also passed to a called PL/I procedure.  The
static backchain

is passed in ESI, and the address of the
Program Global Table (PGT)
 is passed in EDI.
These are passed for both "system" and PL/I linkage unless the called entry is specified with OPTIONS(ASM).
The PGT is a vector table containing the addresses of some required runtime routines.  The
static
backchain
 is the EBP value of the block which
lexically
 contains the called procedure.
This allows the called procedure to reference automatic data belonging to containing blocks.  This is

not
 necessarily the same as the EBP value of the caller.  For example, consider the following:


  A: PROC;
    CALL B;
    B: PROC;
      CALL B;
      C: BEGIN;
         END;
      END B;
    END A;


A is always the 'containing' block of B, regardless of whether B is called from A or from itself recursively.
B is the containing block of block C.  Using the
static backchain
, C can address AUTOMATIC data from
B and A, and so on.


The stack layout for a standard PL/I call is illustrated below.


                      +--------------//--------------------------+                  Higher addresses
                      |  Storage for returned value              |                        |
                      +------------------------------------------+ <--------+             |
                                                                            |             |
 (optional, locator/  +------------------------------------------+          |             |
  descriptor pair     |  Address of descriptor                   | --->     |             |
  for returned value) +------------------------------------------+          |             |
                      |  Address of data                         | ---------+             |
                      +------------------------------------------+                        |
                                                                                          |
 (optional)           +-------------//---------------------------+                        |
                      |  Dummy arguments                         |                        |
                      |            ...                           |                        v
                      +------------------------------------------+                  Lower addreses

 (optional, locator/  +------------------------------------------+
  descriptor pair     |  Address of descriptor                   | --->
  for arguments       +------------------------------------------+
  requiring a         |  Address of data                         | --->
  descriptor -        +------------------------------------------+ <------+
  one per argument)                                                       |
                                                                          |
 (optional)           +-------------//---------------------------+        |
                      |  Saved values of 'live' registers        |        |
                      |  not saved by the called procedure       |        |
                      |             ...                          |        |
                      +------------------------------------------+        |
                                                                          |
                      +------------------------------------------+        |
                      | Argument list, one address per argument. |        |
                      | This address points to either the data   |        |
                      | or the address of the locator/           | -------+
                      | descriptor pair for the data.            |
                      | The last (highest addressed) argument    |   -or-
                      | identifies the returned value.           | -------> argument
         ESP ----->   +------------------------------------------+


EXTERNAL
data
 (not EXTERNAL ENTRY) is not shared between executable and DLL code, nor
among procedures in different DLLs.  For example, data declared

DECLARE a EXTERNAL POINTER;
 will identify different "a"s in
procedures linked into in the executable, in DLL "1", and DLL "2".
Within
 the executable
or any one of the DLLs all occurrences of that declaration will identify the same "a".
Each level-1 external data declaration generates a segment for the linker.


Oncodes


                  Oncode
Condition         Value
finish              4
error               9
name               10
record             20
transmit           40
key                50
endfile            70
undefinedfile      80
endpage            90
pending           100
stringsize        150
overflow          300
fixedoverflow     310
zerodivide        320
underflow         330
size              340
stringrange       350
area              360
attention         400
storage           450
condition         500
check             510
subscriptrange    520
conversion        600


Efficient Programming


Avoid use of the ENTRY statement.  The PACKAGE statement is a more efficient way of grouping
multiple externally-callable entry points in a single compile unit.
If you do have a procedure with multiple entries, try to minimize the number of RETURN statements.

Avoid use of data declared FIXED DECIMAL OPTIONS(IBM).

Any use of the GET and PUT statements, including GET STRING and PUT STRING,
in a statically-linked program causes library conversion and format-processing
code to be included.  If there are only have one or two GET or PUT statements,
consider using record I/O and formatting the data yourself.
The PL/I runtime library itself does not use any stream I/O.

Read and write as much data as possible with a single GET or PUT statement.
Instead of:

  PUT LIST(a);
  PUT LIST(b);
use:
  PUT LIST(a,b);


Use "adjustable" strings and arrays only when necessary.

The SUBSCRIPTRANGE condition is intended for debugging and it's use in a
    production program is discouraged.

Use the SIZE and STRINGSIZE prefixes only when necessary as they
    can generate a lot of code.
    Apply the prefix to the smallest piece of code necessary (procedure, block, or single statement).

When possible provide your own checks to prevent computational conditions such as
    FIXEDOVERFLOW for FIXED DECIMAL data rather than relying on an ON-unit.

Aggregate asssignment to very large AUTOMATIC arrays, with the initial
    attribute or in an assignment statement, may exceed the compiler's
    available storage.  Use the "PLIFILL" builtin if possible or initialize them in a loop.

The compiler "-N" option (statement offset table) requires 8 bytes per
    executable statement plus 10 bytes, plus the length of the external procedure
    or package name.

Array assignments by default generate inline code. For large arrays it is
    more storage-efficient to code a loop and assign the elements individually.
    Note the the compiler -O option will use a single block move instruction to
    assign an entire array if the source and target attributes match.

Minimize references to self-defining structures (REFER option) and
    iSub-defined arrays, since each reference involves a subroutine call.


Debugging PL/I programs


Debuggers

PL/I code has no special requirements for a debugger.  On OS/2, the WATCOM® debugger
may be used to debug PL/I programs;  on Linux gdb works well.

Source-level debugging is not yet available, but since the
compiler can generate assembly-language output, debugging
at the assembly level is straightforward.


Sometimes it may be desirable to compile-in a breakpoint
to be activated when a specific location is reached or
condition occurs.  The following statement inserted in your
source will activate a breakpoint when it is reached when running
under control of a debugger:

 *PROCESS DBG(INT3);

If the resulting executable is
not
 run under a debugger, the
interrupt will be ignored.  Nevertheless, INT3 traps should not be
left in a production program.


SNAP traceback

The PL/I statement "ON CONDITION(xxx) SNAP..." will generate a traceback
on stderr when the specified condition is raised.  Beginning with the procedure
generating the traceback (_pli_Trace) and working backwards, the trace
lists the address of the active entry point of the procedure, the address
of the statement in the calling procedure that called that entry, and
the name of the entry point if available.


PLIDUMP

The PL/I statement "CALL PLIDUMP( "<options>", "<title>" );
will generate debugging output on stderr when executed.

"title" is a character string used as the
title
 of the dump.

"options" provide one or more dump options:

'S' = terminate after dump

'C' = continue after dump

'T' = print
traceback

'B' = dump automatic storage.

'H' = dump static storage (not yet implemented).

'F' = dump opened file information (not yet implemented).

Options may be combined and may appear in
any order, for example "TCB" means: print trace, continue after snap, and dump
all automatic storage. Case is not significant. Unrecognized options are ignored.

The stack is dumped from the current procedure (_pli_Dump) [lowest address] to
the PL/I startup code (_pli_Init) [highest address].  Storage for each procedure is
dumped in three sections (see sample segment of PLIDUMP output below):

Temporaries
 (argument lists
and storage used during expression evaluation),

Data
 (AUTOMATIC variables and other non-temporary user data in
the stack frame),
and
DSA
 (system data in stack frame).  The layout of the DSA is
given in lib\include\dsa.inc.  This is subject to change in future releases.

If the dump is issued as result of a system exception or trap,
trap information

will appear immmediately before the DSA of the procedure in which the exception occurred.  The
line beginning "System Code" will display the system error code and description.


Each data line includes the
actual address of the first byte, the hex offset (±EBP for DSA and data, +ESP
for temporaries), up to sixteen bytes of data in hex as actually stored in memory
(no byte-swapping), and the ASCII representation of the same data.

The DSA display provides additional formatted information regarding the ON-conditions
enabled for this block and identifies runtime-library procedures.

Sample PLIDUMP output


***error dump***


Address  Caller   Entry name

           ... omitted from the example ...


OS/2 System Trap Information

  System Code=C0000005 XCPT_ACCESS_VIOLATION
  Exception Address=000100D7
  EAX 44000B00  EBX 00020000  ECX 00000000  EDX 44000B00
  ESI 00000000  EDI F8070200
  DS  0053      ES  0053      FS  150B      GS  0000
  CS:EIP 005B:000100D7  SS:ESP 0053:00048600  EBP 00048628
  EFLAGS 00012216

0001868A 00027402 FTPRO


Temporaries at 003C80C4

003C80C4 ( 000000) 74823C00 5C863C00 98823C00 3C813C00 |t.<.\.<...<.<.<.|
003C80D4 ( 000010) 53003C00 F4803C00 5C863C00 A8A2021C |S.<...<.\.<.....|
           ... omitted from the example ...
003C83B4 ( 0002F0) 3C843C00 00000000 3C843C00 C4833C00 |<.<.....<.<...<.|
003C83C4 ( 000300) 64140500                            |d...|


Data at 003C83C8

003C83C8 (-000074) 786D0500 00843C00 3E350400 E4C34300 |xm....<.>5....C.|
003C83D8 (-000064) 00000000 30823C00 0083E180 B8813C00 |....0.<.......<.|
003C83E8 (-000054) 0000E181 E4C34300 00000000 B5570400 |......C......W..|
003C83F8 (-000044) 31000000 10843C00 786D0500 80853C00 |1.....<.xm....<.|
003C8408 (-000034) 12720200 600F3C00 01001500 00001B00 |.r..`.<.........|
003C8418 (-000024) E0320500                            |.2..|


DSA at 003C841C

Enabled: CONV FOFL OFL UFL ZDIV
003C841C (-000020) C8833C00 0000E180 E4C34300 00000000 |..<.......C.....|
003C842C (-000010) 8A860100 31000000 48843C00 786D0500 |....1...H.<.xm..|
003C843C ( 000000) 80853C00 02740200                   |..<..t..|

00026C79 00010123 PH10


Temporaries at 003C8444

003C8444 ( 000000) 48843C00 11000000 42000000          |H.<.....B...|
           ... omitted from the example ...


Using the procedure map

The
procedure map
 can be used in conjunction with a runtime
error message, SNAP output, or PLIDUMP to determine the line in the program causing
the error.


If the error occurs in your PL/I program the runtime error message will identify the
source line causing the error, if the procedure map was generated at compile time.
If the error occurs in a library procedure this information is less meaningful [in a
 future release the message will identify the source line where the library
procedure was invoked]. Given the SNAP output or corresponding PLIDUMP information
and the link map the offset in your code that called the library procedure can be
computed. This offset can then be used with the procedure map to determine the
line number which originally caused the error.  The
offset
 in the map is the
offset in the program of the
first
 machine-language instruction for the
source line identified by
line
. The computed offset between that shown
in the map and the offset for the next line will identify the appropriate line number.
Program initialization code is included in a separate area of code labeled
"Initialization" following the END statement of the procedure or BEGIN-Block  for which
the initialization is being performed.

  Offset  Line Statement type             Offset  Line Statement type             Offset  Line Statement type
    3261  1187 PROCEDURE                B2D
    3276  1192 IF                           3285  1192 RETURN                       3294  1193 Assignment
    329D  1194 IF                           32AA  1194 DO                           32AA  1195 Assignment
    32AE  1196 Assignment                   32B7  1197 END                          32B7  1198 DO
    32ED  1199 IF                           3304  1199 LEAVE                        330C  1200 END
    3311  1201 IF                           331B  1201 Assignment                   3324  1202 RETURN
    3332  1203 END                          3337  1203 Initialization               3347  1214 END


version 1.4.1, 15 Apr 2026


http://www.iron-spring.com
