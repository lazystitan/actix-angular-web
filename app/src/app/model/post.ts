export interface Post {
  id: number,
  title: string,
  author: string,
  digest: string,
  content: string,
  create_time: string,
  last_update_time: string
}

export interface PostInsert {
  title: string,
  digest: string,
  content: string
}
