import {Component, OnInit, Input} from '@angular/core';

import {Post} from '../../model/post'
import {TokenStorageService} from "../../service/auth/token-storage.service";
import {PostService} from "../../service/posts/post.service";

@Component({
  selector: 'app-post-card',
  templateUrl: './post-card.component.html',
  styleUrls: ['./post-card.component.scss']
})
export class PostCardComponent implements OnInit {

  @Input() post: Post = {
    id: -1,
    title: '',
    author: '',
    content: ``,
    create_time: '',
    last_update_time: ''
  };

  constructor(
    public tokenService: TokenStorageService,
    private postService: PostService
  ) {
  }

  ngOnInit(): void {
  }

  deletePost(id : number) {
    this.postService.deletePost(id).subscribe(data => {
      if (data.code == 0) {
        alert('删除成功！');
        window.location.reload();
      } else {
        alert(data.message);
      }
    })
  }

}
