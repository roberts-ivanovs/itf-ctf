import axios, { AxiosError } from 'axios';
import { AnswerlessFlag, BasicAPI } from './types';

// TODO This is not needed if we're not using sessions
const apiClient = axios.create({
  xsrfCookieName: 'csrftoken',
  xsrfHeaderName: 'X-CSRFTOKEN',
});

const urls = {
  tasks: '/api/v1/ctf/flag',
};

async function get<T, B>(url: string, params: B): Promise<T> {
  const response = await apiClient
    .get<T>(url, { params })
    .then((val) => val.data);
  return response;
}

async function post<T, B>(url: string, params: B): Promise<T> {
  const response = await apiClient
    .post<T>(url, params)
    .then((val) => val.data);

  return response;
}

class Requester {
  getAllFlags = (): Promise<BasicAPI<Array<AnswerlessFlag>>> => get(urls.tasks, {});
}

const requester = new Requester();

export { requester as Requester };
