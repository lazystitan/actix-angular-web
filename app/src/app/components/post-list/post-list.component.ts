import {Component, OnInit} from '@angular/core';

import {PostService} from "../../service/posts/post.service";
import {Post} from "../../model/post";

@Component({
  selector: 'app-post-list',
  templateUrl: './post-list.component.html',
  styleUrls: ['./post-list.component.scss']
})
export class PostListComponent implements OnInit {

  posts:Post[] = [];

  constructor(
    private postService: PostService
  ) {
  }

  ngOnInit(): void {
    this.getPosts()
  }

  getPosts():void {
    this.postService.getPosts().subscribe(posts => this.posts = posts)
  }

}
