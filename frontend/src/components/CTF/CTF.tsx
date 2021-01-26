import React, { ReactElement, useEffect, useState } from 'react';
import { Requester } from '../../utils/Requester';
import { AnswerlessFlag } from '../../utils/types';
import { Flag } from './Flag';

import style from './CTF.module.scss';

async function getFlags(callback: (arg0: Array<AnswerlessFlag>) => void): Promise<void> {
  const res = await Requester.getAllFlags();
  callback(res.data);
}

export function CTF(): ReactElement {
  const [flags, setFlags] = useState<Array<AnswerlessFlag>>([]);

  useEffect(() => {
    void getFlags(setFlags);
  }, []);

  return (
    <div className={`${style['content-wrapper']}`}>
      <h1>CTF Title here</h1>
      <div className={style['task-wrapper']}>
        {flags.map((e) => (
          <React.Fragment key={e.id}>
            <div key={e.id} className={`${style['flag-box']}`} data-toggle="modal" data-target={`#modal${e.id}`}>
              <div className={`${style['gears-icon']}`}>
                <i className="fas fa-cogs" />
              </div>
              <div className={`${style['flag-title']}`}>
                <p>{e.name}</p>
              </div>
            </div>
            <Flag key={`${e.id}modal`} id={e.id.toString()} flag={e} />
          </React.Fragment>
        ))}
      </div>
    </div>
  );
}
