# Stugbokningssystem
Välkommen till stugbokningsystemet. Källkoden ligger under `src/`. Programmetär skrivet i Rust och Javascript. 

## Getting started
Starta `stugbokningssystem.exe` i rooten av denna mapp för att starta appen.
Det finns 4 användare: 
 - admin
 - guest
 - user1
 - user2

Alla har samma lösenord: `pwd`. 
När du startat appen besöker du [http://127.0.0.1:5800/](http://127.0.0.1:5800/). Där kan du antingen gå vidare till appen eller kolla på OpenAPI specen för applikationen. Om du går vidare till appen, dvs följer länken till [http://127.0.0.1:5800/app](http://127.0.0.1:5800/app) så kommer du behöva logga in med något av kontona ovan. Adminkontot är administratör och kan se alla stugor. De andra användarna kan se lediga stugor och sina egna stugor. För att testa flera olika användare samtidigt kan du använda inkognitofönster eller liknande. 

## Kör koden
Kompilera koden själv behöver du installera rust. Detta kan enkelt göras härifrån: [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install).

När det är installerat borde det bara vara att köra `> crago run` i denna mapp. Den kommer då ladda ner dependencies och kompilera (kan ta lite tid första gången går fortare andra gången) och starta programmet. Efter det kan du använda programmet precis som när du startar `.exe`-filen ovan, genom att besöka [http://127.0.0.1:5800/app](http://127.0.0.1:5800/app).

## Om systemet
Systemet använder [Salvo](https://salvo.rs/) som server och OpenAPI sppec-genererare samt en embeded [SurrealDB](https://surrealdb.com/) in-memory databas för att lagra användare och stugor. På frontend-sidan så används [Handlebars](https://handlebarsjs.com/) som ett enkelt templateingverktyg för att generera GUI. Då 4h är ganska kort tid behövde några genvägar tas, därav att koden ser lite lustig ut på vissa platser, jag ber om ursäkt för det, men jag tror den funkar ganska bra ändå. 
