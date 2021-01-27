import { AxiosError } from "axios";
import React, { ReactElement, useCallback, useEffect, useState } from 'react';
import { Link } from 'react-router-dom';
import { Requester } from '../../utils/Requester';
import { BasicAPI, Flag } from '../../utils/types';

async function getFlags(callback: (arg0: Array<Flag>) => void): Promise<void> {
  const res = await Requester.getAllFlagsWithAnswers();
  callback(res.data);
}
export function RootAdmin(): ReactElement {
  const [flags, setFlags] = useState<Array<Flag>>([]);
  const [status, setStatus] = useState<ReactElement>();

  useEffect(() => {
    void getFlags(setFlags);
  }, []);


  const del = useCallback(
    async (flagId: number) => {
      await Requester.deleteFlag(flagId).then((e) => setStatus(
        <div className="alert alert-success">
          Deleted
        </div>,
      )).catch((e: AxiosError<BasicAPI<string>>) => {
        setStatus((
          <div className="alert alert-danger">
            {e.response?.data.msg}
          </div>));
        return null;
      });
      await getFlags(setFlags);
    },
    [],
  );

  return (
    <>
      <Link className="btn btn-success" to="/veryobfuscatedadminpanel/new">
        CREATE A NEW FLAG
      </Link>
      {status}
      <table className="table">
        <thead>
          <tr>
            <th scope="col">ID</th>
            <th scope="col">Name</th>
            <th scope="col">Descirption</th>
            <th scope="col">Answer</th>
            <th scope="col">File</th>
            <th scope="col">Edit</th>
            <th scope="col">Delete</th>
          </tr>
        </thead>
        <tbody>
          {flags.map((e) => (
            <tr key={e.id}>
              <th scope="row">{e.id}</th>
              <td>{e.name}</td>
              <td>{e.description}</td>
              <td>{e.answer}</td>
              <td>{e.filepath
                && (
                <a href={`/common_static/${e.filepath}`} download>
                  <p>Download attached file</p>
                </a>
                )}
              </td>
              <td>
                <Link className="btn btn-info" to={`/veryobfuscatedadminpanel/edit/${e.id}`}>Edit</Link>
              </td>
              <td>
                <button type="button" className="btn btn-danger" onClick={() => del(e.id)}>
                  Delete
                </button>
              </td>
            </tr>
          ))}
        </tbody>
      </table>
    </>
  );
}
