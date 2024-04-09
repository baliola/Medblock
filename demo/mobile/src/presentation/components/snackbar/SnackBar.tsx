import TextPrimary from '@components/text/TextPrimary';
import Colors from '@constants/colors';
import React, {
  createContext,
  useCallback,
  useContext,
  useEffect,
  useRef,
  useState,
} from 'react';
import {TouchableOpacity, Animated, StyleSheet, ColorValue} from 'react-native';

export enum SnackbarDuration {
  SHORT = 1000,
  MEDIUM = 3000,
  LONG = 5000,
  INFINITE = 1000000,
}

interface SnackbarProps {
  visible: boolean;
  backgroundColor?: ColorValue;
  message: string;
  duration: SnackbarDuration;
  actionText?: string;
  onActionPress?: () => void;
}

const Snackbar: React.FC<SnackbarProps> = ({
  message,
  backgroundColor = Colors.primary_normal,
  duration = SnackbarDuration.SHORT,
  actionText,
  onActionPress,
  visible,
}) => {
  const opacity = useRef(new Animated.Value(0)).current;

  useEffect(() => {
    if (visible) {
      Animated.timing(opacity, {
        toValue: 1,
        duration: duration,
        useNativeDriver: true,
      }).start();
    } else {
      Animated.timing(opacity, {
        toValue: 0,
        duration: duration,
        useNativeDriver: true,
      }).start();
    }
  }, [visible, opacity, duration]);

  if (!visible) {
    return null;
  }

  const styles = StyleSheet.create({
    snackbar: {
      height: 'auto',
      position: 'absolute',
      bottom: 20,
      left: 20,
      right: 20,
      backgroundColor: backgroundColor,
      padding: 16,
      borderRadius: 4,
      flexDirection: 'row',
      justifyContent: 'space-between',
      alignItems: 'center',
      opacity,
    },
  });

  return (
    <Animated.View style={styles.snackbar}>
      <TextPrimary text={message} classStyle="text-xs text-white" />
      {actionText && (
        <TouchableOpacity onPress={onActionPress}>
          <TextPrimary text={actionText} classStyle="text-xs text-white" />
        </TouchableOpacity>
      )}
    </Animated.View>
  );
};

export default Snackbar;

const SnackbarContext = createContext(null);

export const useSnackbar = () => useContext(SnackbarContext);

interface SnackbarProviderProps {
  children: React.ReactElement<any, any>;
}

export const SnackbarProvider: React.FC<SnackbarProviderProps> = ({
  children,
}) => {
  const [snackbarProps, setSnackbarProps] = useState<SnackbarProps>({
    visible: false,
    message: '',
    duration: SnackbarDuration.SHORT,
    backgroundColor: Colors.primary_normal,
    actionText: undefined,
    onActionPress: () => {},
  });

  const showSnackbar = useCallback(
    (
      message: string,
      backgroundColor: string,
      duration: SnackbarDuration,
      actionText: string,
      onActionPress: () => void,
    ) => {
      setSnackbarProps({
        visible: true,
        message,
        duration,
        backgroundColor,
        actionText,
        onActionPress: () => {
          onActionPress && onActionPress();
          setSnackbarProps(prevState => ({...prevState, visible: false}));
        },
      });

      setTimeout(() => {
        setSnackbarProps(prevState => ({...prevState, visible: false}));
      }, 3000);
    },
    [],
  );

  return (
    <SnackbarContext.Provider value={{showSnackbar}}>
      {children}
      <Snackbar {...snackbarProps} />
    </SnackbarContext.Provider>
  );
};
