import {Component, OnInit} from '@angular/core';
import {FormBuilder, FormGroup} from '@angular/forms';
import {PostService} from '../../service/posts/post.service';
import {PostInsert} from '../../model/post';
import {ActivatedRoute, Router} from '@angular/router';
import {MatSnackBar} from '@angular/material/snack-bar';

@Component({
  selector: 'app-post-edit',
  templateUrl: './post-edit.component.html',
  styleUrls: ['./post-edit.component.scss']
})
export class PostEditComponent implements OnInit {

  postForm: FormGroup;
  editPostId = 0;

  constructor(
    private formBuilder: FormBuilder,
    private postService: PostService,
    private router: Router,
    private snackBar: MatSnackBar,
    private route: ActivatedRoute,
  ) {
    this.postForm = this.formBuilder.group({
      title: '',
      digest: '',
      content: '',
    });
  }

  ngOnInit(): void {
    this.getPost();
  }

  getPost(): void {
    let id;
    if (this.route.snapshot.paramMap.get('id')) {
      id = parseInt(this.route.snapshot.paramMap.get('id') ?? '0', 0);
      this.postService.getPost(id)
        .subscribe(post => {
          this.postForm = this.formBuilder.group({
            title: post.title,
            digest: post.digest,
            content: post.content,
          });
          this.editPostId = post.id;
        });
    }
  }

  onSubmit(post: PostInsert): void {
    const resProcess = (res: any) => {
      if (res.code === 0) {
        this.snackBar.open('成功', undefined, {duration: 3000});
        setTimeout(() => {
          this.router.navigate([`/post/${res.id}`]);
        }, 1000);
      } else {
        this.snackBar.open('失败', undefined, {duration: 3000});
      }
    };
    if (this.editPostId !== 0) {
      this.postService.update(post, this.editPostId).subscribe(res => {
        resProcess(res);
      });
    } else {
      this.postService.addPost(post).subscribe(res => {
        resProcess(res);
      });
    }
  }

}
