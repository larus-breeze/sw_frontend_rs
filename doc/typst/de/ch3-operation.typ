#import "../manual.typ": *

= #hr[Betrieb]
== #hr[Bedienung]

#figure(
    image("/img/vario-display.jpg", height: 6cm),
    caption: [#hr[LARUS Vario Display]],
)<vario-display>

#tr[Das Gerät verfügt über einen zweistufigen Drehknopf mit zwei unabhängig voneinander bedienbaren 
Drehgebern und einem integrierten Drucktaster. Den Bedienelementen sind folgende Funktionen 
zugewiesen:

- Kleiner, oberer Drehknopf: Lautstärkeregelung
- Großer, unterer Drehknopf: Einstellung des MacCready-Werts
- Kurzer Tastendruck: Aufruf des Flight-Menüs
- Langer Tastendruck: Aufruf des Settings-Menüs

Die Navigation innerhalb der Menüs erfolgt über die Drehgeber. Durch Drehen wird der gewünschte Menüpunkt 
ausgewählt und durch einen kurzen Tastendruck aktiviert. Je nach Menüpunkt öffnet sich entweder ein 
Untermenü oder ein Konfigurationsparameter. Die Navigation innerhalb der Untermenüs erfolgt nach demselben 
Prinzip.

Nach Auswahl eines Konfigurationsparameters kann dessen Wert durch Drehen verändert werden. Änderungen 
werden unmittelbar wirksam. Ein kurzer Tastendruck beendet die Bearbeitung und führt zum übergeordneten 
Menü zurück.

Unabhängig davon, ob sich das Gerät in einem Menü, einem Untermenü oder bei der Bearbeitung eines 
Konfigurationsparameters befindet, kann jederzeit durch einen langen Tastendruck direkt zur 
Basisanzeige zurückgekehrt werden.

Das Gerät stellt drei Basisanzeigen zur Verfügung: Vario, Horizont und Geräteinformationen. Der Wechsel 
zwischen diesen Anzeigen erfolgt über den Menüpunkt „Display“ im Flight-Menü, das durch einen kurzen 
Tastendruck aufgerufen wird. Alternativ kann die gewünschte Basisanzeige direkt ausgewählt werden, indem 
der Drehknopf gedrückt gehalten und gleichzeitig gedreht wird. Die gewählte Anzeige bleibt dauerhaft 
gespeichert. 

In den folgenden Abschnitten werden die verschiedenen Anzeigemodi näher beschrieben.
]

== Vario Display
<vario-display>

#tr[Die Vario-Anzeige stellt das Steigen und Sinken auf einer analogen Skala dar. Das Gerät passt sich 
automatisch an die aktuelle Flugsituation an und stellt die jeweils relevanten Informationen zur 
Verfügung.

Dabei wird zwischen zwei Betriebsarten unterschieden: dem Kreisflugmodus und dem Geradeausflugmodus. 
Die Umschaltung erfolgt automatisch anhand der gemessenen Drehgeschwindigkeit. Überschreitet die 
Drehgeschwindigkeit für mehr als zehn Sekunden den Wert von 1°/s, wird der Kreisflugmodus aktiviert. 
Liegt sie für mehr als zehn Sekunden unter diesem Wert, schaltet das Gerät wieder in den 
Geradeausflugmodus zurück.

Abhängig vom aktiven Modus ändern sich die Informationen in der Mitte und am rechten Rand des Displays. 
Welche Werte in den beiden Modi angezeigt werden, lässt sich über die Konfiguration festlegen.

Unabhängig von der Umschaltung zwischen Geradeaus- und Kreisflugmodus unterscheidet das Gerät zwischen 
dem Vario- und dem Sollfahrtmodus. Beide Funktionen arbeiten unabhängig voneinander. Im Variomodus zeigt 
das Gerät das aktuelle Steigen oder Sinken an. Im Sollfahrtmodus dient die Anzeige dagegen als 
Kommandogerät, das die optimale Vorfluggeschwindigkeit vorgibt. Die Umschaltung zwischen Vario- und 
Sollfahrtmodus erfolgt abhängig von der Konfiguration entweder automatisch anhand der Fluggeschwindigkeit, 
über einen angeschlossenen Schalter oder durch Steuerbefehle über die CAN- bzw. NMEA-Schnittstelle.

Ergänzend zur visuellen Anzeige erfolgt eine akustische Signalisierung über einen angeschlossenen 
Lautsprecher. Die Art der Signalisierung hängt vom aktiven Vario- bzw. Sollfahrtmodus ab und wird in 
den folgenden Abschnitten näher erläutert.]

