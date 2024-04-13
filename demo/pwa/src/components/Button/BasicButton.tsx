import { CSSProperties } from 'react';

interface BasicButtonProps {
  label?: string;
  classStyle?: string;
  child?: React.ReactElement<any, any>;
  style?: CSSProperties;
  labelStyle?: CSSProperties;
  onPress: () => void;
}

const BasicButton: React.FC<BasicButtonProps> = ({
  label,
  onPress,
  classStyle,
  child,
  style,
  labelStyle,
}) => {
  return (
    <button
      onClick={() => {
        onPress();
      }}
      className={'flex justify-center items-center ' + classStyle}
      style={style}
    >
      {label ? (
        <p className="text-center font-bold" style={labelStyle}>
          {' '}
          {label}{' '}
        </p>
      ) : (
        child
      )}
    </button>
  );
};

export default BasicButton;
