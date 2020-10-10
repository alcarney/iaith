const rust = import('./pkg');

rust
    .then(m => m.execute('++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++.'))
    .catch(console.error);
