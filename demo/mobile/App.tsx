import React, {useEffect} from 'react';
import {NavigationContainer} from '@react-navigation/native';
import {Provider} from 'react-redux';
import SplashScreen from 'react-native-splash-screen';
import {persistor, store} from './src/presentation/redux/store';
import {I18nextProvider} from 'react-i18next';
import i18n from './i18n';
import {QueryClient, QueryClientProvider} from 'react-query';
import {PersistGate} from 'redux-persist/integration/react';
import GeneralLoading from '@components/loading/GeneralLoading';
import AppNavigation from '@navigations/AppNavigation';
import {StatusBar} from 'react-native';

const queryClient = new QueryClient();

function App(): React.JSX.Element {
  useEffect(() => {
    SplashScreen.hide();
  }, []);

  return (
    <I18nextProvider i18n={i18n}>
      <NavigationContainer>
        <StatusBar backgroundColor="transparent" translucent={true} />
        <QueryClientProvider client={queryClient}>
          <Provider store={store}>
            <PersistGate loading={<GeneralLoading />} persistor={persistor}>
              <AppNavigation />
            </PersistGate>
          </Provider>
        </QueryClientProvider>
      </NavigationContainer>
    </I18nextProvider>
  );
}

export default App;
