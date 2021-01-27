/* eslint-disable react-hooks/exhaustive-deps */
import { AxiosError } from 'axios';
import React, { ReactElement, useCallback, useState } from 'react';
import { Requester } from '../../utils/Requester';
import { AnswerlessFlag, BasicAPI } from '../../utils/types';

import style from './CTF.module.scss';

interface Props {
  id: string;
  flag: AnswerlessFlag;
}

export function Flag({ flag, id }: Props): ReactElement {
  const [email, setEmail] = useState('');
  const [answer, setAnswer] = useState('');
  const [status, setStatus] = useState<ReactElement>();
  const [status2, setStatus2] = useState<ReactElement>();

  const submit = useCallback(
    async () => {
      // Check if user exists
      const userId = await Requester.getUserId(email)
        .then((e) => e.data)
        .catch(async () => {
          // If does not exist -> register
          const resNew = await Requester.postCreateUser({ email }).then(async (e) => Requester.getUserId(email).then((e) => e.data))
            .catch((e) => {
              const error = e.response.data as BasicAPI<null>;
              setStatus((
                <div className="alert alert-danger">
                  {error.msg}
                </div>));
              return null;
            });
          return resNew;
        });

      if (!userId) return;

      setStatus2(
        <div className="alert alert-success">
          Your secret name -- {userId.name}
        </div>,
      );

      // Send request to save score
      // TODO save to localstorage context
      await Requester.postAnswer(
        {
          metadata: {
            flagId: flag.id,
            userId: userId.id,
          },
          answer,
        },
      ).then((e) => setStatus(
        <div className="alert alert-success">
          {e.data}
        </div>,
      )).catch((e: AxiosError<BasicAPI<string>>) => {
        // const error = e.response.data as BasicAPI<string>;
        // console.log('here', error);
        // e.response?.data
        setStatus((
          <div className="alert alert-danger">
            {e.response?.data.msg}
          </div>));
        return null;
      });
    },
    [answer, email, flag.id],
  );

  return (
    <div className="modal fade" id={`modal${id}`} tabIndex={-1} role="dialog" aria-labelledby={`modalLabel${id}`} aria-hidden="true">
      <div className={`modal-dialog modal-dialog-centered ${style['modal-window-box']}`} role="document">
        <div className="modal-content">

          <div className="modal-header">
            <h2 className="modal-title" id={`modalLabel${id}`}>{flag.name}</h2>
            <button type="button" className="close" data-dismiss="modal" aria-label="Close">
              <span aria-hidden="true">&times;</span>
            </button>
          </div>

          <div className="modal-body">
            <div className={`${style['modal-form-wrapper']}`}>
              {status}
              {status2}
              <div className={`${style.description}`}>
                <div dangerouslySetInnerHTML={{ __html: flag.description }} />
              </div>
              {flag.filepath
                && (
                <a className={`${style.download}`} href={`/common_static/${flag.filepath}`} download>
                  <i className="fas fa-file-download" />
                  <p>Download file</p>
                </a>
                )}
              <div>
                <div className={`${style.input}`}>
                  <p>E-mail:</p>
                  <input type="email" onChange={(e) => setEmail(e.target.value)} value={email} />
                </div>
                <div className={`${style.input}`}>
                  <p>Answer:</p>
                  <input type="text" onChange={(e) => setAnswer(e.target.value)} value={answer} />
                </div>
                <div className={`${style['btn-wrapper']}`}>
                  <button className={`${style['submit-btn']}`} type="button" onClick={submit}>
                    Submit
                  </button>
                </div>
              </div>
            </div>
          </div>

          <div className={`modal-footer ${style['modal-footer-wrapper']}`}>
            <button type="button" className={`btn ${style.close}`} data-dismiss="modal">Close</button>
          </div>

        </div>
      </div>
    </div>
  );
}
