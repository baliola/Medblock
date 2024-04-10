import {styled} from 'nativewind';
import React from 'react';
import {GestureResponderEvent, TextStyle, TouchableOpacity} from 'react-native';
import TextPrimary from '../text/TextPrimary';

const StyledTouchableOpacity = styled(TouchableOpacity);

interface TextButtonProps {
  label: string;
  classStyle?: string;
  isBold?: boolean;
  isDisabled?: boolean;
  onPress: (event: GestureResponderEvent) => void;
  style?: TextStyle;
}

const TextButton: React.FC<TextButtonProps> = ({
  label,
  classStyle,
  onPress,
  isBold = true,
  isDisabled,
  style,
}) => {
  return (
    <StyledTouchableOpacity onPress={onPress} disabled={isDisabled}>
      <TextPrimary
        text={label}
        classStyle={classStyle + ` ${isDisabled ? 'opacity-30' : ''}`}
        isBold={isBold}
        style={style}
      />
    </StyledTouchableOpacity>
  );
};

export default TextButton;
