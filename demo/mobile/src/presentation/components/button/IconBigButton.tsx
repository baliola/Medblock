import TextPrimary from '@components/text/TextPrimary';
import {styled} from 'nativewind';
import React from 'react';
import {
  GestureResponderEvent,
  Image,
  StyleSheet,
  TouchableOpacity,
  View,
} from 'react-native';

const StyledTouchableOpacity = styled(TouchableOpacity);
const StyledView = styled(View);
const StyledImage = styled(Image);

interface IconBigButtonProps {
  icon: string;
  label: string;
  onPress: ((event: GestureResponderEvent) => void) | undefined;
  classStyle?: string;
}

const IconBigButton: React.FC<IconBigButtonProps> = ({
  icon,
  label,
  onPress,
  classStyle,
}) => {
  return (
    <StyledTouchableOpacity
      onPress={onPress}
      className={classStyle + ' items-center'}>
      <StyledView className="h-20 w-20">
        <StyledImage
          source={parseInt(icon, 10)}
          className="h-full w-full"
          style={styles.icon}
        />
      </StyledView>
      <TextPrimary
        text={label}
        classStyle="text-gray-800 mt-2 text-xs"
        isBold={true}
      />
    </StyledTouchableOpacity>
  );
};

const styles = StyleSheet.create({
  icon: {objectFit: 'scale-down'},
});

export default IconBigButton;
