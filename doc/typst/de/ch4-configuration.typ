#import "../manual.typ": *

= Settings
<settings>

#tr[Die hier hinterlegten Funktionen werden zur Konfiguration des Systems genutzt. Die Überschriften
 entsprechen denen englischen Texten in den Menüs des LARUS Vario Display, um die Auffindbarkeit zu
 vereinfachen.]

== Views
<views>
=== Circling, Straight
#tr[Diese beiden Menupunkte legen fest, was im Geradeausflug und was im Kreisflug angezeigt wird.
Die Umschaltung zwischen Kreisflug und Geradeausflug erfolgt automatisch.

Folgende Ansichten können jeweils festgelegt werden:

- #keep[*Center Content:*] Mittenanzeige
- #keep[*Info 1 Content:*] Obere Zeile
- #keep[*Info 2 Content:*] Untere Zeile
- #keep[*Info 3 Content:*] Rechter Rand

In den Ansichten können verschiedene Informationen dargestellt werden. Nachfolgend
ist aufgelistet welche Informationen an welcher Stelle möglich sind:

#keep[*Center Content*]:
- #keep[*Single Arrow*] Einfacher Pfeil mit Windfahne
- #keep[*Double Arrow*] Zwei Pfeile (Wind und mittlerer Wind)
- #keep[*Dotted Assistant*] Thermik Assistent mit Punkten
- #keep[*Spider Assistant*] Thermik Assistent in Form eines Spinnennetzes

#keep[*Info 1 Content*]:
- #keep[*None*] Nichts
- #keep[*Avg Climb Rate*] Mittleres Steigen
- #keep[*Battery Voltage*] Batteriespannung
- #keep[*Circle Diameter*] Durchmesser beim Kreisen
- #keep[*Circle Max-Min*] Differenz maximales/minimales Steigen
- #keep[*Drift Angle*] Driftwinkel
- #keep[*Flight Level*] Flugfläche
- #keep[*G-Load*] Beschleunigung
- #keep[*Speed to Fly*] Sollfahrt
- #keep[*True Air Speed*] Wahre Geschwindigkeit gegenüber Luft
- #keep[*True Course*] Wahrer Kurs
- #keep[*UTC Time*] UTC Zeit

#keep[*Info 2 Content*]:
- #keep[*None*] Nichts
- #keep[*Avg Climb Rate*] Mittleres Steigen
- #keep[*Battery Voltage*] Batteriespannung
- #keep[*Circle Diameter*] Durchmesser beim Kreisen
- #keep[*Circle Max-Min*] Differenz maximales/minimales Steigen
- #keep[*Drift Angle*] Driftwinkel
- #keep[*Flight Level*] Flugfläche
- #keep[*G-Load*] Beschleunigung
- #keep[*Speed to Fly*] Sollfahrt
- #keep[*True Air Speed*] Wahre Geschwindigkeit gegenüber Luft
- #keep[*True Course*] Wahrer Kurs
- #keep[*UTC Time*] UTC Zeit
- #keep[*Wind, Avg Wind*] Wind, mittlerer Wind
- #keep[*Wind and Delta*] Wind und Differenz zum mittleren Wind

#keep[*Info 3 Content*]:
- #keep[*None*] Nichts
- #keep[*CLimbing*] Steigen, gemittelt über den kompletten Aufwind
- #keep[*Speed to Fly*] Sollfahrt
]

=== Units

#tr[Verschiedene Darstellungen der Anzeige erfolgen mit einer Maßeinheit. Hier wird festgelegt,
werche Maßeinheiten zur Anwendung kommen:

- #keep[*Horizontal Speed*] Maßeinheit der Horizontalgeschwindigkeit
- #keep[*Vertical Speed*] Maßeinheit der Vertikalgeschwindigkeit
- #keep[*Height*] Maßeinheit der Höhenanzeige
]

=== Energy Arrow

#tr[Der Energiepfeil zeigt an, in welcher Richtung vermutlich ein Steigen zu erwarten ist. Angezeigt wird die vektorielle Differenz zwischen dem aktuellen und dem mittleren gemessenen Wind.

Grundlage dieser Funktion ist die Annahme, dass sich in der Umgebung eines thermischen Aufwindes ein Strömungsfeld befindet. Dieses beeinflusst den horizontalen Wind in der Nähe des Aufwindes.]

#figure(
    image("/img/energy-arrow.svg", width: 16cm),
    caption: [#hr[Strömungsfeld eines thermischen Aufwindes nach Martin Dinges @dinges und die Auswirkung auf die Windanzeige nach Joe Wurts @wurts]],
)<img-energy-arrow>

