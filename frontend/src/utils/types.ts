export interface BasicAPI<T> {
  status: number,
  msg: string,
  data: T
}

export interface AnswerlessFlag {
  id: number,
  name: string,
  description: string,
  filepath: string,
}

export interface Flag extends AnswerlessFlag {
  answer: string,
}

export interface User {
  id: number,
  email: string,
  name: string,
}

export interface Register {
  email: string,
}

interface ScoreRawCreate {
  flagId: number,
  userId: number,
}

export interface Score {
  user: User,
  flags: Array<AnswerlessFlag>,
  score: number,
}

export interface PostAnswer {
  metadata: ScoreRawCreate,
  answer: string,
}
