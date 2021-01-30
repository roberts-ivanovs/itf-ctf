import React, { ReactElement } from 'react';

export function About(): ReactElement {
  return (
    <div className="container pt-2">
      <h2 className="text-center">About</h2>
      <p className="p-3">
        Šogad aprit 15 gadi kopš Ventspils Augstskolas Informācijas Tehnoloģiju fakultātes dibināšanas. Svinēsim attālināti, bet kopā šo jubileju svinīgā pasākumā “ITF days”, kas notiks 4.februārī plkst. 14:00 tiešsaistē bbb vidē.
        Gaidot pasākumu, Tev ir iespēja piedalīties VeA ITF “Fakultātes logo izstrādes konkursā” un Capture the flag jeb karoga zagšanas izaicinājumā.
        Capture the flag jeb karoga zagšana ir izaicinājums Tev palauzt galvu, pildot dažādus uzdevumus, kurus atradīsi lapā ctf.venta.lv . Izpildi uzdevumus ātrāk par visiem, iesūti pareizās atbildes un tad tieši Tevi mēs redzēsim uzvarētāju paziņošanā svinīgajā pasākumā “ITF days”, kas notiks 4.februārī tiešsaistē.
        ENG

      </p>
      <p className="p-3">
        Capture the flag is a challenge for you to scratch your head to by doing different tasks you can find on ctf.venta.lv . Finish your tasks, submit the answers and we will see you at the awards ceremony of event “ITF days” which will be held on 4th of February online.

        This year marks 15 years of formation of the Information Technology faculty of Ventspils University of Applied Sciences. We will celebrate together this anniversary remotely on event “ITF days” which will be held on 4th of February online bbb .
        While you wait for this event, you can participate in “IT faculty logo competition” and challenge “Capture the flag”.
      </p>
      <h2 className="text-center">How to</h2>
      <p className="pl-3">
        The answers are hidden somewhere in the task.
      </p>
      <p className="pl-3">
        They can be distinguished usually by some kind of a recognisable prefix, like - <b>veactf2021{'{answer}'}</b>.
      </p>
      <p className="pl-3">
        Upon finding the answer, you must enter it in the <b>Answer</b> input field, as well as the <b>e-mail</b>
        you wish to be recognised by. Upon submitting, you'll get a randomly generated name that will be visible in the scoreboard.
      </p>
    </div>
  );
}
