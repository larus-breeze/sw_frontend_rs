#import "../manual.typ": *

= #hr[Installation]
== #hr[Lieferumfang]

#tr[Folgende Teile sind enthalten:

- LARUS Vario Display
- Befestigungsschrauben
- 1:1 Standard-RJ45-Kabel
- Micro-SD-Karte mit Adapter
- D-Sub-9-Lötstecker und Gehäuse
- 1,5 mm Sechskantschlüssel zur Montage der Drehknöpfe]

== #hr[Schnellstartanleitung]

#tr[In einigen Fällen können die Standard-Einstellungen verwendet werden und das Gerät in einer
vereinfachten Art und Weise in Betrieb genommen werden. Dann ist es ausreichend, wenn folgende
Punkte beachtet werden:

+ Bitte demontieren Sie beide Drehknöpfe mit einem 1,5-mm-Sechskantschlüssel (im Lieferumfang 
  enthalten).
+ Befestigen Sie das LARUS Vario Display mit drei Schrauben in einer 57 mm großen Aussparung 
  der Instrumententafel. Es kann auch um 90°, 180° und 270° verdreht montiert werden.
+ Beide Drehknöpfe montieren
+ Verbinden Sie den CAN-Anschluss des LARUS Vario-Displays und der LARUS-Sensoreinheit mit
  im Lieferumfang enthaltenen 1:1-Patchkabel.
+ Schalten Sie LARUS ein.
+ Überprüfen Sie, ob das Satellitensymbol auf dem Bildschirm gelb oder grün ist und die aktuelle 
  Kursrichtung angezeigt wird.
+ Wählen Sie eine geeignete Polare für Ihr Segelflugzeug aus oder erstellen Sie eine.
+ Ihr LARUS Vario Display ist nun flugbereit.]	

== #hr[Design und Funktion]

#tr[Das LARUS Vario Display zeigt die von LARUS gemessenen und berechneten Daten an. LARUS ist ein
fortschrittliches Variometer mit Echtzeit-Windmessfunktion. Es verfügt über modernste Drucksensoren,
eine fortschrittliche IMU und GNSS-Empfänger, um präzise Flugdaten zu erfassen. Die wichtigsten
Merkmale des Displays sind:

- Rundes Display für 57-mm-Standard-Instrumententafelausschnitte
- Heller und farbenfroher Bildschirm
- Leichtes, kompaktes Design mit schwarz eloxiertem Aluminiumgehäuse
- Drehknopf mit zwei Ebenen und Drucktastenfunktion zum Ändern von Einstellungen und
            zum Aufrufen der Menüs 1,5-mm-Sechskantschlüssel.

Das LARUS Vario Display wird von Prof. Dr. Klaus Schaefer, Maximilian Betz, Winfried Simon,
Peter Simon und dem SteFly-Team entwickelt und kontinuierlich verbessert. Sie können sich gerne 
an der Entwicklung beteiligen und Verbesserungsvorschläge oder Probleme einbringen.]

#link("https://github.com/larus-breeze")[github.com/larus-breeze]

== #hr[Systemkonfigurationen]<system-configurations>
=== #hr[Konfiguration im Einsitzer]

#figure(
    image("/img/config-singleseater.jpg", height: 8cm),
    caption: [#hr[Anschlussplan Einsitzer-Konfiguration]],
)<single-seater-configuration>

#tr[Der Navigations-Rechner wird an das LARUS Vario Display angeschlossen (Option 1). Oftmals ist
dies die einfachste Variante. Alternativ kann der Navigationsrechner jedoch auch an die LARUS
Sensoreinheit angeschlossen werden (Option 2).]

=== #hr[Konfiguration im Doppelsitzer]

#figure(
    image("/img/config-doubleseater.jpg", height: 8cm),
    caption: [#hr[Anschlussplan Doppelsitzer-Konfiguration]],
)<double-seater-configuration>

#tr[Im Doppelsitzer werden die Navigationsrecher typischerweise an die LARUS Vario Displays
angeschlossen. Es ist aber genauso möglich, die Navigationsrechner an die LARUS-Sensoreinheit
anzuschließen. Die Daten werden zwischen den LARUS-Komponenten ausgetauscht und an die
Navigationsrechner verteilt, so dass keine Informationen verloren gehen.]

== #hr[Steckverbinder und Verkabelung]
=== #hr[Steckverbinder auf der Rückseite des Gerätes]

#figure(
    image("/img/connectors-overview.jpg", width: 6cm),
    caption: [#hr[Ansicht von der Rückseite des Gerätes]],
)<connectors-overview>