=== #hr[Kreisflugmodus]

#figure(
    image("/img/circling-explained.svg", width: 15cm),
    caption: [#hr[Anzeige im Kreisflugmodus]],
)<circling-explained>

#tr[Die Anzeigen Info 1, Info 2, Info3 und die zentrale Anzeige sind davon abhängig, ob geradeaus
oder im Kreis geflogen wird. Der Inhalt dieser Anzeigen ist für beide Modi einstellbar.

Inhalt im Kreisflugmodus:

- Aktuelles Steigen
- Mittleres Steigen
- MacCready Wert
- Zentrale Anzeige, hier Thermikassistent
- Info 1 Anzeige, hier Uhrzeit
- Info 2 Anzeige, hier Windrichtung und Windstärke
- Info 3 Anzeige, hier durchschnittliche Steigen seit Beginn des Kreisens
- Symbol Bereich: Die Farbe des Sat Symbols steht für die Datenqualität:
  - Grün: Verbindung zur LARUS-Sensoreinheit hergestellt. Gerät hat GPS-Fix.
  -	Gelb: Verbindung zur LARUS-Sensoreinheit hergestellt, Einheit hat keinen GPS-Fix
  - Rot: Keine Verbindung zur LARUS-Sensoreinheit
- Symbol Bereich: Die Farbe des Batteriesymbols entspricht der Betriebsspannung:
  - Grün: Die Spannung der Batterie ist ausreichend.
  - Gelb: Die Spannung der Batterie ist im kritischen Bereich.
  - Rot: Die Spannung der Batterie liegt unter dem Mindestwert.
- Symbol Bereich: Kreis: Usage Mode Club , User Profile 1 (Usage Mode Normal als Quadrat).

Das dargestellte Szenario zeigt den Variomodus im Kreisflug. Erkennbar ist dies an der Anzeige „Info 3“, 
die das durchschnittliche Steigen darstellt.

Parallel zur visuellen Anzeige erfolgt die akustische Ausgabe des Variotons. In der Standardkonfiguration 
wird Sinken durch einen Dauerton signalisiert, dessen Frequenz mit zunehmendem Sinken abnimmt. Steigen 
wird dagegen durch einen unterbrochenen Ton angezeigt, dessen Frequenz mit zunehmendem Steigen ansteigt.

Die Mittenfrequenz, die Frequenzspreizung sowie die Signalform lassen sich über die Konfiguration 
individuell anpassen. Darüber hinaus kann ein Ruhebereich definiert werden, innerhalb dessen keine 
akustische Signalisierung erfolgt.]

=== #hr[Geradeausflugmodus]

#figure(
    image("/img/straight-explained.svg", width: 15cm),
    caption: [#hr[Anzeige im Geradeausflugmodus]],
)<straight-explained>

#tr[Inhalt im Geradeausflugmodus:

- Zentrale Anzeige, hier Windrichtung in bezug auf Flugzeuglängsachse
- Mittleres Steigen
- MacCready Wert
- Sollfahrtgeber: Die Position des Bandes zeigt an, ob Sie zu schnell oder zu langsam fliegen. 
  Positive Werte bedeuten, dass Sie zu schnell fliegen, negative Werte bedeuten, dass Sie zu 
  langsam fliegen. Die Länge des Bandes gibt an, um wie viel. 1 m/s entspricht 10 km/h.
- Symbol Bereich: Batterie Symbol ok, Sat Symbol ok, Usage Mode Club, User Profil 1
- Info 1 Anzeige, hier Winkel Windversatz
- Info 2 Anzeige, hier Windrichtung und Windstärke
- Info 3 Anzeige, hier Sollfahrt 

Das dargestellte Szenario zeigt den Geradeausflug im Sollfahrtmodus. Erkennbar ist dies an der Anzeige 
„Info 3“, die die Sollfahrt darstellt, sowie am SpeedToFly-Differenzbalken innerhalb der analogen 
Anzeige.

In der Standardkonfiguration wird eine zu geringe Fluggeschwindigkeit akustisch durch einen tiefen Dauerton 
signalisiert. Fliegt der Pilot dagegen zu schnell, ertönt ein hoher, unterbrochener Ton.

Innerhalb eines Toleranzbereichs von ±10 km/h gegenüber der Sollfahrt erfolgt keine akustische Signalisierung.
Neben den bereits beschriebenen akustischen Parametern lässt sich auch dieser Ruhebereich über die 
Konfiguration individuell anpassen.]

