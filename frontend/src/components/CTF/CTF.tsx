/* eslint-disable jsx-a11y/control-has-associated-label */
import React, { ReactElement, useEffect, useState } from 'react';
import { Requester } from '../../utils/Requester';
import { AnswerlessFlag, FlagResponse } from '../../utils/types';
import { Flag } from './Flag';

import style from './CTF.module.scss';
import { Event } from "../Adout/Event";

async function getFlags(callback: (arg0: Array<FlagResponse>) => void): Promise<void> {
  const res = await Requester.getAllFlags();
  callback(res.data);
}

export function CTF(): ReactElement {
  const [flags, setFlags] = useState<Array<FlagResponse>>([]);

  useEffect(() => {
    void getFlags(setFlags);
  }, []);

  return (
    <div className={`${style['content-wrapper']}`}>
      <Event />
      <div className={style['task-wrapper']}>
        {flags.map((e) => (
          <React.Fragment key={e.flag.id}>
            <div className={`${style['flag-box']}`} data-toggle="modal" data-target={`#modal${e.flag.id}`}>
              <div className={`${style['gears-icon']}`}>
                <i className="fas fa-cogs" />
              </div>
              <div className={`${style['flag-title']}`}>
                <abbr title={`Correct answers: ${e.totalAnswers}`}>
                  <div className="row">
                    <div className="col-2">
                      Difficulty
                    </div>
                    <div className="col">
                      <div className="progress">
                        <div
                          className="progress-bar progress-bar-striped bg-danger progress-bar-animated"
                          role="progressbar"
                          style={{ width: `${100 - (e.totalAnswersOfPopulation * 100)}%` }}
                        />
                      </div>

                    </div>
                  </div>
                </abbr>
                <p>{e.flag.name}</p>
              </div>
            </div>
            <Flag key={`${e.flag.id}modal`} id={e.flag.id.toString()} flag={e.flag} />
          </React.Fragment>
        ))}
      </div>
    </div>
  );
}
