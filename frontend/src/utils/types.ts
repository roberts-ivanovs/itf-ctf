export interface BasicAPI<T> {
  status: number,
  message: string,
  data: T
}

export interface AnswerlessFlag {
  id: number,
  name: string,
  description: string,
  filepath: string,
}
