import {TestBed} from '@angular/core/testing';

import {APIInterceptorInterceptor} from './api.interceptor';

describe('APIInterceptorInterceptor', () => {
  beforeEach(() => TestBed.configureTestingModule({
    providers: [
      APIInterceptorInterceptor
    ]
  }));

  it('should be created', () => {
    const interceptor: APIInterceptorInterceptor = TestBed.inject(APIInterceptorInterceptor);
    expect(interceptor).toBeTruthy();
  });
});