#tr[Bitte beachten Sie, dass die Richtung von der Höhe, in der sich das Segelflugzeug befindet, abhängig ist. In niedriger Höhe zeigt die Anzeige in Richtung des Aufwindes, in großer Höhe hingegen in die entgegengesetzte Richtung. In niedriger Höhe fließt die Luft in Richtung Aufwind, in großer Höhe dagegen vom Aufwind weg.

Die Anzeige kann aber auch in anderen Situationen, wie z. B. beim Hangfliegen, von Bedeutung sein.]

#figure(
    image("/img/energy-arrow.png", width: 5cm),
    caption: [#hr[Anzeige des Energy Arrow im Flug]],
)

#tr[Das im gezeigten Beispiel dargestellte Ereignis wurde knapp über einem Grat aufgenommen, als das Segelflugzeug an einem Aufwind vorbeiflog, den es später nutzen konnte. Die Anzeige kann mit einem Faktor von 0,0 bis 10,0 angepasst werden. Dabei bedeutet 0,0 keine Darstellung des #keep[*Energy Arrow*] und 10,0 eine sehr große Darstellung. Starten Sie mit einer Einstellung von 3,5.

Nur moderne Variometer-Systeme, die Windstärke und -richtung instantan bestimmen, ermöglichen eine sinnvolle Darstellung des Energy Arrow. Mit konventionellen Varios ist dies nicht möglich.]

=== Display Rotation

#tr[Das Display kann unterschiedlich verdreht (0°, 90°, 180° oder 270°) eingebaut werden. Die
Anzeige kann hier auf die Einbausituatioin angepasst werden]

=== Glider Symbol

#tr[Im Geradeausflug bezieht sich die Windanzeige auf die Flugzeuglängsachse. Dies kann durch die
Darstellung des Grundrisses eines Segelflugzeuges symbolisiert werden. Die Symbolik kann aktiviert
oder deaktiviert werden.]

== Advanced
<advanced>
=== User Profiles
==== Usage Mode, Code
<usage-modes>

#tr[Die Einstellmöglichkeiten des LARUS Vario Display erlauben individuelle Konfigurationen nach den
Bedürfnissen des Piloten. Im Vereinsbetrieb kommt es deshalb nicht selten zu Irritationen,
wenn ein Pilot auf ein ungewohnt konfiguriertes Gerät trifft. Aus diesem Grund unterstützt das LARUS
Vario Display zwei Modi: Normal und Club.

Im Modus Normal (Auslieferzustand) können alle Einstellungen beliebig vorgenommen werden. Es stehen
in diesem Modus vier Nutzungsprofile (0..3) zur Verfügung. So können bis zu vier Piloten dauerhaft
unterschiedliche Einstellungen verwenden.

Im Modus Club werden zwei gegensätzliche Ziele verfolgt. Einerseits sollen den Piloten die
nützlichen Einstellungen ermöglicht werden, andereseits aber standardisierte Einstellungen
vorgehalten werden. Um dies zu erreichen, wird das Profil 0 gesperrt. Dieses Profil dient als
Kopiervorlage für die standardisierten Einstellungen. Profile 1, 2 und 3 können wie gewohnt genutzt
werden. Dabei sind einige Konfigurationspunkte wie z.B. Einstellungen der Polare, Zuordnung der
Hardware Pins oder Zugriff auf die Sensoreinheit ausgenommen. Profil 1 wird an jedem neuen Flugtag
auf die Standardwerte zurückgesetzt und aktiviert. Im Menupunkt "#keep[User Profile]" (@user-profile) 
wird zusätzlich eine Funktion vorgehalten, um das gewählte Profil bei Bedarf auf die Standardwerte zurückzusetzen.

Die Umschaltung zwischen den "#keep[Usage Modes]" ist durch einen Code gesichert.
Der Code leitet sich aus der Version der Firmware ab. Die Firmware v0.3.8.56 erwartet beispielsweise
den Code 3856.]

==== Config Reset
#tr[Das aktuell ausgewählte User Profil kann mit Hilfe dieser Funktion auf die Defaultwerte zurück gesetzt werden. Die Defaultwerte sind im Gerät "hart codiert" und können nicht verändert werden. Es werden alle Einstellungen zurückgesetzt, welche die angezeigten Daten betreffen. Einstellungen zum Flugzeug oder zur Hardware bleiben erhalten.