=== #hr[Vario Mode - Vario, Netto, Netto Plus]

#tr[In der Standardkonfiguration zeigt das Display das von der LARUS-Sensoreinheit ermittelte Steigen oder 
Sinken des Segelflugzeugs an. Dieser Wert setzt sich aus der meteorologisch bedingten Vertikalbewegung der 
umgebenden Luftmasse und dem flugzeugbedingten Eigensinken zusammen.

Insbesondere im schnellen Geradeausflug führt das zunehmende Eigensinken dazu, dass die angezeigte 
Sinkgeschwindigkeit sehr groß wird. Dies erschwert dem Piloten die Beurteilung der meteorologischen 
Verhältnisse.

Um die Anzeige an unterschiedliche Anforderungen anzupassen, stehen über den Konfigurationsparameter VarioMode 
drei Betriebsarten zur Verfügung: Vario, Netto und Netto Plus.

*Vario*

Im Modus Vario wird das tatsächliche Steigen oder Sinken des Segelflugzeugs angezeigt. Die Anzeige berücksichtigt 
damit sowohl die Vertikalbewegung der Luftmasse als auch das geschwindigkeitsabhängige Eigensinken des Flugzeugs.

*Netto*

Im Modus Netto wird das anhand der Flugzeugpolare berechnete Eigensinken kompensiert. Die Anzeige entspricht damit 
näherungsweise der vertikalen Bewegung der umgebenden Luftmasse und ist weitgehend unabhängig von der geflogenen 
Geschwindigkeit. Da das Eigensinken herausgerechnet wird, zeigt das Netto-Vario im Aufwind einen höheren Wert als 
das tatsächlich vom Segelflugzeug erreichte Steigen an. Diese Betriebsart eignet sich insbesondere zur Beurteilung 
der meteorologischen Vertikalbewegungen und erleichtert das Erkennen von Aufwind- und Abwindgebieten.

*Netto Plus*

Der Modus Netto Plus kombiniert die Eigenschaften der beiden zuvor beschriebenen Betriebsarten. Bei 
Fluggeschwindigkeiten unterhalb der Geschwindigkeit des besten Gleitens entspricht die Anzeige dem herkömmlichen 
Variomodus. Bei höheren Geschwindigkeiten wird das geschwindigkeitsabhängige Eigensinken anhand der Flugzeugpolare 
kompensiert. Zusätzlich wird ein konstanter Korrekturwert berücksichtigt, sodass die Anzeige weitgehend unabhängig 
von der Fluggeschwindigkeit bleibt.

Damit erhält der Pilot eine Abschätzung des Steigens, das er bei einer Verringerung der Fluggeschwindigkeit auf die 
für den Kreisflug typische Geschwindigkeit in derselben Luftmasse erreichen könnte. Insbesondere im schnellen 
Geradeausflug erleichtert diese Darstellung die Einschätzung, ob ein durchflogenes Aufwindgebiet für einen 
anschließenden Kreisflug geeignet ist.]


#tr[Die folgenden Beispiele veranschaulichen das Anzeigeverhalten der verschiedenen Vario-
Betriebsarten anhand ausgewählter Flugsituationen im Geradeausflug:

#align(left, block[
    #figure(
        table(
            columns: (auto, auto, auto, auto, auto),
            align: (left, auto, center, center, center),
            table.hline(),
            table.header(
                [*Modus*],  [*Steigen*],    [*bei 80 km/h*],    [*bei 105 km/h*],   [*bei 180 km/h*],
            ),
            table.hline(),
            [Vario],        [0 m/s],        [-0,6 m/s],         [-0,7 m/s],         [-1,9 m/s],
            [Netto],        [0 m/s],        [0,0 m/s],          [0,0 m/s],          [0,0 m/s],
            [Netto Plus],   [0 m/s],        [-0,6 m/s],         [-0,7 m/s],         [-0,7 m/s],
            table.hline(),
            [Vario],        [3 m/s],        [2,4 m/s],          [2,3 m/s],          [1,1 m/s],
            [Netto],        [3 m/s],        [3,0 m/s],          [3,0 m/s],          [3,0 m/s],
            [Netto Plus],   [3 m/s],        [2,4 m/s],          [2,3 m/s],          [2,3 m/s],
            table.hline(),
            [Vario],        [-2 m/s],       [-2,6 m/s],         [-2,7 m/s],         [-3,9 m/s],
            [Netto],        [-2 m/s],       [-2,0 m/s],         [-2,0 m/s],         [2,0 m/s],
            [Netto Plus],   [-2 m/s],       [-2,6 m/s],         [-2,7 m/s],         [-2,7 m/s],
            table.hline(),
        ),
        caption: [#hr[Vario Anzeigen, verschiedene Steigwerte, Polare LS-3 WL]],
    )
])
]

