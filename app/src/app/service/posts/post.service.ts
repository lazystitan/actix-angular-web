import { Injectable } from '@angular/core';
import {MessageService} from "../message.service";
import {Observable, of} from "rxjs";
import {Post} from "../../model/post";
import {HttpClient} from "@angular/common/http";
import {POSTS} from "../../mock/mock-posts";

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
  private postUrl = 'post';

  getPosts(): Observable<Post[]> {
    this.log('PostService: fetched posts')
    return this.http.get<Post[]>(this.postsUrl)
  }

  getPost(id:Number): Observable<Post> {
    this.messageService.add(`PostService: fetched post id=${id}`);
    return this.http.get<Post>(this.postUrl + `/${id}`)
  }
}