Diese Funktion ist auch im "#keep[Usage Mode]" Normal verfügbar und darf nicht mit dem Rücksetzen
auf Standardwerte aus Profil 0 im "#keep[Usage Mode]" Club verwechselt werden.]

==== Factory Reset
#tr[Hiermit wird das Gerät auf den Auslieferungszustand zurück gesetzt. Dies betrifft wirklich 
alle Einstellungen für alle Profile.]

=== Vario
==== Avg Climb Source, TC Climb Source

#tr[Es werden zwei verschiedene Quellen zur Ermittlung des Mittleren Steigens unterstützt. Die Unterschiede
stellen sich wie folgt dar *Avg Climb Source:* 

- *Frontend:* Die Mittelwertbildung erfolgt während des Kreisens. Beim Übergang
  von Sollfahrt nach Vario wird der aktuelle Variowert als Startwert genutzt. Beim Übergang
  von Vario nach Sollfahrt wird die Mittelung angehalten und die Anzeige bleibt konstant.
  Die Zeitkonstante der Mittelwertbildung kann mit #keep[*TC Climb Source*] angepasst werden.
- *Sensorbox:* Die Mittelwertbildung erfolgt laufend. Während des Geradeausfluges
  wird mit fester Zeitkonstante gemittelt, welche im Sensorbox Menu eingestellt werden kann.
  Während des Kreisens erfolgt die Mittelung synchron zum Kreisen.
]

==== Vario Upper Limit, Vario Lower Limit

#tr[Die akustische Signalisierung  wird zwischen diesen beiden Werten stumm geschaltet.]

=== Speed to Fly
==== TC Circle Hyst

#tr[Die Hysteres, also die Wartezeit bei der Umschaltung zwischen Vario und Sollfahrt wird hier
gesetzt.]

==== TC Speet to Fly

#tr[Die Darstellung der Sollfahrt wird gedämpft, um den Piloten nicht mit einer nervösen Anzeige zu
irritieren. Hier kann vorgegeben werden, mit welcher Zeitkonstante diese Dämpfung erfolgen soll.]

==== Vario Control, StF Pin Config

#tr[Das LARUS Vario Display unterstützt verschiedene Methoden, um zwischen der Vario- und der
Sollfahrt-Anzeige hin- und herzuschalten. Folgende Möglichkeiten stehen zur Verfügung:

- *Auto:* Die Umschaltung erfolgt abhängig von der Fluggeschwindigkeit. Die Grenze
          liegt bie dem 1.1 fachen der Geschwindigkeit für das beste Gleiten. Bei der Festlegung der
          Grenze wird die Flugzeugpolare, sowie die Beladung (Pilotengewicht, Wasser Ballast)
          berücksichtigt. Während des Kreisens wird nicht auf Vario zurückgeschalte.
- *Input Pin:* Die Umschaltung wird durch einen Schalter bzw. Taster (wählbar)
          ausgelöst. Die Hardware Konfiguration muss zusätzlich konfiguriert werden (Schalter/Taster
          und Polarität) *StF Pin Config*.
- *NMEA:* Die Umschaltung wird durch XCSoar/OpenSoar ausgelöst. Diese Einstellung
          kann auch verwendet werden, wenn eine Knüppelfernbedienung für XCSoar/OpenSoar mit
          Sollfahrt Taste zum Einsatz kommt.
- *CAN:* In Doppelsitzer Installationen kann es gewünscht sein, dass die
          Umschaltung Sollfahrt/Vario durch das zweite Anzege Gerät ausgelöst wird. Wenn diese
          zweite Gerät die Umschaltung beispielsweise automatisch vornimmt, wird hiermit sicher
          gestellt, dass beide Anzeigen synchronisiert arbeiten.
]
==== Stf Upper Limit, Stf Lower Limit

#tr[Sie können einen Geschwindigkeitsbereich festlegen, in dem das akustische Sollfahrt Signal
stummgeschaltet wird. Im Lieferzustand wird das Audiosignal in einem Bereich von +/- 10 km/h
deaktiviert. Dieser Bereich kann hier an die individuellen Wünsche angepasst werden.]

=== Gear Alarm

#tr[Die Fahrwerkswarnung soll den Piloten an das Ausfahren des Fahrwerkes erinnern, falls er es vor
der Landung vergisst. Die Fahrwerkswarnung bassiert auf zwei Schaltern, die Bremsklappen und
Fahrwerk überwachen. Die Warnung erfolgt sowohl optisch auf dem Display als auch akustisch. Die
Schalter können beide direkt an das LARAUS Vario Display als auch in Reihenschaltung durch eine
Signalleitung angeschlossen werden.