#tr[Die Genauigkeit der Anzeige in den Modi Netto und Netto Plus hängt wesentlich von der 
hinterlegten Flugzeugpolare ab. Die verwendete quadratische Näherung der tatsächlichen 
Flugzeugpolare kann je nach Flugzeugtyp erhebliche Abweichungen aufweisen. Dies gilt 
insbesondere für den Geschwindigkeitsbereich unterhalb der Geschwindigkeit des besten Gleitens.
Darüber hinaus weisen die Polaren moderner Segelflugzeuge bei hohen Geschwindigkeiten häufig 
einen ausgeprägten Knick auf, der durch eine einfache quadratische Funktion nicht ausreichend 
abgebildet werden kann.

Diese Abweichungen führen zu Fehlern bei der Berechnung des polaren Eigensinkens und beeinflussen 
damit unmittelbar die Genauigkeit der Netto- und Netto-Plus-Anzeige. Besonders im reinen Netto-Modus 
können daraus erhebliche Anzeigefehler resultieren.]

=== #hr[Warnungen]

#figure(
    image("/img/warning.svg", height: 38mm),
    caption: [#hr[Anzeige einer Warnung und Ermittlung der Ursache]],
)<straight-explained>

#tr[Das LARUS Vario Display warnt den Nutzer, wenn es Probleme bei der Datenauswertung hat. Dann ist ein rotes Warndreieck mit einem Ausrufezeichen zu sehen. Das Vario arbeitet zwar nach wie vor korrekt, aber die Anzeigequalität könnte schlechter sein.

Mögliche Ursachen:

- Der GNSS-Empfang könnte (zeitweise) eingeschränkt sein.
- Es könnte sein, dass der Magnetsensor gestört ist.

Die Ursache der Störung kann auf der Seite „Device Info" (@device-info) ermittelt werden. Im gezeigten Beispiel liegt ein eingeschränkter GNSS-Empfang vor. Wenn diese Warnung im LARUS Vario Display oft oder dauerhaft angezeigt wird, liegt ein Installationsproblem vor (siehe @trouble-shooting).
]

=== #hr[Verfügbare zentrale Anzeigen]
==== #hr[Zentrale Anzeigen im Kreisflugmodus]

#figure(
    image("/img/circling-single-arrow.png", width: 5cm),
    caption: [#hr[Windanzeige mit einem Pfeil und Fahne]],
)<circling-single-arrow>

#tr[Die aktuelle Windrichtung wird durch einen zentralen Pfeil dargestellt. Die Größe des Pfeils ist
proportional zur Windgeschwindigkeit. Änderungen der Windrichtung gegenüber der mittleren Richtung
werden durch eine Windfahne angezeigt, Änderungen der Windgeschwindigkeit gegenüber der
mittelfristigen Durchschnittsgeschwindigkeit werden durch die Breite der Windfahne dargestellt. Die
Pfeilrichtung bezieht sich auf Norden, symbolisiert durch das N in der Skala oben.]

#figure(
    image("/img/circling-double-arrow.png", width: 5cm),
    caption: [#hr[Windanzeige mit zwei Pfeilen]],
)<circling-double-arrow>

#tr[Die aktuelle Windrichtung und -geschwindigkeit wird durch den blauen Pfeil dargestellt,
Informationen zum mittleren Wind werden durch den grauen Pfeil im Hintergrund angezeigt. Die
Pfeilgröße hängt von der Windgeschwindigkeit ab. Die Pfeilrichtung bezieht sich auf Norden.]

#figure(
    image("/img/circling-dotted-assistant.png", width: 5cm),
    caption: [#hr[Zentrierhilfe mit Punkten]],
)<circling-dotted-assistant>

#tr[Der Termik Assistent kann den Piloten beim Zentrieren der Thermik unterstützen. Er zeigt
anschaulich, wo gutes und weniger gutes Steigen zu finden ist. Diese Information ist besonders
nützlich, wei das LARUS System das Steigen verzögerungsfrei anzeigt. 

Bedeutung der Farben der Kreispunkte:

