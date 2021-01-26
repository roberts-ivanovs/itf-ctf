import React, { ReactElement, useCallback, useState } from 'react';
import { AnswerlessFlag } from '../../utils/types';

interface Props {
  flag: AnswerlessFlag
}

export function Flag({ flag }: Props): ReactElement {
  const [email, setEmail] = useState('');
  const [answer, setAnswer] = useState('');

  const submit = useCallback(
    () => {

    },
    [flag.id,  ],
  )

  return (
    <div>
      <h3>
        {flag.name}
      </h3>
      <div>
        {flag.description}
      </div>
      <a href={`/common_static/${flag.filepath}`} download>
        Lejupielādēt uzdevumu
      </a>
      <div>
        <div>
          e-pasts:
          <input type="text" onChange={(e) => setEmail(e.target.value)} value={email} />
        </div>
        <div>
          atbilde:
          <input type="text" onChange={(e) => setAnswer(e.target.value)} value={answer} />
        </div>
        <button type="button">
          Iesniegt atbild
        </button>
      </div>
    </div>
  );
}
