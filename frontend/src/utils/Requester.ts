import axios, { AxiosError } from 'axios';
import {
  AnswerlessFlag, BasicAPI, Flag, PostAnswer, Register, ScoreRaw, User,
} from './types';

// TODO This is not needed if we're not using sessions
const apiClient = axios.create({
  xsrfCookieName: 'csrftoken',
  xsrfHeaderName: 'X-CSRFTOKEN',
});

const urls = {
  flag: '/api/v1/ctf/flag',
  score: '/api/v1/ctf/score',
  users: '/api/v1/ctf/user',
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
  getUserId = (email: string): Promise<BasicAPI<number>> => get(`${urls.users}/email/${email}`, {});

  getAllFlags = (): Promise<BasicAPI<Array<AnswerlessFlag>>> => get(urls.flag, {});

  getAllScores = (): Promise<BasicAPI<Array<ScoreRaw>>> => get(urls.score, {});

  postAnswer = (params: PostAnswer): Promise<BasicAPI<string>> => post(urls.score, params);

  postCreateUser = (params: Register): Promise<BasicAPI<number>> => post(urls.users, params);
}

const requester = new Requester();

export { requester as Requester };