#tr[Auf der Rückseite des LARUS Vario Displays befinden sich die Anschlüsse für CAN, RS232 und die
Ein-/Ausgänge. Des Weiteren ist dort der Schlitz zum Einführen der SD-Karte, sowie die 3,5mm Buchse
des Audio-Ausgangs vorhanden.]

=== #hr[CAN- und RS232-Anschlüsse]

#figure(
    image("/img/connectors-rj45.svg", width: 6cm),
    caption: [#hr[Die Rj45 Anschlüsse im Detail]],
)<connectors-rj45>

#h(5mm)
#align(left, block[
    #figure(
        table(
            columns: (auto, auto, auto),
            align: (center, left, left),
            table.hline(),
            table.header(
                [*Pin*], [*CAN*], [*RS232*],
            ),
            table.hline(),
            [1], [GND (internally connected)],	[GND (internally connected)],
            [2], [GND (internally connected)],	[GND (internally connected)],
            [3], [NC], 						  	[RS232-1-RX],
            [4], [CAN Low], 					[RS232-1-TX],
            [5], [CAN High], 					[NC],
            [6], [NC], 							[NC],
            [7], [VCC [9-28V DC] ], 			[VCC [9-28V DC] ],
            [8], [VCC [9-28V DC] ], 			[VCC [9-28V DC] ],
            table.hline(),
        ),
        caption: [#hr[Pinbelegung CAN und RS232 RJ45]],
    )
])

=== GPIO / D-SUB 9

#tr[Über den D-Sub-Stecker können mehrere weitere Schalter, Sensoren und Geräte angeschlossen
werden. Die folgende Skizze zeigt die Ansicht in den Stecker des LARUS Vario Displays.]

#figure(
    image("/img/connector-dsub9.png", width: 6cm),
    caption: [#hr[DSUB-9 Anschluss]],
)<connectors-dsub9>

#h(5mm)
#align(left, block[
    #figure(
        table(
            columns: (auto, auto, auto),
            align: (center, left, left),
            table.hline(),
            table.header(
                [*Pin*], [*Name*], [*I/O, Ground*],
            ),
            table.hline(),
            [1], [GND],						[Ground],
            [2], [DI3 - Gear],				[Input],
            [3], [DI1 - Water Ballast], 	[Input],
            [4], [DO2], 					[Output],
            [5], [GND], 					[Ground],
            [6], [DI4 - Speed Breakes], 	[Input],
            [7], [DI2 - Speed to Fly], 		[Input],
            [8], [GND], 					[Ground],
            [9], [D01 - Canopy Flasher ], 	[Output],
            table.hline(),
        ),
        caption: [#hr[Pinbelegung GPIO / D-SUB 9]],
    )
])

#tr[Zur leichteren Identifizierung der Pins sind diese Nummern auch in die Buchse (im Lieferumfang
enthalten) eingeprägt. Nach der Verkabelung müssen die Einstellungen im LARUS Vario Display
vorgenommen werden.]

=== #hr[Audio] 

#tr[Eine Audiobuchse ist für den Anschluss eines Lautsprechers mit einem 3,5-mm-Klinkenstecker
verfügbar. Der Innenwiderstand des Lautsprechers muss zwischen 4 und 8 Ω liegen (max. output of 3
W bei 4 Ω).]

#picnote("/img/pictograph-yellow-warning.svg")[
    #tr[Ein einzelner Lautsprecher darf nicht an mehr als ein Gerät angeschlossen werden.]
]

=== #hr[SD-Karte]

#tr[Das Gerät verfügt über einen SD-Kartensteckplatz für Firmware-Updates.]

#picnote("/img/pictograph-yellow-warning.svg")[
    #tr[Da SD-Kartenerweiterungen das LARUS Vario Display beschädigen können, übernehmen wir 
    keine Haftung für Schäden, die durch deren Verwendung entstehen.]
]

=== #hr[Verkabelung]

#tr[Wenn eine serielle Verbindung (RS232) zwischen SteFly NAV und LARUS Vario Display hergestellt
wird, werden beispielsweise die MC-Werteinstellungen zwischen den Geräten synchronisiert/übertragen.
Darüber hinaus verarbeitet das LARUS Vario Display Eingaben von einem Speed-to-Fly-/Vario-Schalter,
der direkt mit dem SteFly-Fernsteuerungshebel verbunden ist. Die folgenden Schritte sind
erforderlich:

