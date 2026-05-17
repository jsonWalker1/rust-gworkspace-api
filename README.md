### rust-gworkspace-api

V podstatě tohle je můj první větší experiment s Rust backendem. Zároveň i první hlubší práce kolem Google Workspace – hlavně Domain Wide Delegation, správa service accountů v GCP a nastavování správných scope oprávnění.

Backend komunikuje s Google Workspace API a umí vytvořit uživatele v testovacím prostředí. Původní myšlenka byla automatizace okolo ODOO – aplikace přijme JSON s ID zaměstnance, následně získá potřebné údaje a vytvoří účet ve Workspace.

Šel jsem na to poměrně přímočaře přes:

service account
key.json
JWT
Domain Wide Delegation
Google Admin SDK

Žádná magie, spíš snaha pochopit, jak spolu tyhle věci opravdu fungují pod kapotou.

Je mi jasné, že dneska se často jede přes OAuth2, SSO nebo robustnější identity řešení. Tenhle projekt ale vznikl hlavně jako experiment a způsob, jak si osahat backend, autentizaci a automatizaci kolem Google ekosystému.

A jo… Rust je pro takhle malou službu nejspíš overkill 😄. Ale cílem nebylo postavit enterprise řešení, spíš si rozšířit obzory a pochopit backend trochu víc do hloubky.

Aplikace má v podstatě jediné API, kam se pošle JSON s ID zaměstnance. Backend si následně stáhne potřebné informace a vytvoří účet v testovacím Google Workspace.

## Co jsem si tím osahal

- Rust backend základy 
- Google Workspace Admin SDK
- Domain Wide Delegation
- JWT a service account autentizaci
- Environment variables
- Práci s API a JSON payloady
- Google Cloud Platform konfiguraci
- Nastavení Google Workspace od nuly
- Ověření vlastnictví domény
- DNS záznamy a jejich propagaci
- Propojení vlastní domény s Google Workspace
- Správu oprávnění a scope pro service accounty
- Testovací prostředí pro automatické vytváření účtů

## Co bych teď udělal/použil jinak...

- Oauth
- určitě bych nepoužíval key.json
- volil bych automatizaci přes N8N 
- spoustu věcí se dá přímo integrovat v ODOO 