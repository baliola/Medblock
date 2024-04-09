import TextPrimary from '@components/text/TextPrimary';
import {styled} from 'nativewind';
import React from 'react';
import {
  GestureResponderEvent,
  StyleProp,
  TextStyle,
  TouchableOpacity,
  ViewStyle,
} from 'react-native';

const StyledTouchableOpacity = styled(TouchableOpacity);

interface BasicButtonProps {
  label?: string;
  classStyle?: string;
  child?: React.ReactElement<any, any>;
  style?: StyleProp<ViewStyle>;
  labelStyle?: TextStyle;
  onPress: (event: GestureResponderEvent) => void;
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
    <StyledTouchableOpacity
      onPress={onPress}
      className={'flex justify-center items-center ' + classStyle}
      style={style}>
      {label ? (
        <TextPrimary
          classStyle="text-center"
          style={labelStyle}
          text={label}
          isBold={true}
        />
      ) : (
        child
      )}
    </StyledTouchableOpacity>
  );
};

export default BasicButton;