- Geräte mit Kabel verbinden:
  - Der CAN-Anschluss der LARUS Sensoreinheit muss über 1:1-Patchkabel mit dem CAN-Anschluss des 
    LARUS VARIO Displays verbunden werden (bei Doppelsitzerkonfigurationen unter Verwendung der Kabelbox).
  - Die Geräte, auf denen XCSoar läuft, können über ein gekreuztes Patchkabel RX/TX zwischen dem 
    RS232-Anschluss des Vario-Displays und dem ttySx-Anschluss des SteFly NAV verbunden werden.
- Schließen Sie optional Schalter für die Erkennung des Ablassvorgangs vom Wasserballast und die 
  Fahrwerkswarnung an den entsprechenden DSUB-9-Pin und einen beliebigen GND-Pin an. Für die 
  Fahrwerkswarnung haben Sie die Möglichkeit, zwei separate, direkt angeschlossene Schalter für 
  das Fahrwerk und die Bremsklappen zu verwenden (empfohlen) oder ein Kabel mit den beiden 
  Schaltern in Reihe zum DSUB-9-Anschluss.
- Der SteFly-Fernbedienungsstick kann auf zwei verschiedene Arten angeschlossen werden:
  - Abhängig von der Ausstattung: mit einem separaten Kabel zum DSUB-9-Pin 7
  - Verwendung des USB-Kabels (immer verfügbar)
- Bitte passen Sie die Einstellungen in XCSoar (Version 7.44 oder höher) / OpenSoar (7.43 oder 
  höher) an: Wenn das Vario-Display direkt an das SteFly NAV angeschlossen ist, wählen Sie bitte den 
  entsprechenden ttyS-Port aus, Baudrate 38400, Treiber Larus, Synchronisation (Option) mit 
  Gerät aktivieren.
- Bitte passen Sie die Einstellungen im LARUS Vario Display für die angeschlossenen optionalen 
  Schalter an.
  - Warnung bezüglich des Fahrwerks
  - Wasserballast
  - Konfiguration der Sollfahrt Umschaltung (falls nicht automatisch geschaltet):
    - Eingangs-Pin: Um einen externen Schalter (am Steuerknüppel, Klappenhebel usw.) zu verwenden, 
      stellen Sie die Vario-Steuerung auf „Eingangs-Pin“ und wählen Sie die richtige Einstellung       
      in der StF-Pin-Konfiguration. (active: when open/closed)
    - NMEA für SteFly-Fernsteuerungsstick. Zusätzlich zu den Einstellungen im Vario müssen Sie 
      eine Ereignisdefinitionsdatei in XCSoar/OpenSoar installieren (Konfiguration / System / 
      Ansicht / Sprache, Eingabe / Ereignisse – Expert aktivieren, auf Ereignisse klicken – 
      Herunterladen – GLB-XCI-xcremote-XCNAV.xci). Verlassen Sie die XCSoar-Konfiguration und 
      starten Sie XCSoar neu.]

== #hr[CAN-Terminierung]

#tr[Das LARUS Vario Display und die LARUS Sensoreinheit sind über den CAN Bus miteinander verbunden.
CAN-Bus-Netzwerke erfordern Abschlusswiderstände an jedem Ende des Netzwerks. Daher verfügen alle
Geräte über einen integrierten Schalter zur Aktivierung des Widerstands:]

#figure(
    image("/img/can-termination.jpg", width: 16cm),
    caption: [#hr[Schalter an den Geräten zur Terminierung des CAN Bus]],
)<can-termination>

#h(5mm)
#align(left, block[
    #figure(
        table(
            columns: (auto, auto, auto, auto, auto),
            align: (left, center, center, center, center),
            table.hline(),
            table.header(
                [#hr[*Beschreibung*]], [#hr[*Display vorne*]], [#hr[*Sensoreinheit*]], [#hr[*CAN Splitter*]], [#hr[*Display hinten*]],
            ),
            table.hline(),
            [#hr[Einsitzer]],                                           [on],  [on],  [-],   [-],
            [#hr[Doppelsitzer, Larus Sensoreinheit vorne]],             [off], [on],  [off], [on],
            [#hr[Doppelsitzer, Larus Sensoreinheit hinten]],            [on],  [on],  [off], [off],
            table.hline(),
        ),
        caption: [#hr[CAN Terminierung Ein-/Doppelsitzer]],
    )
])

#tr[Bitte beachten Sie: Alle LARUS-Sensoreinheiten, die vor März 2025 ausgeliefert wurden, verfügen über
keinen CAN-Abschlussschalter. Die CAN-Abschlusswiderstände sind standardmäßig immer aktiviert.]

== #hr[Externe Sicherung]

#picnote("/img/pictograph-yellow-warning.svg")[
    #tr[In der Regel wird das LARUS Vario Display über ein
    Patchkabel zwischen den CAN-Anschlüssen von LARUS mit Strom versorgt. LARUS muss durch eine
    externe Sicherung (500 mA bis max. 3 A) geschützt werden, wie es bei allen elektrischen Geräten
    in der Luftfahrt üblich ist. Wenn LARUS seine Energie von einem anderen Hauptinstrument bezieht
    (z. B. SteFly NAV über D-Sub-Stecker), stellen Sie bitte sicher, dass das Hauptinstrument durch
    eine externe Sicherung entsprechend geschützt ist.]
]

