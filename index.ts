import('./pkg/index').then(rust => {
    console.log(rust.hello());
});
