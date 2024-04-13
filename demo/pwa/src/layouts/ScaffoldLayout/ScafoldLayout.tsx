import React, { CSSProperties, FC, ReactElement } from 'react';

interface ScaffoldProps {
  topBar?: ReactElement<any, any>;
  children?: ReactElement<any, any>;
  bottomChild?: ReactElement<any, any>;
  loading?: boolean;
  style?: CSSProperties;
}

const Scaffold: FC<ScaffoldProps> = ({
  style,
  bottomChild,
  children,
  loading,
  topBar,
}) => {
  return (
    <div className="w-screen h-screen relative" style={style}>
      {topBar != null ? (
        <div className="fixed top-0 z-10 w-screen bg-white">{topBar}</div>
      ) : null}
      {children}
      {bottomChild != null ? (
        <div className="fixed bottom-0 w-full h-auto z-10">{bottomChild}</div>
      ) : null}
      {/* {loading ? <GeneralLoading loading={loading} /> : null} */}
    </div>
  );
};

export default Scaffold;