== #hr[Installationsort]

#tr[Das folgende Bild zeigt eine typische Einbausituation des LARUS Vario Displays im
Instrumentenbrett eines Segelflugzeugs.]

#figure(
    image("/img/panel.jpg", width: 10cm),
    caption: [#hr[Einbausituation im Instrumentenpanel]],
)<panel>

#tr[Das Display passt in eine Standardaussparung von 57 mm und wird mit 3 M3-Schrauben befestigt.]

#figure(
    image("/img/knobs-and-screws.jpg", width: 8cm),
    caption: [#hr[Befestigungselemente]],
)<knobs-and-screws>

#tr[Für die Installation ist es erforderlich, die beiden Drehknöpfe mit einem
1,5-mm-Sechskantschlüssel zu entfernen.]

== #hr[Installationsorientierung]

#picnote("/img/pictograph-blue-cloud.svg")[
    #tr[Wenn Sie beabsichtigen, ein vorhandenes Gerät durch das LARUS
    Vario Display zu ersetzen, überprüfen Sie bitte die gewünschte Einbaulage, bevor Sie das
    7,3-mm-Loch für den Drehgeber bohren, da das Gehäuse des Displays leicht asymmetrisch ist.]
]

#tr[Das Display kann in den Ausrichtungen 0° / 90° / 180° / 270° montiert werden. Nach der Montage
des Displays muss dessen Ausrichtung im Menü „Display Rotation“ ggfs. angepasst werden.]

== #hr[Erstinbetriebnahme und Funktionstest]

#tr[Für die Inbetriebnahme befolgen Sie bitte die folgenden Schritte:

+ Bitte überprüfen Sie, ob das LARUS Vario Display gemäß den Zeichnungen in  
  @system-configurations angeschlossen ist. 
+ LARUS einschalten.
+ Bitte überprüfen Sie, ob LARUS Vario Displays startet und ein gelbes oder grünes 
  Satellitenpiktogramm angezeigt wird. Die Vario-Zeiger sollten leichte Bewegungen um die
  Nullposition ausführen.
+ Jetzt sollten sie das Gerät konfigurieren. Alle Einstellmöglichkeiten sind detailliert in Kapitel 
  settings dokumentiert.]

== #hr[Wartung] 

#tr[Das gesamte System enthält keine zu wartenden Teile. Um Garantieleistungen in Anspruch zu 
nehmen, wenden Sie sich bitte direkt an SteFly.]

#picnote("/img/pictograph-yellow-warning.svg")[
    #tr[Das Öffnen des Gehäuses des LARUS Vario Displays führt zum Erlöschen der Garantie.]
]

== #hr[Firmware-Aktualisierung]

#tr[Das LARUS-Team verbessert die Software kontinuierlich und veröffentlicht Firmware Updates. Um
die Firmware zu aktualisieren, gehen Sie wie folgt vor:

+ Schalten Sie das Gerät aus.
+ Speichern Sie die neue \*.bin-Datei auf der im Lieferumfang enthaltenen SD-Karte und stecken Sie 
  diese in den SD-Kartensteckplatz auf der Rückseite des LARUS Vario Display.
+ Schalten Sie das Gerät ein.
+ Wenn eine Firmware auf der SD-Karte erkannt wird, bleibt das Display etwa 3-5 Sekunden lang 
  schwarz, bevor die Meldung "#keep[Installing... Do NOT power off device]" angezeigt wird.
+ Das Gerät startet automatisch neu. Während der ersten 10 Sekunden ist im Info1 Bereich
  die Firmwareversion zu sehen.

Das LARUS Vario Display installiert nur kompatible Firmware Versionen. Wenn mehrere Firmware 
Versionen auf der Karte gespeichert sind, wird die Neueste installiert.

Sollte die Installation fehlschlagen, dann widerholen Sie bitte den Vorgang. Sollte die
Installation wieder misslingen, verwenden Sie bitte eine andere SD-Karte. Die SD-Karte muss
mindestens 4 GByte groß (Typ SDHC) und mit FAT32 formatiert sein. Das Format muss kompatibel zu
DOS/Windows 95 (nicht GPT) sein.]