Direkter Anschluss von beiden Schaltern an das LARUS Vario Display: Es müssen beide Pins korrekt
eingerichtet werden: #keep[*Gear Pin Config*, *Airbrakes Pin Config*. *Gear Alarm
Config*] muss dann auf #keep[*Two Pin Mode*] eingestellt werden.

Anschlüsse von den Schaltern in Serienschaltung: Die gemeinsame Leitung wird mit 
#keep[*GearPinConfig*] eingerichtet. #keep[*Gear Alarm Config*] muss dann auf #keep[*One Pin Mode*] eingestellt werden.

#keep[*Alarm Volume*] ermöglicht die Einstellung der Lautstärke eines Alarms.
]

=== Drain Control

#tr[Der Schalter, der die Wasserablassvorrichtung überwacht wird mit #keep[*Drain Pin Config*]
eingerichtet.

Es wird von einem konstanten Durchfluss ausgegangen, der hier vorgegeben werden muss:] *Flow*. 

=== Flash Control
#tr[Die Funktionen zum Haubenblitzer sind wie folgt organisiert:

- *Flash Control:* Das LARUS Vario Display ist in der Lage, einen Haubenblitzer anzusteuern, der
              bei einer Geschwindigkeit von über 40 km/h gegenüber Luft aktiviert wird. Hier muss 
              festgelegt werden, ob der Blitzer aktiv wird, wenn der Schalter offen oder 
              geschlossen ist.
- *Flash Test:* Der Haubenblitzer wird für 10 Sekunden aktiviert. Somit kann überprüft werden,
              ob dieser korrekt funktioniert.
]

=== Sound
#tr[Die Audioausgabe kann durch verschiedene Einstellungen an den persönlichen Geschmack angepasst werden.

- *Center Frequency:* Gibt die Tonfrequenz bei 0 m/s Steigen an.
- *Waveform:* Auswahl der Wellenform der Audioausgabe. Dies beeinflusst die Tonfarbe mit mehr oder weniger Oberwellen.
- *Spreading Factor:* Gibt an, um wie viel sich die Frequenz ändert, wenn das Segelflugzeug steigt oder sinkt. Ein Wert von 1,0 bedeutet, dass der Bereich von -5 m/s bis +5 m/s auf zwei Oktaven aufgeteilt wird.]

=== More Settings

#tr[In diesem Abschnitt sind folgende Einstellungen zusammengefasst:

- *Battery Good:* Oberhalb des hier eingestellten Grenzwertes ist die
              Spannungsversorgung in Ordnung (grünes Batteriesymbol).
- *Battery Low:* Unterhalb der hier vorgegebenen Spannung wird das
              Batteriesymbol rot dargestellt. Wenn die Spannung zwischen den beiden Werten liegt,
              erfolgt die Darstellung des Batteriesymbols in Orange.
]

== Polar Settings
<polar-settings>

#tr[Um die korrekten Sollfahrt Informationen zu erhalten, müssen Sie die richtigen Polarenwerte für
Ihren Segelflugzeugtyp einstellen. Das LARUS Vario Display verfügt werksseitig über mehr als 200
Polaren von verschiedenen Segelflugzeugen.

Sollten Sie Ihren Segelflugzeugtyp nicht in der Liste finden, können Sie eine beliebige
Segelflugzeugpolare auswählen und die einzelnen Einstellungen auf die Werte Ihrer
Segelflugzeugpolare ändern.]

=== Glider

#tr[Wählen Sie die richtige oder nächstliegende Polare aus. Der Name des Flugzeugtyps kann nicht
verändert werden.]

#picnote("/img/pictograph-yellow-warning.svg")[
      #tr[Die Auswahl eines Flugzeugtyps überschreibt alle
        folgenden Einstellungen wie Leergewicht, maximaler Wasserballast usw. Dies kann nicht
        rückgängig gemacht werden, auch wenn später wieder der identische Typ ausgewählt wird. Alle
        spezifischen Werte müssen dann erneut angegeben werden.]
]

=== Empty Mass

#tr[Nach der Auswahl des Segelflugzeugtyps sollten Sie das Leergewicht (ohne Gewicht des Piloten)
Ihres Segelflugzeugs anpassen, damit die Berechnungen korrekt durchgeführt werden können.]

=== Max Ballast
#picnote("/img/pictograph-yellow-warning.svg")[
      #tr[Achten Sie darauf, dass der maximale Wasserballast mit den
    Angaben von XCSoar/OpenSoar übereinstimmt, da ansonsten der Abgleich des Wasserballastes nicht
    korrekt funktoniert.]
]
    
