import { Injectable } from '@angular/core';
import {MessageService} from "../message.service";
import {Observable} from "rxjs";
import {Post, PostInsert} from "../../model/post";
import {HttpClient} from "@angular/common/http";

@Injectable({
  providedIn: 'root'
})
export class PostService {

  constructor(
    private http: HttpClient,
    private messageService: MessageService
  ) { }

  private log(message: string) {
    this.messageService.add(`PostService: ${message}`);
  }

  private postsUrl = 'posts';
  private postUrl = 'post';

  getPosts(): Observable<Post[]> {
    this.log('PostService: fetched posts');
    return this.http.get<Post[]>(this.postsUrl);
  }

  getPost(id:Number): Observable<Post> {
    this.messageService.add(`PostService: fetched post id=${id}`);
    return this.http.get<Post>(this.postUrl + `/${id}`);
  }

  addPost(post: PostInsert): Observable<any> {
    this.messageService.add(`PostService: add post`);
    return this.http.post<PostInsert>(this.postUrl, post);
  }

  deletePost(post_id: number): Observable<any> {
    this.messageService.add(`PostServcie: delete post ${post_id}`);
    return this.http.delete<number>(this.postUrl + `/${post_id}`);
  }
}
