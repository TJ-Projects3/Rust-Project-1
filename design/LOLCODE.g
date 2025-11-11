grammar LOLCODE;

HAI	:	'#HAI' | '#hai';
KTHXBYE	:	'#KTHXBYE' | '#kthxbye';
OBTW	:	'#OBTW' | '#obtw';
MAEK_HEAD
	:	'#MAEK HEAD' | '#maek head';
GIMMEH_TITLE : '#GIMMEH TITLE' | '#gimmeh title';
MKAY	:	'#MKAY' | '#mkay';
MAEK_PARAGRAF
	:	'#MAEK PARAGRAF' | '#maek paragraf';
MAEK_LIST
	:	'#MAEK LIST' | '#maek list';

OIC	:	'#OIC' | '#oic';
GIMMEH_BOLD
	:	'#GIMMEH BOLD' | '#gimmeh bold';
GIMMEH_ITALICS 
	:	'#GIMMEH ITALICS' | '#gimmeh italics';
GIMMEH_ITEM
	:	'#GIMMEH ITEM' | '#gimmeh item';
GIMMEH_NEWLINE
	:	'#GIMMEH NEWLINE' | '#gimmeh newline';
GIMMEH_SOUNDZ
	:	'#GIMMEH SOUNDZ' | '#gimmeh soundz';
GIMMEH_VIDZ
	:	'#GIMMEH VIDZ' | '#gimmeh vidz';
I_HAZ	:	'#I HAZ' | '#i haz';
IT_IZ	:	'#IT IZ' | '#it iz';
LEMME_SEE
	:	'#LEMME SEE' | '#lemme see';
TLDR	:	'#TLDR' | '#tldr';
TEXT 	: 	('A'..'Z' | 'a'..'z' | '0'..'9' | ',' | '.' | '"' | ':' | '?' | '!' | '%' | '/' | ' ' | '\t' | '\r' | '\n')+;
ADDRESS : 	('A'..'Z' | 'a'..'z' | '0'..'9' | ':' | '/' | '.' | '%' | '_' | '-')+;

lolcode	:	HAI body KTHXBYE;
body	:	(head? comment* content*);
head 	:	 MAEK_HEAD title OIC;
title 	: 	GIMMEH_TITLE TEXT MKAY;
comment : 	OBTW TEXT TLDR;
content	: 	paragraph 
	        | bold 
	        | italics
	        | listblock
	        | newline
	        | video
	        | audio
	        | varDefine
	        | varUse
	        | text
	        ;
paragraph 
	: MAEK_PARAGRAF paragraphContent* OIC;
bold 	: GIMMEH_BOLD TEXT MKAY;
italics : GIMMEH_ITALICS TEXT MKAY;
listblock
	:	MAEK_LIST listItem+ OIC;
listItem:	GIMMEH_ITEM TEXT MKAY;

newline	:	GIMMEH_NEWLINE;
video : GIMMEH_VIDZ ADDRESS MKAY;
audio : GIMMEH_SOUNDZ ADDRESS MKAY;
paragraphContent
	:	bold | italics | audio | video | newline | listblock | text;
varDefine
	:	I_HAZ TEXT IT_IZ TEXT MKAY;
text	:	TEXT;
varUse	:	LEMME_SEE TEXT MKAY;


	

	
