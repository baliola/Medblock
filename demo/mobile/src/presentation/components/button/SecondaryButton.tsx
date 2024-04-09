import {styled} from 'nativewind';
import React from 'react';
import {GestureResponderEvent, TouchableOpacity} from 'react-native';
import TextPrimary from '../text/TextPrimary';

const StyledTouchableOpacity = styled(TouchableOpacity);

interface SecondaryButtonProps {
  label?: string;
  classStyle?: string;
  child?: React.ReactElement<any, any>;
  onPress: (event: GestureResponderEvent) => void;
}

const SecondaryButton: React.FC<SecondaryButtonProps> = ({
  label,
  onPress,
  child,
  classStyle,
}) => {
  return (
    <StyledTouchableOpacity
      onPress={onPress}
      className={'rounded-2xl bg-secondary-normal w-full p-5 ' + classStyle}>
      {label ? (
        <TextPrimary
          classStyle="text-center text-white"
          text={label}
          isBold={true}
        />
      ) : (
        child
      )}
    </StyledTouchableOpacity>
  );
};

export default SecondaryButton;
