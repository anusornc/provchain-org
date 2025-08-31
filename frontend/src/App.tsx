import { useState, useEffect, Suspense, lazy } from 'react';
import './App.css';
import Navigation from './components/layout/Navigation';
import { ThemeProvider } from './contexts/ThemeContext';
import LoadingSpinner from './components/ui/LoadingSpinner';
import type { Block, Transaction } from './types';

// Lazy load components for code splitting and performance optimization
const Dashboard = lazy(() => import('./components/dashboard/Dashboard'));
const BlockExplorer = lazy(() => import('./components/explorer/BlockExplorer'));
const BlockDetails = lazy(() => import('./components/explorer/BlockDetails'));

// Lazy load existing components for backward compatibility
const OntologyManager = lazy(() => import('./features/ontology/OntologyManager'));
const RDFTripleStore = lazy(() => import('./features/rdf/RDFTripleStore'));
const KnowledgeGraph = lazy(() => import('./features/knowledge-graph/KnowledgeGraph'));
const ProvenanceTracker = lazy(() => import('./features/provenance/ProvenanceTracker'));
const TraceabilityQueries = lazy(() => import('./features/queries/TraceabilityQueries'));

// Lazy load new Phase 3 traceability components
const TraceabilityExplorer = lazy(() => import('./components/traceability/TraceabilityExplorer'));
const SPARQLQueryBuilder = lazy(() => import('./components/traceability/SPARQLQueryBuilder'));
const TransactionExplorer = lazy(() => import('./components/explorer/TransactionExplorer'));

// Lazy load Phase 2 enhancement components
const AdvancedSearch = lazy(() => import('./components/search/AdvancedSearch'));
const Timeline = lazy(() => import('./components/timeline/Timeline'));
const ParticipantsManager = lazy(() => import('./components/participants/ParticipantsManager'));
const AnalyticsDashboard = lazy(() => import('./components/analytics/AnalyticsDashboard'));

export type TabType = 
  | 'dashboard'
  | 'explorer' 
  | 'blocks'
  | 'transactions'
  | 'search'
  | 'traceability'
  | 'items'
  | 'knowledge-graph'
  | 'timeline'
  | 'participants'
  | 'analytics'
  | 'semantic'
  | 'sparql'
  | 'query-builder'
  | 'ontology'
  | 'rdf'
  | 'provenance'
  | 'queries';

function App() {
  const [activeTab, setActiveTab] = useState<TabType>('dashboard');
  const [isLoading, setIsLoading] = useState(true);
  const [selectedBlock, setSelectedBlock] = useState<Block | null>(null);
  const [, setSelectedTransaction] = useState<Transaction | null>(null);

  useEffect(() => {
    const initializeApp = async () => {
      try {
        // Check if we have an auth token, if not, auto-login for development
        const token = localStorage.getItem('authToken');
        if (!token) {
          console.log('No auth token found, attempting auto-login for development...');
          try {
            const response = await fetch('http://localhost:8080/auth/login', {
              method: 'POST',
              headers: {
                'Content-Type': 'application/json',
              },
              body: JSON.stringify({
                username: 'admin',
                password: 'admin123'
              })
            });
            
            if (response.ok) {
              const authData = await response.json();
              localStorage.setItem('authToken', authData.token);
              console.log('Auto-login successful');
            } else {
              console.warn('Auto-login failed:', response.statusText);
            }
          } catch (authError) {
            console.warn('Auto-login error:', authError);
          }
        }
        
        // Simulate initialization delay
        await new Promise(resolve => setTimeout(resolve, 1000));
      } catch (error) {
        console.error('Error initializing app:', error);
      } finally {
        setIsLoading(false);
      }
    };

    initializeApp();
  }, []);

  if (isLoading) {
    return (
      <ThemeProvider>
        <div className="min-h-screen bg-gradient-to-br from-blue-50 to-indigo-100 dark:from-gray-900 dark:to-gray-800 flex items-center justify-center">
          <LoadingSpinner size="lg" message="Initializing ProvChain Explorer..." />
        </div>
      </ThemeProvider>
    );
  }

  const handleBlockSelect = (block: Block) => {
    setSelectedBlock(block);
  };

  const handleBlockBack = () => {
    setSelectedBlock(null);
  };

  const handleTransactionSelect = (transaction: Transaction) => {
    setSelectedTransaction(transaction);
  };

  const renderActiveTab = () => {
    switch (activeTab) {
      case 'dashboard':
        return <Dashboard />;
      
      // Block Explorer tabs
      case 'explorer':
      case 'blocks':
        if (selectedBlock) {
          return (
            <BlockDetails
              block={selectedBlock}
              onBack={handleBlockBack}
              onTransactionSelect={handleTransactionSelect}
            />
          );
        }
        return <BlockExplorer onBlockSelect={handleBlockSelect} />;
      
      case 'transactions':
        return <TransactionExplorer onTransactionSelect={handleTransactionSelect} />;
      
      case 'search':
        return <AdvancedSearch />;
      
      // Traceability tabs
      case 'traceability':
      case 'items':
        return <TraceabilityExplorer />;
      
      case 'knowledge-graph':
        return <KnowledgeGraph />;
      
      case 'timeline':
        return <Timeline />;
      
      // Other tabs
      case 'participants':
        return <ParticipantsManager />;
      
      case 'analytics':
        return <AnalyticsDashboard />;
      
      // Semantic tabs
      case 'semantic':
      case 'sparql':
        return <TraceabilityQueries />;
      
      case 'query-builder':
        return <SPARQLQueryBuilder />;
      
      case 'ontology':
        return <OntologyManager />;
      
      // Legacy tabs for backward compatibility
      case 'rdf':
        return <RDFTripleStore />;
      
      case 'provenance':
        return <ProvenanceTracker />;
      
      case 'queries':
        return <TraceabilityQueries />;
      
      default:
        return <Dashboard />;
    }
  };

  const handleTabChange = (tab: string) => {
    setActiveTab(tab as TabType);
    // Reset selected items when changing tabs
    setSelectedBlock(null);
    setSelectedTransaction(null);
  };

  return (
    <ThemeProvider>
      <div className="min-h-screen bg-gray-50 dark:bg-gray-900 flex">
        {/* Navigation Sidebar */}
        <Navigation activeTab={activeTab} onTabChange={handleTabChange} />
        
        {/* Main Content */}
        <main className="flex-1 lg:ml-0">
          <div className="lg:pl-64">
            <Suspense fallback={
              <div className="flex items-center justify-center min-h-screen">
                <LoadingSpinner size="lg" message="Loading component..." />
              </div>
            }>
              {renderActiveTab()}
            </Suspense>
          </div>
        </main>
      </div>
    </ThemeProvider>
  );
}

export default App;
