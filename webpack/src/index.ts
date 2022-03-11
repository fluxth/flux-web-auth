import "bootstrap";

import "bootstrap/dist/css/bootstrap.min.css";
import "./styles/index.scss";

import { library, dom } from "@fortawesome/fontawesome-svg-core";
import { faExclamationTriangle } from "@fortawesome/free-solid-svg-icons";

library.add(faExclamationTriangle);

dom.watch();
