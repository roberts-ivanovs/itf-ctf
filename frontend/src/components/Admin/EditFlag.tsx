/* eslint-disable jsx-a11y/label-has-associated-control */
import React, {
  ReactElement, useCallback, useEffect, useState,
} from 'react';
import { useRouteMatch } from 'react-router-dom';
import { Requester } from '../../utils/Requester';
import {
  AnswerlessFlag, FinalUpdateFlag, Flag, UpdateFlag,
} from '../../utils/types';

async function getFlag(falgId: number, callback: (arg0: Flag) => void): Promise<void> {
  const res = await Requester.getSingleFlag(falgId);
  callback(res.data);
}

interface Match {
  id: string
}

export function EditFlag(): ReactElement {
  const m = useRouteMatch<Match>();
  const [existingFlag, setExistingFlag] = useState<Flag>();
  const [file, setFile] = useState<string|null>(null);

  useEffect(() => {
    if (m.params.id) {
      void getFlag(Number(m.params.id), setExistingFlag);
    }
  }, [m.params.id]);

  const save = useCallback(
    async (withFile: boolean) => {
      if (m.params.id && existingFlag) {
        const flagWithoutAnswer: FinalUpdateFlag = { flag: existingFlag, file: withFile ? file : null };
        const res = await Requester.updateFlag(Number(m.params.id), flagWithoutAnswer);
      }
    },
    [m.params.id, existingFlag, file],
  );

  return (
    <>
      {existingFlag
        && (
        <div className="container mt-3">
          <h4> Edit flag</h4>
          <form>
            <div className="form-group">
              <label htmlFor="Name">Name</label>
              <input
                type="text"
                className="form-control"
                id="Name"
                aria-describedby="emailHelp"
                value={existingFlag?.name}
                onChange={(e) => existingFlag && setExistingFlag({ ...existingFlag, name: e.target.value })}
              />
            </div>
            <div className="form-group">
              <label htmlFor="Description">Description (as valid HTML)</label>
              <textarea
                className="form-control"
                id="Description"
                value={existingFlag?.description}
                onChange={(e) => existingFlag && setExistingFlag({ ...existingFlag, description: e.target.value })}
              />
            </div>
            <div className="form-group">
              <label htmlFor="Answer">Answer</label>
              <input
                type="text"
                className="form-control"
                id="Answer"
                aria-describedby="emailHelp"
                value={existingFlag?.answer}
                onChange={(e) => existingFlag && setExistingFlag({ ...existingFlag, answer: e.target.value })}
              />
            </div>
            <button type="button" className="btn btn-success" onClick={() => save(false)}>
              SAVE
            </button>
          </form>
          <h4> Update with file</h4>
          <div className="PostImageUpload">
            <h2>Upload file (will be saved with .zip appendix)</h2>
            <input
              type="file"
              id="image"
              name="image"
              onChange={(e) => {
                if (e.target.files && e.target.files[0]) {
                  let res: string | ArrayBuffer = '';
                  const reader = new FileReader();
                  reader.onload = (e2) => {
                    const t = e2.target;
                    if (t !== null && t.result !== null) {
                      res = t.result;
                    }
                    setFile(res as string);
                  };
                  reader.readAsDataURL(e.target.files[0]);
                }
              }}
            />
          </div>
          <button type="button" className="btn btn-success" onClick={() => save(true)}>
            SAVE WITH FILE
          </button>
        </div>
        )}
    </>
  );
}