- Gelb: Maximum des Steigens
- Schwarz: Minimum des Steigens
- Rot: Das Steigen liegt über dem Durchschnitt
- Blau: Das Steigen liegt unter dem Durchschnitt 

Der Durchmesser der Punkte ist proportional zur Steigrate. Ein konstanter Aufwind ist dann 
optimal zentriert, wenn blaue und rote Punkte etwa gleich häufig vorkommen.]

#figure(
    image("/img/circling-spider-assistant.png", width: 5cm),
    caption: [#hr[Zentrierhilfe mit Spinnennetz]],
)<circling-spider-assistant>

#tr[Bedeutung der Farben der Kreissegmente:

- Gelb: Maximum des Steigens
- Schwarz: Minimum des Steigens
- Rot: Das Steigen liegt über dem Durchschnitt
- Blau: Das Steigen liegt unter dem Durchschnitt

Der Durchmesser des Segments ist proportional zur Steiggeschwindigkeit. Ein konstanter Aufwind
ist dann optimal zentriert, wenn blaue und rote Flächen gleich häufig erscheinen.]

==== #hr[Zentrale Anzeigen im Geradeausflugmodus]

#figure(
    image("/img/straight-single-arrow.png", width: 5cm),
    caption: [#hr[Windanzeige im Geradeausflug mit einem Pfeil und Fahne]],
)<straight-single-arrow>

#tr[Die aktuelle Windrichtung wird durch einen zentralen Pfeil dargestellt. Die Größe des Pfeils ist
proportional zur Windgeschwindigkeit. Änderungen der Windrichtung gegenüber der mittleren Richtung
werden durch eine Windfahne angezeigt, Änderungen der Windgeschwindigkeit gegenüber der
mittelfristigen Durchschnittsgeschwindigkeit werden durch die Breite der Windfahne dargestellt. Das
Flugzeugsymbol deuted an, dass sich die Darstellung auf die Flugrichtung bezieht.]

#figure(
    image("/img/straight-double-arrow.png", width: 5cm),
    caption: [#hr[Windanzeige im Geradeausflug mit zwei Pfeilen]],
)<straight-double-arrow>

#tr[Die aktuelle Windrichtung und -geschwindigkeit wird durch den blauen Pfeil dargestellt,
Informationen zum mittleren Wind werden durch den grauen Pfeil im Hintergrund angezeigt. Die
Größe der Pfeile hängt von der Windgeschwindigkeit ab. Die Pfeilrichtung bezieht sich auf die
Längsachse des Segelflugzeugs. Das Segelflugzeugsymbol deuted an, dass sich die Darstellung auf die
Flugrichtung bezieht.]

== #hr[Künstlicher Horizont]

#picnote("/img/pictograph-yellow-warning.svg")[
    #tr[Dies ist kein amtlich zugelasssener künstlicher Horizont. Dashalb darf diese Anzeige nicht 
    dazu benutzt werden in Wolken oder anderweitig außerhalb von VFR Bedingeungen zu fliegen.]
]

#figure(
    image("/img/horizon.png", width: 5cm),
    caption: [#hr[Künstlicher Horizont]],
)<horizon>

#tr[Die Anzeige des künstlichen Horizonts beinhaltet folgende Informationen:

- Die blaue Fläche stellt den Himmel dar.
- Die Grenze zur braunen Fläche entspricht dem Horizont.
- Die kreisförmige Skala oben zeigt in 15° Schritten die aktuelle
      Querneigung des Segelflugzeugs (hier etwa 30°, rote Pfeilspitze).
- Die Skala parallel zum Horizont steht für den Steig- / Sinkwinkel in 10° Schritten
      (hier 0°).
- Im unteren Bereich ist eine Libelle zu sehen, die ein eventuelles Schieben sichtbar
      macht.

Das Segelflugzeug befindet sich derzeit in einer sauberen Rechtskurve mit einer Querneigung von
30°.]

#figure(
    image("/img/horizon-blocked.png", width: 3cm),
    caption: [#hr[Künstlicher Horizont blockiert]],
)<horizon-blocked>

#picnote("/img/pictograph-yellow-warning.svg")[
  #tr[In manchen Wettbewerben ist die Anzeige des künstlichen
  Horizontes verboten. Deshalb kann in der LARUS Sensoreinheit die Ausgabe der
  Horizontinformationen blockiert werden. In diesem Fall wird statt des Horizontes eine Warnung
  ausgegeben.]
]
  
== #hr[Geräteinformationen]
<device-info>

