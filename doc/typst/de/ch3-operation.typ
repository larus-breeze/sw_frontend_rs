#import "../manual.typ": *

= #hr[Betrieb]
== #hr[Bedienung]

#figure(
    image("/img/vario-display.jpg", height: 6cm),
    caption: [#hr[LARUS Vario Display]],
)<vario-display>

#tr[Das Gerät verfügt über einen Drehknopf mit zwei Ebenen und einem Drucktaster. Dem Knopf und den
beiden Drehknöpfen sind folgende Funktionen zugewiesen:

- Drehen Sie den kleinen/oberen Drehknopf: Lautstärkeregelung
- Drehen Sie den großen/unteren Drehknopf: MacCready Wert
- Kurzer Druck auf den Drehknopf: Flight Menu
- Längeres Drücken des Drehknopfs: Settings Menu

In den Menüs kann der gewünschte Punkt mit Hilfe der Drehgeber ausgewählt werden. Die Aktivierung erfolgt durch 
einen kurzen Tastendruck. In den Menüs gibt es entweder Submenüs oder Konfigurationsparameter zur Auswahl. 
Submenüs werden auf die gleiche Weise durchsucht. Nach der Aktivierung eines Konfigurationsparameters 
kann dieser geändert werden. Die Änderungen erfolgen unmittelbar. Durch einen kurzen Tastendruck kehrt man in 
das aufrufende Menü zurück.

Egal, ob Sie sich in einem Menü, einem Submenü oder im Editor befinden: Sie können immer mittels langem 
Tastendruck direkt in die Basiszeige zurückkehren.

Das Gerät stellt drei Basisanzeigen zur Verfügung: Vario, Horizont und Geräteinformationen. 
Zwischen den Anzeigen kann gewechselt werden, indem nach einem kurzen Tastendruck, im Flight 
Menu der Punkt Display genutzt wird. Alternativ dazu kann bei gedrückten Drehknopf die gewünschte 
Basisanzeige durch Drehen ausgewählt werden. Die eingestellte Anzeige beibt dauerhaft erhalten. 
In den folgenden Abschnitten werden die verschiedenen Anzeigemodi beschrieben. ]

== Vario Display
<vario-display>

#tr[Die zentrale Anzeige unterscheidet zwischen dem Kreisflugmodus und dem Geradeausflugmodus. Die 
Umschaltung zwischen diesen beiden Modi erfolgt automatisch. Wenn eine Drehgeschwindigkeit von 
mindestens 1°/Sek. für mehr als 10 Sekunden gemessen wird, wird der Kreisflugmodus aktiviert. Wenn 
die Drehgeschwindigkeit für mehr als 10 Sekunden unter diesem Wert liegt, wird wieder in den Geradeausflugmodus zurückgeschaltet.]

#picnote("/img/pictograph-blue-cloud.svg")[
  #tr[Der Wechsel vom Variometer Modus zum Sollfahrt Modus ist unabhängig vom Wechsel der zentralen Anzeigeinformationen.]
]

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
- Symbol Bereich: Kreis: Usage Mode Club , User Profile 1 (Usage Mode Normal als Quadrat). ]

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
- Info 3 Anzeige, hier Sollfahrt ]

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
  
== #hr[Geräteinformationen]<device-info>

#tr[In dem Anzeigemodus Geräteinformationen werden Dateilinformationen zum LARUS Vario Display und
zur LARUS Sensoreinheit angezeigt. Diese Anzeige kann hilfreich sein, um Fehleranalysen
durchzuführen oder spezielle Informationen abzurufen. Beispielsweise können hier sämtliche Zustände
der Ein- und Ausgänge eingesehen werden. Auch sind Fehlerzustände der LARUS Sensoreinheit
erkennbar.]

#figure(
    image("/img/device-info.png", width: 5cm),
    caption: [#hr[Geräteinformationen]],
)

== Flight Menu
<flight_menu>

#tr[Das #keep[Flight Menu] stellt Einstellmöglichkeiten bereit, die vor dem Flug oder während des
Fluges benötigt werden. Das Menu erreicht man durch kurzes Drücken des Bedienknopfes. Die Überschriften indiesem Abschnitt sind so wie im Gerät benannt.]

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

