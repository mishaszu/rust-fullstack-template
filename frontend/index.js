import './src/style.scss';

import("./pkg").then(module => {
  module.run_app();
});