=== Reference Weight

#tr[Die nachfolgenden angegebenen Sinkwerte zur Polare beziehen sich auf ein Segelflugzeug mit
der hier vorgegebenen Referenzmasse.]

=== Polar v1, v2, v3, si1, si2, si3

#tr[Die Geschwindigkeiten und Sinkwerte beschreiben die Leistung des genutzen Segelflugzeuges. Die
Polare wird wie üblich durch eine quadratische Gleichung abgebildet. Wichtig ist der
Geschwindigkeitsbereich in dem zwischen den Aufwinden geflogen wird, damit die Sollfahrt korrekt
berechnet werden kann.]

#figure(
    image("/img/replicated-polar.svg", width: 12cm),
    caption: [#hr[ASG 32 Flugzeugpolare laute Datenblatt und Näherung]],
)<replicated-polar>

#tr[Die Polaren klassischer Flugzeuge wie der ASW 20 oder der LS 3 lassen sich mit einer quadratischen Näherung perfekt nachbilden. Bei modernen Flugzeugen ist es hingegen wichtig, den relevanten Geschwindigkeitsbereich zu treffen. Darüber hinaus ist die Nachbildung der Polare ungenau. Im Beispiel ist die Polare der ASG 32  von Alexander Schleicher Flugzeugbau @asg32 zu sehen, die im Bereich von 100 km/h bis 180 km/h sehr gut abgebildet wird, darüber hinaus jedoch nicht. Die Abweichung im unteren Bereich kann vernachlässigt werden, im oberen Bereich sollte man sie jedoch berücksichtigen. Der Sollfahrtgeber wird bei sehr gutem Wetter zu hohe Geschwindigkeiten vorgeben.]

== Sensor Box
<sensor-box>

=== #hr[Kalibrierung der LARUS Sensoreinheit]
<sensorunit-calibration>

#tr[Bevor Sie Ihren ersten Flug beginnen, müssen die Lagesensoren des LARUS-Sensoreinheit präzise
eingestellt werden. Die Kalibrierungsschritte durch einen einfachen Ablauf durchgeführt, der
nachfolgend beschrieben ist und über Funktionen im LARUS Vario Display angestoßen wird. Die
Kalibrierung erfolgt in zwei Schritten.]

=== #hr[Erste Kalibrierung am Boden:]

#tr[Bauen Sie Ihren Segelflugzeug auf und stellen Sie ihn auf eine ebene Fläche. Nachdem Sie die
einzelnen Positionen eingenommen haben, warten Sie, bis keine Vibrationen mehr im Flugzeug zu spüren
sind, bevor Sie mit der Kalibrierung fortfahren. Verwenden Sie keinen Heckwagen, um die vertikale
Achse Ihres Segelflugzeugs während der folgenden Vorgänge zu fixieren.

- *Left Wing Down:* Legen Sie den linken Flügel ab, warten Sie kurz und rufen Sie die Funktion auf
- *Right Wing Down:* Legen Sie den rechten Flügel ab, warten Sie kurz und rufen Sie die Funktion auf.
- *Wings Straight:* Halten Sie den Flügel horizontal, warten Sie, rufen Sie die Funktion auf.
- *Calc Orientation:* Für diesen Schritt ist es wichtig, alle drei zuvor
            genannten Schritte durchzuführen. Die Reihenfolge der Schritte spielt keine Rolle,
            jedoch müssen sie vollständig abgeschlossen sein.
]

=== #hr[Feinjustage in der Luft:]

#tr[Die exakte Neigungswinkelkalibrierung wird während des Fluges durchgeführt. Es wird empfohlen,
 diesen Schritt in einem Flug durchzuführen, der nicht durch thermische Böen gestört wird. Richten
 Sie Ihr Segelflugzeug bei der Geschwindigkeit mit der besten Gleitzahl aus (wenn Sie Wölbklappen
 haben, stellen Sie diese auf diese Geschwindigkeit ein). Rufen Sie #keep[*Straight Flight*] auf.
 Damit sind Sie fertig. Sie können die Kalibrierung überprüfen, indem Sie zur Anzeige des
 künstlichen Horizont (#keep[@horizon]) wechseln.]

 === Reset Sensorbox
 
 #tr[Diese Funktion löst einen Neustart der Sensorbox aus.]

 === Init Settings
 
 #tr[Diese Einstellmöglichkeiten für die LARUS Sensoreinheit ist Experten vorbehalten und wird hier
 nicht näher beschreiben.]