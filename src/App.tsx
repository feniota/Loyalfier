import { DrawerProps } from "@fluentui/react-components";
import * as React from "react";
import {
  NavDivider,
  NavDrawer,
  NavDrawerBody,
  NavItem,
} from "@fluentui/react-nav-preview";

import {
  Label,
  Radio,
  RadioGroup,
  Switch,
  makeStyles,
  tokens,
  useId,
} from "@fluentui/react-components";
import {
  Board20Filled,
  Board20Regular,
  Settings20Filled,
  Settings20Regular,
  History20Filled,
  History20Regular,
  Info20Filled,
  Info20Regular,
  bundleIcon
} from "@fluentui/react-icons";

import "./App.css";

const useStyles = makeStyles({
  root: {
    overflow: "hidden",
    display: "flex",
    height: "100dvh",
  },
  content: {
    flex: "1",
    padding: "16px",
    display: "grid",
    justifyContent: "flex-start",
    alignItems: "flex-start",
  },
  field: {
    display: "flex",
    marginTop: "4px",
    marginLeft: "8px",
    flexDirection: "column",
    gridRowGap: tokens.spacingVerticalS,
  },
  title: {
    fontSize: "24px",
    fontWeight: "bold",
    marginTop: "16px",
    marginBottom: "4px",
    marginLeft: "8px",
    marginRight: "8px",
  },
  subTitle: {
    fontSize: "12px",
    opacity: 0.5,
    fontFamily: "Noto Serif SC",
    marginTop: "0px",
    marginBottom: "4px",
    marginLeft: "8px",
    marginRight: "8px",
  },
  navItemBottom: {
    marginTop: "auto",
    marginBottom: "12px",
  }
});


const Dashboard = bundleIcon(Board20Filled, Board20Regular);
const Settings = bundleIcon(Settings20Filled, Settings20Regular);
const History = bundleIcon(History20Filled, History20Regular);
const About = bundleIcon(Info20Filled, Info20Regular);

type DrawerType = Required<DrawerProps>["type"];

function App() {
  const styles = useStyles();

  const typeLableId = useId("type-label");
  const linkLabelId = useId("link-label");
  const multipleLabelId = useId("multiple-label");

  const [type, setType] = React.useState<DrawerType>("inline");
  const [isMultiple, setIsMultiple] = React.useState(true);

  return (
    <div className={styles.root}>
      <NavDrawer
        defaultSelectedValue="2"
        defaultSelectedCategoryValue=""
        open={true}
        type={type}
        multiple={isMultiple}
      >

        <NavDrawerBody>

          <div className={styles.title}>
            Loyalfier
          </div>
          <div className={styles.subTitle}>
            It's time to show your loyalty!
          </div>

          <NavDivider />

          <NavItem icon={<Dashboard />} value="1">
            生成
          </NavItem>

          <NavItem icon={<History />} value="2">
            历史
          </NavItem>


          <div className={styles.navItemBottom} >

            <NavItem icon={<Settings />} value="3">
              设置
            </NavItem>

            <NavItem icon={<About />} value="4">
              关于
            </NavItem>
          </div>

        </NavDrawerBody>
      </NavDrawer>
      <div className={styles.content}>
        <div className={styles.field}>
          <Label id={typeLableId}>Type</Label>
          <RadioGroup
            value={type}
            onChange={(_, data) => setType(data.value as DrawerType)}
            aria-labelledby={typeLableId}
          >
            <Radio value="overlay" label="Overlay (Default)" />
            <Radio value="inline" label="Inline" />
          </RadioGroup>
          <Label id={linkLabelId}>Links</Label>

          <Label id={multipleLabelId}>Categories</Label>
          <Switch
            checked={isMultiple}
            onChange={(_, data) => setIsMultiple(!!data.checked)}
            label={isMultiple ? "Multiple" : "Single"}
            aria-labelledby={multipleLabelId}
          />
        </div>
      </div>
    </div >
  );
};

export default App;
