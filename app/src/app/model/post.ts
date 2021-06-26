export interface Post {
  id: number,
  title: string,
  author: string,
  content: string,
  create_time: string,
  last_update_time: string
}

export interface PostInsert {
  title: string,
  content: string
}