#tr[In dem Anzeigemodus Geräteinformationen werden Dateilinformationen zum LARUS Vario Display und
zur LARUS Sensoreinheit angezeigt. Diese Anzeige kann hilfreich sein, um Fehleranalysen
durchzuführen oder spezielle Informationen abzurufen. Beispielsweise können hier sämtliche Zustände
der Ein- und Ausgänge eingesehen werden. Auch sind Fehlerzustände der LARUS Sensoreinheit
erkennbar.]

#figure(
    image("/img/device-info.png", width: 5cm),
    caption: [#hr[Geräteinformationen]],
)

== #hr[Alarmanzeige]

#tr[Beim Auftreten eines Alarms wird die aktuell gewählte Basisanzeige durch eine entsprechende 
Warnmeldung überlagert. Gleichzeitig erfolgt eine akustische Signalisierung. Die Alarmanzeige 
bleibt so lange aktiv, bis die auslösende Bedingung nicht mehr vorliegt.

Auf diese Weise lässt sich beispielsweise eine Fahrwerkswarnung realisieren. Hierzu müssen die 
angeschlossenen Geber und deren Signalzuordnung in der Konfiguration entsprechend festgelegt 
werden.]

#figure(
    image("/img/alarm.png", width: 5cm),
    caption: [#hr[Alarmanzeige]],
)

== Flight Menu
<flight_menu>

#tr[Das #keep[Flight Menu] stellt Einstellmöglichkeiten bereit, die vor dem Flug oder während des
Fluges benötigt werden. Das Menu erreicht man durch kurzes Drücken des Bedienknopfes. Die Überschriften 
in den folgenden Abschnitten entsprechen den englischen Menübezeichnungen des LARUS Vario Displays.
Die Basisanzeige bleibt während der Eingaben im #keep[Flight Menu] teilweise erhalten.]

#figure(
    image("/img/flight-menu.png", width: 5cm),
    caption: [#hr[Flight Menu]],
)

=== Water Ballast
#tr[Hier wird die Menge des getankten Wasserballastes vorgegeben. Beim Ablassen des Wassers während
des Fluges kann dieser Wert manuell korrigiert oder durch einen Schalter am Ventil und
entsprechender Konfiguration automatisch reduziert werden.

Der eingestellte Wert wird zu einem angeschlossenen Navigationsrechner synchronisiert.]

=== Bugs
#tr[Insekten auf Tragflächen und Rumpf reduzieren die Gleitleistungen des Segelflugzeuges. Diese
Veränderungen der Leistung eines Segelflugzeuges können näherungsweise mit dieser Einstellung
berücksichtigt werden. Es sind Vorgaben von 0 bis 50 Prozent möglich. Bei einer Einstellung von
50 Prozent verdoppelt sich die Sinkrate bei gegebener Geschwindigkeit.] 

Der Algorithmus arbeit exakt identisch zu XCSoar / OpenSoar. Der eingestellte Wert wird zu einem
angeschlossenen Navigationsrechner synchronisiert.

=== Pilot Weight
#tr[Das Gewicht des Piloten wird bei der Berechnung der Segelflugzeug Polare berücksichtigt. Bei Doppelsitzern
ist hier das Gewicht der Summe beider Piloten vorzugeben

Der eingestellte Wert wird zu einem angeschlossenen Navigationsrechner synchronisiert.]

=== Display
#tr[Der Menupunkt Display ermöglicht die Auswahl der ständigen Anzeige des Gerätes. Hier kann zwischen
Vario, Horizont und Geräteinformationen gewechselt werden.]

=== User Profile<user-profile>
#tr[Das LARUS Vario Display bietet viele Einstellmöglichkeiten, um die Anzeigen an die Bedürfnisse
des Piloten anzupassen. Wenn mehrere Piloten auf einem Segelflugzeug fliegen, ermöglichen die 
#keep[User Profiles] ein komfortables Umschalten zwischen den verschiedenen Einstellungen. Es können 
bis zu 4 unterschiedliche Profile genutzt werden.

Wie im Abschnitt Usage-Mode beschrieben, hängt es vom aktivierten Mode ab, ob 3 oder
4 Nutzungsprofile zur Verfügung stehen. Einige Einstellungen sind in allen vier Nutzungsprofilen
gleichgeschaltet, da sie vom Segelflugzeug und der Einbausituation abhängen. Dies betrifft z.B. die
Polare des Segelflugzeuges sowie Hardware Pin Konfigurationen. Damit wird sichergestellt, dass diese
Einstellungen in allen Profilen zur Verfügung stehen.]

