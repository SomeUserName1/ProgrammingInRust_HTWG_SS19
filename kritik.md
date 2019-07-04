# Selbstevaluation
- Viel vorgenommen, noch weniger geschafft
- Scope: normalerweise ein Team von 10+ Menschen über 6 Monate Arbeit für ein minimum viable product 
- Beispiele:
    - [Witnet Roadmap](https://medium.com/witnet/an-updated-witnet-roadmap-to-mainnet-cb8543c534a4): 18 contributers, Dev start Sep '18, testnet seit Mai; unstable  
    - [Exonum Roadmap](https://exonum.com/\#roadmap_q1_2019): First issue on GitHub: Sep '17, Milestone für Q1 2019: Secure Storage for Public Key of the node, 47 Contributer \\
    - Bitcoins [Lightning Network](https://de.m.wikipedia.org/wiki/Lightning-Netzwerk}{Lightning Netzwerk): Implementiert 2018 um scalability zu verbesser aber: Probleme beim Routing, DDoS anfällig
 - Aktives Forschungsgebiet

## Bilanz
Vorgenommen: 
         blockchain mit beliebigen Transaktionsinhalten
         asynchroner p2p server
         signing und encryption
         (nicht explizit aber offensichtlich hilfreich: Persistenz)
    Geschafft (was ohne warning kompiliert, dokumentiert und getestet ist):
         locale PoC blockchain
         PGP lib eingebunden (aber nicht nutzbar um Transaktionen zu signen oder das Messages enum zu verschlüsseln)
         P2P server im Prinzip fertig aber hat noch \inline{Errors} (c.f. nächste Folie)

## Woran lag's?
 - Schlechte Aufgabenverteilung
 - Zu Spät angefangen
 - Wenig (oder besser gesagt kein) Vorwissen, daher
     - kein Überblick wie lange Aufgaben dauern
     - viel Einlesen, z.B. tokio 
     - Fehlende Zeit um das Auszugleichen bspw.: Lifetimes von in Closure geboundedten Werten in Futures  
    ```fn x() -> impl Future<Item=(), Error=io::Error>```
- Planung schwierig: ReSy bzw. 51 Credits \& 2 Nebenjobs und nicht genug Vorlauf bei den Abgabeterminen 

Da es keine Lehrevaluation gab, noch ein Satz dazu:  
Normalerweise: Dozent gibt Projekt-Daten inkl. Anforderungen und Termine konkret in der ersten Vorlesung an und schreibt das auf ins Ilias/Moodle.  
Hier: Per Mail, zu beliebig gewählten Zeitpunkten.
