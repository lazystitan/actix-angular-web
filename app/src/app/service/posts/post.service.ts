import {Injectable} from '@angular/core';
import {MessageService} from '../message.service';
import {Observable} from 'rxjs';
import {Post, PostInsert} from '../../model/post';
import {HttpClient} from '@angular/common/http';

@Injectable({
  providedIn: 'root'
})
export class PostService {

  private postsUrl = 'posts';
  private postUrl = 'post';

  constructor(
    private http: HttpClient,
    private messageService: MessageService
  ) {
  }

  getPosts(): Observable<Post[]> {
    this.log('PostService: fetched posts');
    return this.http.get<Post[]>(this.postsUrl);
  }

  getPost(id: number): Observable<Post> {
    this.messageService.add(`PostService: fetched post id=${id}`);
    return this.http.get<Post>(this.postUrl + `/${id}`);
  }

  addPost(post: PostInsert): Observable<any> {
    this.messageService.add(`PostService: add post`);
    return this.http.post<PostInsert>(this.postUrl, post);
  }

  deletePost(postId: number): Observable<any> {
    this.messageService.add(`PostServcie: delete post ${postId}`);
    return this.http.delete<number>(this.postUrl + `/${postId}`);
  }

  update(post: PostInsert, postId: number): Observable<any> {
    this.messageService.add(`PostServcie: update post ${postId}`);
    return this.http.patch<PostInsert>(this.postUrl + `/${postId}`, post);
  }

  private log(message: string): void {
    this.messageService.add(`PostService: ${message}`);
  }
}
