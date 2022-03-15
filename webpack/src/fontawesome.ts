import { library, dom } from "@fortawesome/fontawesome-svg-core";
import {
  faExclamationTriangle,
  faCircleCheck,
  faCircleInfo,
} from "@fortawesome/free-solid-svg-icons";

library.add(faExclamationTriangle, faCircleCheck, faCircleInfo);

dom.watch();
