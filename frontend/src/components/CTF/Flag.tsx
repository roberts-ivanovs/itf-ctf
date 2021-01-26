/* eslint-disable react-hooks/exhaustive-deps */
import React, { ReactElement, useCallback, useState } from 'react';
import { Requester } from '../../utils/Requester';
import { AnswerlessFlag, BasicAPI } from '../../utils/types';

interface Props {
  id: string;
  flag: AnswerlessFlag;
}

export function Flag({ flag, id }: Props): ReactElement {
  const [email, setEmail] = useState('');
  const [answer, setAnswer] = useState('');
  const [status, setStatus] = useState<ReactElement>();

  const submit = useCallback(
    async () => {
      // Check if user exists
      const userId = await Requester.getUserId(email)
        .then((e) => e.data)
        .catch(async () => {
          // If does not exist -> register
          const resNew = await Requester.postCreateUser({ email }).then((e) => e.data)
            .catch((e) => {
              const error = e.response.data as BasicAPI<null>;
              console.log('here', error);
              setStatus((
                <div className="alert alert-danger">
                  {error.msg}
                </div>));
              return null;
            });
          return resNew;
        });

      if (!userId) return;

      // Send request to save score
      // TODO save to localstorage context
      await Requester.postAnswer(
        {
          metadata: {
            flagId: flag.id,
            userId,
          },
          answer,
        },
      ).then((e) => setStatus(
        <div className="alert alert-success">
          {e.data}
        </div>,
      )).catch((e) => {
        const error = e.response.data as BasicAPI<string>;
        console.log('here', error);
        setStatus((
          <div className="alert alert-danger">
            {error.msg}
          </div>));
        return null;
      });
    },
    [answer, email, flag.id],
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
              {status}
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
                  <input type="email" onChange={(e) => setEmail(e.target.value)} value={email} />
                </div>
                <div>
                  atbilde:
                  <input type="text" onChange={(e) => setAnswer(e.target.value)} value={answer} />
                </div>
                <button type="button" onClick={submit}>
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
