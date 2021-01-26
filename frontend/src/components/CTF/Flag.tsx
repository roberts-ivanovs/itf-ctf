/* eslint-disable react-hooks/exhaustive-deps */
import React, { ReactElement, useCallback, useState } from 'react';
import { AnswerlessFlag } from '../../utils/types';

interface Props {
  id: string;
  flag: AnswerlessFlag;
}

export function Flag({ flag, id }: Props): ReactElement {
  const [email, setEmail] = useState('');
  const [answer, setAnswer] = useState('');

  const submit = useCallback(
    () => {

    },
    [flag.id],
  );

  return (
    <div id={`modal${id}`} className="modal" tabIndex={-1} role="dialog">
      <div className="modal-dialog" role="document">
        <div className="modal-content">
          <div className="modal-header">
            <h5 className="modal-title">Modal title</h5>
            <button type="button" className="close" data-dismiss="modal" aria-label="Close">
              <span aria-hidden="true">&times;</span>
            </button>
          </div>
          <div className="modal-body">
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
          </div>
          <div className="modal-footer">
            <button type="button" className="btn btn-primary">Save changes</button>
            <button type="button" className="btn btn-secondary" data-dismiss="modal">Close</button>
          </div>
        </div>
      </div>
    </div>
  );
}
