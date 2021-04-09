import { Injectable } from '@angular/core';
import {MessageService} from "../message.service";
import {Observable, of} from "rxjs";
import {Post} from "../post";
import {HttpClient} from "@angular/common/http";
import {POSTS} from "../mock-posts";

@Injectable({
  providedIn: 'root'
})
export class PostService {

  constructor(
    private http: HttpClient,
    private messageService: MessageService
  ) { }

  private log(message: string) {
    this.messageService.add(`PostService: ${message}`)
  }

  private postsUrl = 'posts';

  getPosts(): Observable<Post[]> {
    this.log('PostService: fetched posts')
    return this.http.get<Post[]>(this.postsUrl)
  }

  getPost(id:Number): Observable<Post> {
    this.messageService.add(`PostService: fetched post id=${id}`);
    if (POSTS.find(post => post.id === id) !== undefined) {
      return of(POSTS.find(post => post.id === id) as Post);
    } else {
      return of({
        id: 0,
        author: "",
        content: "",
        createTime: "",
        lastUpdateTime: "",
        title: ""
      });
    }
  }
}
